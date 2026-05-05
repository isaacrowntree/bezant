//! Optional sqlite persistence for captured events.
//!
//! The ring buffer in [`super::ring::TopicRing`] is the primary read
//! path — fast, in-memory, bounded. Sqlite is the **historical** store:
//! every event we push to a ring is also appended to a single
//! `events` table so the `GET /events/{topic}/history?since_ts=…`
//! endpoint can reach back beyond ring capacity (and beyond container
//! lifetime).
//!
//! Schema:
//!
//! ```sql
//! CREATE TABLE events (
//!   id           INTEGER PRIMARY KEY AUTOINCREMENT,
//!   cursor       INTEGER NOT NULL,
//!   topic        TEXT    NOT NULL,
//!   received_at  TEXT    NOT NULL,    -- ISO 8601
//!   reset_epoch  INTEGER NOT NULL,
//!   payload      TEXT    NOT NULL     -- JSON
//! );
//! CREATE INDEX events_topic_received_at ON events(topic, received_at);
//! ```
//!
//! Retention is configurable per topic:
//! - `orders`     — 90 days (default)
//! - `pnl`        — 90 days
//! - `marketdata:*` — 14 days (high volume; bigger window doesn't pay)
//! - `gap`        — 365 days (low volume, useful forever)
//!
//! [`EventLog::prune_older_than`] is intended to be called from a
//! once-an-hour task; nothing in the read path waits on it.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};

use super::ObservedEvent;

/// Sqlite-backed historical event store. Wraps a [`Connection`] in a
/// [`Mutex`] so it can be shared across the connector + axum tasks
/// without unsafe juggling. Reads are fast; writes serialise.
pub struct EventLog {
    conn: Mutex<Connection>,
    /// Path the connection was opened against (for diagnostics).
    path: PathBuf,
}

impl std::fmt::Debug for EventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventLog").field("path", &self.path).finish()
    }
}

/// Per-topic retention policy. Topics not in the map default to 30 days.
#[derive(Clone, Debug)]
pub struct RetentionPolicy {
    /// Default retention for topics not explicitly listed.
    pub default_days: i64,
    /// Days for `orders`.
    pub orders_days: i64,
    /// Days for `pnl`.
    pub pnl_days: i64,
    /// Days for `marketdata:*` topics.
    pub marketdata_days: i64,
    /// Days for `gap`.
    pub gap_days: i64,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            default_days: 30,
            orders_days: 90,
            pnl_days: 90,
            marketdata_days: 14,
            gap_days: 365,
        }
    }
}

impl RetentionPolicy {
    /// How many days to keep events for `topic`.
    #[must_use]
    pub fn days_for(&self, topic: &str) -> i64 {
        if topic == "orders" {
            self.orders_days
        } else if topic == "pnl" {
            self.pnl_days
        } else if topic == "gap" {
            self.gap_days
        } else if topic.starts_with("marketdata:") {
            self.marketdata_days
        } else {
            self.default_days
        }
    }
}

