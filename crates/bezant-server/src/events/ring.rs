//! Per-topic ring buffer for captured events.
//!
//! Each [`TopicRing`] is bounded in capacity. When a push would exceed
//! capacity, the oldest event is evicted (FIFO). Cursors are server-
//! assigned monotonically within `(topic, reset_epoch)` — they never
//! collide as long as `reset_epoch` is unique per process run, and they
//! never reset within a single run (so client cursor comparisons stay
//! cheap).
//!
//! The ring is read by cursor: `read_since(cursor, limit)` returns events
//! whose cursor is strictly greater than `cursor`, up to `limit`. If the
//! caller's cursor is older than the buffer's current head (the smallest
//! cursor still buffered), the read returns [`ReadResult::CursorExpired`]
//! so the client can resync to head and emit a synthetic gap event.

use std::collections::VecDeque;

use serde_json::Value;

use super::ObservedEvent;

/// FIFO ring of [`ObservedEvent`] keyed by cursor. Single owner; not
/// `Send + Sync` on its own — wrap in `Arc<RwLock<…>>` for cross-task
/// access.
#[derive(Debug)]
pub struct TopicRing {
    inner: VecDeque<ObservedEvent>,
    capacity: usize,
    next_cursor: u64,
    head_cursor: u64,
    reset_epoch: u64,
    topic: String,
}

/// Outcome of [`TopicRing::read_since`].
#[derive(Debug, Clone)]
pub enum ReadResult {
    /// Zero or more events newer than the requested cursor. `next_cursor`
    /// is the cursor the caller should send on their next poll.
    Ok {
        /// Events strictly newer than the requested cursor, in insertion
        /// order, capped at the requested limit.
        events: Vec<ObservedEvent>,
        /// Cursor to send on the next poll.
        next_cursor: u64,
    },
    /// The caller's cursor is older than the oldest event still buffered.
    /// They must reset to `head_cursor - 1` (or `0`) and emit a gap.
    CursorExpired {
        /// Smallest cursor currently buffered. Use `head_cursor - 1` (or
        /// `0` if `head_cursor == 0`) as the new `since=` parameter.
        head_cursor: u64,
        /// Current `reset_epoch` so the caller can detect resets.
        reset_epoch: u64,
    },
}

impl TopicRing {
    /// Construct a ring with the given capacity for a specific topic and
    /// reset epoch.
    #[must_use]
    pub fn new(topic: impl Into<String>, capacity: usize, reset_epoch: u64) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity.max(1)),
            capacity: capacity.max(1),
            next_cursor: 1,
            head_cursor: 1,
            reset_epoch,
            topic: topic.into(),
        }
    }

    /// Topic name this ring captures (e.g. `"orders"`).
    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }

    /// Current `reset_epoch`.
    #[must_use]
    pub fn reset_epoch(&self) -> u64 {
        self.reset_epoch
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the ring contains no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Smallest cursor currently buffered. Equal to `next_cursor` when empty.
    #[must_use]
    pub fn head_cursor(&self) -> u64 {
        self.head_cursor
    }

    /// Cursor that the next push will use. Useful in tests.
    #[must_use]
    pub fn next_cursor(&self) -> u64 {
        self.next_cursor
    }

    /// Push a new event with the given payload + RFC 3339 receive
    /// timestamp. Returns the assigned cursor. If the ring is at
    /// capacity, the oldest event is evicted (and `head_cursor` advances).
    pub fn push(&mut self, payload: Value, received_at: String) -> u64 {
        let cursor = self.next_cursor;
        self.next_cursor = self.next_cursor.saturating_add(1);

        let event = ObservedEvent {
            cursor,
            topic: self.topic.clone(),
            received_at,
            reset_epoch: self.reset_epoch,
            payload,
        };

        if self.inner.len() == self.capacity {
            // Evict oldest. head_cursor moves to whatever's now the
            // oldest still-buffered event's cursor.
            let evicted = self.inner.pop_front();
            if let Some(_) = evicted {
                // After eviction the new oldest is whatever's at front
                // (or, if empty, the cursor we're about to push).
                self.head_cursor = self
                    .inner
                    .front()
                    .map(|e| e.cursor)
                    .unwrap_or(cursor);
            }
        }

        self.inner.push_back(event);
        // If this was the very first event in a fresh ring, anchor head.
        if self.inner.len() == 1 {
            self.head_cursor = cursor;
        }

        cursor
    }

    /// Read events whose cursor is strictly greater than `since`, capped
    /// at `limit`. `since == 0` means "everything currently buffered".
    ///
    /// Returns [`ReadResult::CursorExpired`] when `since < head_cursor - 1`
    /// AND the ring has evicted at least one event the caller hasn't seen
    /// — i.e. there's a real gap, not just "you've read everything".
    #[must_use]
    pub fn read_since(&self, since: u64, limit: usize) -> ReadResult {
        // Caller is fully caught up — no gap, no expiry, just empty.
        if since >= self.next_cursor.saturating_sub(1) {
            return ReadResult::Ok {
                events: Vec::new(),
                next_cursor: self.next_cursor.saturating_sub(1),
            };
        }

        // Caller's cursor is older than our oldest buffered event AND we
        // have evicted at least one event past their cursor.
        if since + 1 < self.head_cursor {
            return ReadResult::CursorExpired {
                head_cursor: self.head_cursor,
                reset_epoch: self.reset_epoch,
            };
        }

        let events: Vec<ObservedEvent> = self
            .inner
            .iter()
            .filter(|e| e.cursor > since)
            .take(limit.max(1))
            .cloned()
            .collect();

        let next_cursor = events
            .last()
            .map(|e| e.cursor)
            .unwrap_or_else(|| self.next_cursor.saturating_sub(1));

        ReadResult::Ok { events, next_cursor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ts() -> String {
        "2026-05-06T00:00:00Z".to_string()
    }

    #[test]
    fn push_assigns_monotonic_cursors() {
        let mut ring = TopicRing::new("orders", 10, 0);
        let c1 = ring.push(json!({"n": 1}), ts());
        let c2 = ring.push(json!({"n": 2}), ts());
        let c3 = ring.push(json!({"n": 3}), ts());
        assert_eq!(c1, 1);
        assert_eq!(c2, 2);
        assert_eq!(c3, 3);
    }

    #[test]
    fn read_since_zero_returns_all_buffered() {
        let mut ring = TopicRing::new("orders", 10, 0);
        ring.push(json!({"n": 1}), ts());
        ring.push(json!({"n": 2}), ts());
        match ring.read_since(0, 100) {
            ReadResult::Ok { events, next_cursor } => {
                assert_eq!(events.len(), 2);
                assert_eq!(next_cursor, 2);
                assert_eq!(events[0].payload, json!({"n": 1}));
                assert_eq!(events[1].payload, json!({"n": 2}));
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn read_since_returns_only_new_events() {
        let mut ring = TopicRing::new("orders", 10, 0);
        ring.push(json!({"n": 1}), ts());
        ring.push(json!({"n": 2}), ts());
        ring.push(json!({"n": 3}), ts());
        match ring.read_since(1, 100) {
            ReadResult::Ok { events, next_cursor } => {
                assert_eq!(events.len(), 2);
                assert_eq!(events[0].cursor, 2);
                assert_eq!(events[1].cursor, 3);
                assert_eq!(next_cursor, 3);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn read_since_caught_up_returns_empty() {
        let mut ring = TopicRing::new("orders", 10, 0);
        ring.push(json!({"n": 1}), ts());
        ring.push(json!({"n": 2}), ts());
        match ring.read_since(2, 100) {
            ReadResult::Ok { events, next_cursor } => {
                assert!(events.is_empty());
                assert_eq!(next_cursor, 2);
            }
            other => panic!("expected empty Ok, got {other:?}"),
        }
    }

    #[test]
    fn read_since_respects_limit() {
        let mut ring = TopicRing::new("orders", 100, 0);
        for i in 0..50 {
            ring.push(json!({ "n": i }), ts());
        }
        match ring.read_since(0, 10) {
            ReadResult::Ok { events, next_cursor } => {
                assert_eq!(events.len(), 10);
                assert_eq!(next_cursor, 10);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn capacity_evicts_oldest_first() {
        let mut ring = TopicRing::new("orders", 3, 0);
        for i in 1..=5 {
            ring.push(json!({"n": i}), ts());
        }
        // After 5 pushes into cap-3 ring, cursors 1+2 evicted; 3,4,5 remain.
        assert_eq!(ring.len(), 3);
        assert_eq!(ring.head_cursor(), 3);
        match ring.read_since(0, 100) {
            ReadResult::CursorExpired { head_cursor, .. } => {
                assert_eq!(head_cursor, 3);
            }
            other => panic!("expected CursorExpired, got {other:?}"),
        }
    }

    #[test]
    fn read_after_overflow_at_head_succeeds() {
        let mut ring = TopicRing::new("orders", 3, 0);
        for i in 1..=5 {
            ring.push(json!({"n": i}), ts());
        }
        // Asking from head-1 (cursor 2) is OK — we still have cursor 3.
        match ring.read_since(2, 100) {
            ReadResult::Ok { events, next_cursor } => {
                assert_eq!(events.len(), 3);
                assert_eq!(events[0].cursor, 3);
                assert_eq!(next_cursor, 5);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn read_at_or_past_overflow_returns_expired() {
        let mut ring = TopicRing::new("orders", 3, 0);
        for i in 1..=5 {
            ring.push(json!({"n": i}), ts());
        }
        // Cursor 1 was evicted; asking from 0 sees a gap.
        assert!(matches!(
            ring.read_since(0, 100),
            ReadResult::CursorExpired { .. }
        ));
        assert!(matches!(
            ring.read_since(1, 100),
            ReadResult::CursorExpired { .. }
        ));
    }

    #[test]
    fn reset_epoch_propagates_to_read_result() {
        let mut ring = TopicRing::new("orders", 2, 42);
        for i in 1..=4 {
            ring.push(json!({"n": i}), ts());
        }
        match ring.read_since(0, 100) {
            ReadResult::CursorExpired { reset_epoch, .. } => {
                assert_eq!(reset_epoch, 42);
            }
            other => panic!("expected CursorExpired, got {other:?}"),
        }
    }

    #[test]
    fn reset_epoch_propagates_to_pushed_events() {
        let mut ring = TopicRing::new("orders", 4, 7);
        ring.push(json!({"n": 1}), ts());
        match ring.read_since(0, 10) {
            ReadResult::Ok { events, .. } => {
                assert_eq!(events[0].reset_epoch, 7);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn empty_ring_read_since_zero_is_ok_empty() {
        let ring = TopicRing::new("orders", 10, 0);
        match ring.read_since(0, 100) {
            ReadResult::Ok { events, next_cursor } => {
                assert!(events.is_empty());
                assert_eq!(next_cursor, 0);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn topic_name_appears_on_pushed_events() {
        let mut ring = TopicRing::new("marketdata:265598", 4, 0);
        ring.push(json!({"31": "150.25"}), ts());
        match ring.read_since(0, 10) {
            ReadResult::Ok { events, .. } => {
                assert_eq!(events[0].topic, "marketdata:265598");
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }
}