impl EventLog {
    /// Open or create a sqlite database at `path`. Runs the schema
    /// migration on first call (idempotent).
    pub fn open(path: impl AsRef<Path>) -> rusqlite::Result<Self> {
        let conn = Connection::open(path.as_ref())?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS events (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                cursor       INTEGER NOT NULL,
                topic        TEXT    NOT NULL,
                received_at  TEXT    NOT NULL,
                reset_epoch  INTEGER NOT NULL,
                payload      TEXT    NOT NULL
            );
            CREATE INDEX IF NOT EXISTS events_topic_received_at
                ON events(topic, received_at);
            ",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Open an in-memory database — used by tests.
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            r"
            CREATE TABLE events (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                cursor       INTEGER NOT NULL,
                topic        TEXT    NOT NULL,
                received_at  TEXT    NOT NULL,
                reset_epoch  INTEGER NOT NULL,
                payload      TEXT    NOT NULL
            );
            CREATE INDEX events_topic_received_at
                ON events(topic, received_at);
            ",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
            path: PathBuf::from(":memory:"),
        })
    }

    /// Path the log was opened against.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append an event. Returns the row id.
    pub fn append(&self, event: &ObservedEvent) -> rusqlite::Result<i64> {
        let payload_str = serde_json::to_string(&event.payload)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO events (cursor, topic, received_at, reset_epoch, payload) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                event.cursor as i64,
                event.topic,
                event.received_at,
                event.reset_epoch as i64,
                payload_str,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Read events for a topic since `since_ts` (ISO 8601). Newest-last.
    pub fn query_since(
        &self,
        topic: &str,
        since_ts: &str,
        limit: usize,
    ) -> rusqlite::Result<Vec<ObservedEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT cursor, topic, received_at, reset_epoch, payload \
             FROM events WHERE topic = ?1 AND received_at > ?2 \
             ORDER BY received_at ASC LIMIT ?3",
        )?;
        let rows = stmt.query_map(
            params![topic, since_ts, limit as i64],
            |row| {
                let cursor: i64 = row.get(0)?;
                let topic: String = row.get(1)?;
                let received_at: String = row.get(2)?;
                let reset_epoch: i64 = row.get(3)?;
                let payload_str: String = row.get(4)?;
                let payload: serde_json::Value = serde_json::from_str(&payload_str)
                    .unwrap_or(serde_json::Value::Null);
                Ok(ObservedEvent {
                    cursor: cursor as u64,
                    topic,
                    received_at,
                    reset_epoch: reset_epoch as u64,
                    payload,
                })
            },
        )?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// Drop events older than the topic's retention cutoff. Returns
    /// total rows deleted across all topics.
    pub fn prune(&self, policy: &RetentionPolicy) -> rusqlite::Result<usize> {
        let conn = self.conn.lock().unwrap();
        let now: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();
        let mut total = 0usize;
        let topics: Vec<String> = {
            let mut stmt = conn.prepare("SELECT DISTINCT topic FROM events")?;
            let mapped = stmt.query_map([], |r| r.get::<_, String>(0))?;
            let mut v = Vec::new();
            for m in mapped {
                v.push(m?);
            }
            v
        };
        for topic in topics {
            let days = policy.days_for(&topic);
            let cutoff = now - chrono::Duration::days(days);
            let cutoff_str = cutoff.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            let n = conn.execute(
                "DELETE FROM events WHERE topic = ?1 AND received_at < ?2",
                params![topic, cutoff_str],
            )?;
            total += n;
        }
        Ok(total)
    }

    /// Total row count. Used by tests + diagnostics.
    pub fn count(&self) -> rusqlite::Result<usize> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
            .optional()?
            .unwrap_or(0);
        Ok(n as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn evt(cursor: u64, topic: &str, ts: &str) -> ObservedEvent {
        ObservedEvent {
            cursor,
            topic: topic.to_string(),
            received_at: ts.to_string(),
            reset_epoch: 1,
            payload: json!({"id": cursor}),
        }
    }

    #[test]
    fn append_and_query_round_trip() {
        let log = EventLog::open_in_memory().unwrap();
        log.append(&evt(1, "orders", "2026-05-06T13:30:00Z")).unwrap();
        log.append(&evt(2, "orders", "2026-05-06T13:31:00Z")).unwrap();
        log.append(&evt(3, "pnl", "2026-05-06T13:32:00Z")).unwrap();

        let orders = log.query_since("orders", "1970-01-01T00:00:00Z", 100).unwrap();
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].cursor, 1);
        assert_eq!(orders[1].cursor, 2);

        let pnl = log.query_since("pnl", "1970-01-01T00:00:00Z", 100).unwrap();
        assert_eq!(pnl.len(), 1);
        assert_eq!(pnl[0].topic, "pnl");
    }

    #[test]
    fn query_filters_by_since_ts() {
        let log = EventLog::open_in_memory().unwrap();
        log.append(&evt(1, "orders", "2026-05-06T13:30:00Z")).unwrap();
        log.append(&evt(2, "orders", "2026-05-06T13:31:00Z")).unwrap();
        log.append(&evt(3, "orders", "2026-05-06T13:32:00Z")).unwrap();

        let recent = log.query_since("orders", "2026-05-06T13:30:30Z", 100).unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].cursor, 2);
    }

    #[test]
    fn query_respects_limit() {
        let log = EventLog::open_in_memory().unwrap();
        for i in 1..=10 {
            let ts = format!("2026-05-06T13:{:02}:00Z", 30 + i);
            log.append(&evt(i, "orders", &ts)).unwrap();
        }
        let result = log.query_since("orders", "1970-01-01T00:00:00Z", 3).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].cursor, 1);
    }

    #[test]
    fn prune_drops_old_events_per_topic_policy() {
        let log = EventLog::open_in_memory().unwrap();
        // Old events (>90 days for orders / >14 days for marketdata)
        let ancient = (chrono::Utc::now() - chrono::Duration::days(120))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mid = (chrono::Utc::now() - chrono::Duration::days(20))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let recent = (chrono::Utc::now() - chrono::Duration::days(1))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        log.append(&evt(1, "orders", &ancient)).unwrap();
        log.append(&evt(2, "orders", &mid)).unwrap();
        log.append(&evt(3, "orders", &recent)).unwrap();
        log.append(&evt(10, "marketdata:1", &mid)).unwrap();
        log.append(&evt(11, "marketdata:1", &recent)).unwrap();

        let policy = RetentionPolicy::default();
        let dropped = log.prune(&policy).unwrap();
        // orders: 120-day-old dropped; marketdata: 20-day-old dropped
        assert_eq!(dropped, 2);

        assert_eq!(log.count().unwrap(), 3);
    }

    #[test]
    fn retention_policy_picks_per_topic_days() {
        let p = RetentionPolicy::default();
        assert_eq!(p.days_for("orders"), 90);
        assert_eq!(p.days_for("pnl"), 90);
        assert_eq!(p.days_for("gap"), 365);
        assert_eq!(p.days_for("marketdata:265598"), 14);
        assert_eq!(p.days_for("unknown"), 30);
    }
}
