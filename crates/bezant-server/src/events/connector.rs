//! Long-lived WebSocket connector + REST-side handle.
//!
//! The connector is an actor that owns a single [`bezant::WsClient`] and
//! drives it forever:
//!
//! 1. Connect (with exponential-backoff retry).
//! 2. Subscribe to `orders` + `pnl` (always, on every connect).
//! 3. Loop: dispatch frames into per-topic rings, accept
//!    subscribe/unsubscribe commands for market data, watch a heartbeat
//!    timeout to detect a stalled socket.
//! 4. On any disconnect: sleep with backoff, GOTO 1. The next SUCCESSFUL
//!    connect bumps `reset_epoch` on every ring and records one synthetic
//!    event on the `gap` topic for the whole outage — failed attempts in
//!    between move nothing. A connection that lived 5 minutes resets the
//!    backoff; a failure repeating the last one's kind logs at DEBUG.
//!
//! Two things CPAPI does that the loop above did not survive, and now does:
//!
//! - It REFUSES a subscribe. `sor+{}` is answered with
//!   `{"error":"unable to subscribe","code":500,"topic":"sor"}` on most
//!   reconnects (17 of 20 attempts over Aug–Sep 2026 on one Gateway). That
//!   frame used to be filed as an order event and the subscribe was never
//!   retried, so the socket stayed up for days with `pnl` flowing and
//!   `orders` dead. A refusal is now a control frame: it marks the topic
//!   [`SubscriptionState::Refused`] and schedules a resubscribe with
//!   backoff, primed by `GET /iserver/accounts` (the call CPAPI documents
//!   as the precondition for order queries).
//! - It re-logs in underneath the socket. The Gateway's nightly re-auth,
//!   or an assisted re-login, mints a new session; a socket bound to the
//!   old one keeps heartbeating — so the heartbeat timeout never fires —
//!   while every subscription on it is dead. The connector now compares
//!   the socket's session id against `/tickle` periodically and reconnects
//!   when it changes.
//!
//! [`EventsHandle`] is what axum handlers get. Cloneable, cheap, exposes
//! reads against the rings and a command channel into the actor task.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::FutureExt;

use bezant::{MarketDataFields, WsClient, WsMessage};
use serde_json::json;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time::{sleep, sleep_until, timeout, Instant as TokioInstant};
use tracing::{debug, info, warn};

use super::persistence::{EventLog, RetentionPolicy};
use super::ring::{ReadResult, TopicRing};
use super::{EventsStatus, GapReason, ObservedEvent, SubscriptionState};

/// Configurable knobs for the connector.
#[derive(Clone, Debug)]
pub struct ConnectorCfg {
    /// Capacity of the `orders` ring.
    pub orders_capacity: usize,
    /// Capacity of the `pnl` ring.
    pub pnl_capacity: usize,
    /// Per-conid market-data ring capacity.
    pub marketdata_capacity: usize,
    /// Min reconnect backoff.
    pub backoff_min: Duration,
    /// Max reconnect backoff.
    pub backoff_max: Duration,
    /// If no frame arrives in this long, the connector tears the socket
    /// down and reconnects (assumes the socket is dead, not idle).
    pub heartbeat_timeout: Duration,
    /// Idle market-data subs auto-unsubscribe after this long with no
    /// `ensure_market_data` call (P2 — currently unused, plumbed for
    /// forward compatibility).
    pub marketdata_idle_unsubscribe: Duration,
    /// Optional sqlite-backed historical event log. When set, every
    /// pushed event is also appended here so `/events/{topic}/history`
    /// can serve reads beyond ring capacity.
    pub event_log: Option<Arc<EventLog>>,
    /// Retention policy used by the periodic prune task.
    pub retention: RetentionPolicy,
    /// How often to run the prune task. Defaults to once per hour.
    pub prune_every: Duration,
    /// First retry delay after CPAPI refuses a standing subscription.
    pub resubscribe_min: Duration,
    /// Retry delay ceiling. A `sor+{}` every five minutes is a harmless ask
    /// of a Gateway that keeps saying no; the fund's confirmer no longer
    /// depends on it, but a healthy stream is still the fast path.
    pub resubscribe_max: Duration,
    /// How often to ask `/tickle` whether the Gateway session the socket
    /// was opened under is still the current one.
    pub session_check_every: Duration,
    /// A connection that stayed up at least this long resets the reconnect
    /// backoff to `backoff_min`. Shorter-lived ones keep doubling it, so a
    /// socket that connects and dies at once cannot spin.
    pub backoff_reset_after: Duration,
    /// Resubscribe rounds a standing topic may go unanswered — no frame, no
    /// refusal — before it is reported `quiet` and no longer re-asked.
    /// `0` re-asks forever (the behaviour before this knob existed).
    pub quiet_after_rounds: u32,
}

impl Default for ConnectorCfg {
    fn default() -> Self {
        Self {
            orders_capacity: 1_000,
            pnl_capacity: 5_000,
            marketdata_capacity: 2_000,
            backoff_min: Duration::from_secs(1),
            backoff_max: Duration::from_secs(60),
            heartbeat_timeout: Duration::from_secs(90),
            marketdata_idle_unsubscribe: Duration::from_secs(300),
            event_log: None,
            retention: RetentionPolicy::default(),
            prune_every: Duration::from_secs(3_600),
            resubscribe_min: Duration::from_secs(5),
            resubscribe_max: Duration::from_secs(300),
            session_check_every: Duration::from_secs(60),
            backoff_reset_after: Duration::from_secs(300),
            quiet_after_rounds: 3,
        }
    }
}

/// Commands the connector accepts from outside.
#[derive(Debug)]
enum ConnectorCmd {
    EnsureMarketData {
        conid: i64,
        reply: oneshot::Sender<Result<(), String>>,
    },
    /// Drop the socket (if any) and connect again now, skipping any backoff.
    /// Replies with whether a socket was up when the request arrived.
    Reconnect { reply: oneshot::Sender<bool> },
}

/// How long `POST /events/_reconnect` waits for the connector to take the
/// request. The connector reads commands between frames and while backing
/// off, so this only runs out if it is stuck mid-connect.
const RECONNECT_ACK_TIMEOUT: Duration = Duration::from_secs(10);

/// Cloneable handle the axum handlers use. Reads come straight off the
/// shared ring map; writes go through the command channel.
#[derive(Clone)]
pub struct EventsHandle {
    rings: Arc<RwLock<HashMap<String, TopicRing>>>,
    status: Arc<RwLock<StatusState>>,
    cmd_tx: mpsc::Sender<ConnectorCmd>,
    started_at: Instant,
    event_log: Option<Arc<EventLog>>,
    persist_failures: Arc<AtomicU64>,
}

impl EventsHandle {
    /// Borrow the optional sqlite event log so `/events/{topic}/history`
    /// route handlers can query historical events.
    #[must_use]
    pub fn event_log(&self) -> Option<&Arc<EventLog>> {
        self.event_log.as_ref()
    }
}

#[derive(Debug, Default)]
struct StatusState {
    connected: bool,
    last_message_at: Option<String>,
    reconnect_count: u64,
    reset_epoch: u64,
    /// First cursor of every ring this process creates. See [`boot_seeds`].
    cursor_base: u64,
    topics_subscribed: BTreeSet<String>,
    subscriptions: BTreeMap<String, SubscriptionState>,
    subscribe_refusals: u64,
    session_rollovers: u64,
    connector_restarts: u64,
}

impl EventsHandle {
    /// Read events from a topic. Returns [`None`] if the topic isn't
    /// known (no events ever arrived for it). For `marketdata:<conid>`,
    /// callers should call [`Self::ensure_market_data`] first to register
    /// interest.
    pub async fn read_topic(&self, topic: &str, since: u64, limit: usize) -> Option<ReadResult> {
        let rings = self.rings.read().await;
        rings.get(topic).map(|r| r.read_since(since, limit))
    }

    /// What a read of a topic that has no ring yet should answer: exactly
    /// what an empty ring created right now would. The caller gets this
    /// process's cursor space and live epoch instead of its own cursor
    /// echoed back — an echo keeps a cursor from a previous process alive,
    /// and the first real event then lands below it.
    pub async fn read_empty_topic(&self, topic: &str, since: u64) -> ReadResult {
        let s = self.status.read().await;
        TopicRing::with_base(topic, 1, s.reset_epoch, s.cursor_base).read_since(since, 1)
    }

    /// Snapshot of status. `uptime_seconds` is computed at call time
    /// from the connector's start instant.
    pub async fn status(&self) -> EventsStatus {
        let s = self.status.read().await;
        let buffer_sizes: BTreeMap<String, usize> = self
            .rings
            .read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.len()))
            .collect();
        EventsStatus {
            connected: s.connected,
            last_message_at: s.last_message_at.clone(),
            reconnect_count: s.reconnect_count,
            uptime_seconds: self.started_at.elapsed().as_secs(),
            reset_epoch: s.reset_epoch,
            topics_subscribed: s.topics_subscribed.iter().cloned().collect(),
            buffer_sizes,
            subscriptions: s.subscriptions.clone(),
            subscribe_refusals: s.subscribe_refusals,
            session_rollovers: s.session_rollovers,
            connector_restarts: s.connector_restarts,
            persist_failures: self.persist_failures.load(Ordering::Relaxed),
        }
    }

    /// Ask the connector to drop its socket and reconnect now, bypassing
    /// the backoff — the soft alternative to restarting the container when
    /// the stream has gone quiet. Returns whether a socket was up when the
    /// request was taken.
    ///
    /// # Errors
    /// The connector task is gone, or did not take the request within
    /// [`RECONNECT_ACK_TIMEOUT`].
    pub async fn request_reconnect(&self) -> Result<bool, String> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(ConnectorCmd::Reconnect { reply: tx })
            .await
            .map_err(|_| "connector task is not running".to_string())?;
        match timeout(RECONNECT_ACK_TIMEOUT, rx).await {
            Ok(Ok(was_connected)) => Ok(was_connected),
            Ok(Err(_)) => Err("connector task dropped reply channel".to_string()),
            Err(_) => Err("connector did not take the request in time".to_string()),
        }
    }

    /// Ensure the upstream WS is subscribed to market data for `conid`.
    /// Idempotent; multiple callers can request the same conid and only
    /// one upstream subscribe is sent. Returns `Err` if the connector
    /// task is dead or if the subscribe send failed.
    pub async fn ensure_market_data(&self, conid: i64) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(ConnectorCmd::EnsureMarketData { conid, reply: tx })
            .await
            .map_err(|_| "connector task is not running".to_string())?;
        rx.await
            .map_err(|_| "connector task dropped reply channel".to_string())?
    }
}

impl EventsHandle {
    /// Build a handle that's not backed by a live connector — used by
    /// integration tests to populate rings synthetically and verify the
    /// HTTP surface without standing up a CPAPI WS mock.
    ///
    /// Returns the handle plus a `TestSink` that lets the test push
    /// pre-decoded events into named topics.
    #[doc(hidden)]
    #[must_use]
    pub fn for_test() -> (Self, TestSink) {
        Self::for_test_with_log(None)
    }

    /// Same as [`Self::for_test`] but with an attached event log so
    /// tests can exercise the `/history` route.
    #[doc(hidden)]
    #[must_use]
    pub fn for_test_with_log(event_log: Option<Arc<EventLog>>) -> (Self, TestSink) {
        Self::for_test_seeded(event_log, 1, 1)
    }

    /// Same as [`Self::for_test_with_log`], seeded the way a process boot
    /// seeds a real connector — see [`boot_seeds`]. Two harnesses with
    /// different seeds stand in for a process before and after a restart.
    #[doc(hidden)]
    #[must_use]
    pub fn for_test_seeded(
        event_log: Option<Arc<EventLog>>,
        reset_epoch: u64,
        cursor_base: u64,
    ) -> (Self, TestSink) {
        let rings: Arc<RwLock<HashMap<String, TopicRing>>> = Arc::new(RwLock::new(HashMap::new()));
        let status = Arc::new(RwLock::new(StatusState {
            connected: true,
            reset_epoch,
            cursor_base,
            ..Default::default()
        }));
        let (cmd_tx, cmd_rx) = mpsc::channel::<ConnectorCmd>(8);
        let started_at = Instant::now();
        let handle = Self {
            rings: rings.clone(),
            status: status.clone(),
            cmd_tx,
            started_at,
            event_log: event_log.clone(),
            persist_failures: Arc::default(),
        };
        let sink = TestSink {
            rings,
            status,
            event_log,
            cmd_rx: Arc::new(std::sync::Mutex::new(Some(cmd_rx))),
            reconnects: Arc::default(),
        };
        (handle, sink)
    }
}

/// Sink-side counterpart of [`EventsHandle::for_test`]. Lets integration
/// tests push events into a topic ring, mark the connection
/// disconnected, and bump the reset epoch — without involving a real
/// WebSocket. Public solely so `tests/events_routes.rs` can drive the
/// routes; not part of the production API surface.
#[doc(hidden)]
#[derive(Clone)]
pub struct TestSink {
    rings: Arc<RwLock<HashMap<String, TopicRing>>>,
    status: Arc<RwLock<StatusState>>,
    event_log: Option<Arc<EventLog>>,
    /// The command channel's far end, until [`Self::serve_commands`] takes it.
    cmd_rx: Arc<std::sync::Mutex<Option<mpsc::Receiver<ConnectorCmd>>>>,
    reconnects: Arc<AtomicU64>,
}

#[doc(hidden)]
impl TestSink {
    /// Push `payload` into the named topic's ring. Creates the ring on
    /// first call. Reports the assigned cursor. If a log is attached,
    /// also appends to sqlite.
    pub async fn push(&self, topic: &str, payload: serde_json::Value) -> u64 {
        let cap = match topic {
            "orders" => 1_000,
            "pnl" => 5_000,
            t if t.starts_with("marketdata:") => 2_000,
            _ => 256,
        };
        let (epoch, base) = {
            let s = self.status.read().await;
            (s.reset_epoch, s.cursor_base)
        };
        let received_at = now_iso();
        let mut rings = self.rings.write().await;
        let ring = rings
            .entry(topic.to_string())
            .or_insert_with(|| TopicRing::with_base(topic, cap, epoch, base));
        let cursor = ring.push(payload.clone(), received_at.clone());
        drop(rings);
        if let Some(log) = &self.event_log {
            let _ = log.append(&ObservedEvent {
                cursor,
                topic: topic.to_string(),
                received_at: received_at.clone(),
                reset_epoch: epoch,
                payload,
            });
        }
        self.status
            .write()
            .await
            .topics_subscribed
            .insert(topic.to_string());
        cursor
    }

    /// Force a specific reset_epoch, the way a reconnect moves it: every
    /// existing ring follows.
    pub async fn set_reset_epoch(&self, epoch: u64) {
        self.status.write().await.reset_epoch = epoch;
        for ring in self.rings.write().await.values_mut() {
            ring.set_reset_epoch(epoch);
        }
    }

    /// Force the `connected` flag — useful for `_status` shape testing.
    pub async fn set_connected(&self, connected: bool) {
        self.status.write().await.connected = connected;
    }

    /// Stand in for the connector's command loop: answer every command the
    /// handle sends, as a connected connector would, counting reconnect
    /// requests. Without this, commands queue and are never answered.
    pub fn serve_commands(&self) {
        let rx = self
            .cmd_rx
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        let Some(mut rx) = rx else { return };
        let status = self.status.clone();
        let reconnects = self.reconnects.clone();
        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    ConnectorCmd::EnsureMarketData { reply, .. } => {
                        let _ = reply.send(Ok(()));
                    }
                    ConnectorCmd::Reconnect { reply } => {
                        reconnects.fetch_add(1, Ordering::SeqCst);
                        let _ = reply.send(status.read().await.connected);
                    }
                }
            }
        });
    }

    /// Reconnect requests the connector has taken (see [`Self::serve_commands`]).
    #[must_use]
    pub fn reconnect_requests(&self) -> u64 {
        self.reconnects.load(Ordering::SeqCst)
    }
}

/// Spawn the connector task and return a [`EventsHandle`] for the
/// axum side. The task owns the [`bezant::Client`] reference (cheap, it's
/// already `Arc`-wrapped internally) and runs until the binary exits.
///
/// If `cfg.event_log` is `Some`, also spawns a periodic prune task that
/// trims the sqlite store to retention policy every `cfg.prune_every`.
pub fn spawn_connector(client: bezant::Client, cfg: ConnectorCfg) -> EventsHandle {
    let rings: Arc<RwLock<HashMap<String, TopicRing>>> = Arc::new(RwLock::new(HashMap::new()));
    let high_water = cfg.event_log.as_ref().and_then(|log| match log.high_water() {
        Ok(hw) => hw,
        Err(e) => {
            warn!(error = %e, "events sqlite: could not read the previous run's cursors; seeding from the clock alone");
            None
        }
    });
    let (reset_epoch, cursor_base) = boot_seeds(unix_millis(), high_water);
    info!(
        reset_epoch,
        cursor_base, "events connector: seeded epoch and cursors from boot time"
    );
    let status = Arc::new(RwLock::new(StatusState {
        reset_epoch,
        cursor_base,
        ..StatusState::default()
    }));
    let (cmd_tx, cmd_rx) = mpsc::channel::<ConnectorCmd>(64);
    let started_at = Instant::now();
    let event_log = cfg.event_log.clone();

    if let Some(log) = event_log.clone() {
        let prune_every = cfg.prune_every;
        let policy = cfg.retention.clone();
        tokio::spawn(async move {
            loop {
                sleep(prune_every).await;
                let log = log.clone();
                let policy = policy.clone();
                let dropped = tokio::task::spawn_blocking(move || log.prune(&policy)).await;
                match dropped {
                    Ok(Ok(n)) if n > 0 => info!(rows = n, "events sqlite: prune dropped rows"),
                    Ok(Err(e)) => warn!(error = %e, "events sqlite: prune failed"),
                    _ => {}
                }
            }
        });
    }

    let actor = ConnectorActor::new(client, cfg, rings.clone(), status.clone(), cmd_rx);
    let persist_failures = actor.persist_failures.clone();

    tokio::spawn(actor.run());

    EventsHandle {
        rings,
        status,
        cmd_tx,
        started_at,
        event_log,
        persist_failures,
    }
}

/// Largest integer a JavaScript `number` holds exactly. Cursors and epochs
/// are read by a TypeScript client, so neither may pass it.
const JS_MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Cursor headroom kept below [`JS_MAX_SAFE_INTEGER`]: 2^52 events is more
/// than any ring will see in one process lifetime.
const CURSOR_BASE_CEILING: u64 = JS_MAX_SAFE_INTEGER - (1 << 52);

/// How many cursors one millisecond of wall clock is worth. A restarted
/// process starts its cursors above the previous one as long as that one
/// averaged fewer than this many events per millisecond of uptime.
const CURSORS_PER_MS: u64 = 1_000;

/// The `reset_epoch` and first ring cursor for a process starting at
/// `now_ms` (Unix milliseconds).
///
/// Both used to start at 1 on every boot, so a client holding cursor 500
/// from the previous process was told "caught up" (204) until the new
/// process had seen 500 events on that topic — for `orders`, days. Seeding
/// from the clock puts the new process's cursors and epoch above the old
/// one's. `high_water` is the largest `(cursor, reset_epoch)` in the
/// persisted event log, if there is one, which covers a clock that came up
/// behind the last run (a Pi has no RTC). Where neither holds, the ring
/// answers a cursor it never issued with 412 instead (see [`TopicRing`]).
///
/// Epochs are milliseconds (~1.8e12) and cursors are milliseconds × 1000
/// (~1.8e15); both stay below 2^53 for the next two centuries, and the
/// cursor base is clamped to leave 2^52 of headroom regardless.
#[doc(hidden)]
#[must_use]
pub fn boot_seeds(now_ms: u64, high_water: Option<(u64, u64)>) -> (u64, u64) {
    let (hw_cursor, hw_epoch) = high_water.unwrap_or((0, 0));
    let epoch = now_ms
        .max(hw_epoch.saturating_add(1))
        .clamp(1, JS_MAX_SAFE_INTEGER);
    let base = now_ms
        .saturating_mul(CURSORS_PER_MS)
        .max(hw_cursor.saturating_add(1))
        .clamp(1, CURSOR_BASE_CEILING);
    (epoch, base)
}

fn unix_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

struct ConnectorActor {
    client: bezant::Client,
    cfg: ConnectorCfg,
    rings: Arc<RwLock<HashMap<String, TopicRing>>>,
    status: Arc<RwLock<StatusState>>,
    cmd_rx: mpsc::Receiver<ConnectorCmd>,
    active_marketdata_subs: BTreeSet<i64>,
    resubscribe: Resubscribe,
    link: Link,
    /// sqlite appends that failed since boot (see [`Self::persist`]).
    persist_failures: Arc<AtomicU64>,
    /// Set when the socket was dropped on request: reconnect without
    /// waiting out the backoff.
    reconnect_now: bool,
}

/// The connector's view of its own connection history — what decides when
/// the epoch moves and what gets logged.
#[derive(Debug, Default)]
struct Link {
    /// Whether any connect has ever succeeded in this process. The first
    /// one is not a reconnect: the boot seed already gave it a fresh epoch.
    ever_connected: bool,
    /// When the current connection came up, while it is up.
    connected_at: Option<Instant>,
    /// The outage in progress, if the link is down after having been up.
    outage: Option<Outage>,
    /// The kind of the last failure logged at WARN. A repeat of the same
    /// kind (the Gateway is still logged out, a minute later) goes to
    /// DEBUG; a change is news.
    last_failure: Option<String>,
}

/// One outage: from losing a connection to the next successful connect,
/// however many attempts that takes. It ends in exactly one gap event.
#[derive(Debug)]
struct Outage {
    /// RFC 3339 time the connection was lost.
    since: String,
    /// Connect attempts that failed during it.
    failed_attempts: u64,
}

/// A coarse, stable name for a connect/stream failure, for deciding
/// whether a failure is the same condition as the last one.
fn failure_kind(e: &bezant::Error) -> String {
    use bezant::Error as E;
    match e {
        E::NotAuthenticated => "not_authenticated".into(),
        E::NoSession => "no_session".into(),
        E::Http(h) if h.is_connect() => "gateway_unreachable".into(),
        E::Http(h) if h.is_timeout() => "gateway_timeout".into(),
        E::Http(_) => "http".into(),
        E::UpstreamStatus { status, .. } => format!("upstream_status_{status}"),
        E::WsHandshake { .. } => "ws_handshake".into(),
        E::WsTransport { .. } => "ws_transport".into(),
        E::WsProtocol(m) => format!("ws_protocol: {m}"),
        other => format!("other: {other}"),
    }
}

/// The standing subscriptions CPAPI has refused (or not yet honoured) on the
/// current socket, and when to ask again. Reset on every connect.
#[derive(Debug, Default)]
struct Resubscribe {
    /// Topics waiting on a subscribe that has not been honoured.
    unconfirmed: BTreeSet<&'static str>,
    /// When to send the next round, if any is due.
    due: Option<TokioInstant>,
    /// Delay to use for the NEXT round; doubles per round up to the ceiling.
    backoff: Option<Duration>,
    /// Rounds sent per topic since its last refusal.
    silent_rounds: BTreeMap<&'static str, u32>,
    /// Topics given up on as quiet-but-alive (see [`SubscriptionState::Quiet`]).
    quiet: BTreeSet<&'static str>,
}

impl Resubscribe {
    /// A round is due: decide, per unconfirmed topic, whether to ask again
    /// or to stop asking. Returns `(ask_again, gone_quiet)`.
    ///
    /// CPAPI refuses a subscribe it will not honour with an error frame, but
    /// honours `sor+{}` in silence when there are no live orders to
    /// snapshot — there is no ack. A topic that has been asked
    /// `quiet_after` times with no refusal is therefore subscribed as far as
    /// anyone can tell, and asking forever only keeps it `pending` in
    /// `/events/_status`. `quiet_after == 0` keeps asking forever.
    fn take_round(&mut self, quiet_after: u32) -> (Vec<&'static str>, Vec<&'static str>) {
        self.due = None;
        let mut ask = Vec::new();
        let mut gone_quiet = Vec::new();
        for topic in self.unconfirmed.clone() {
            let rounds = self.silent_rounds.entry(topic).or_insert(0);
            if quiet_after > 0 && *rounds >= quiet_after {
                self.unconfirmed.remove(topic);
                self.quiet.insert(topic);
                gone_quiet.push(topic);
            } else {
                *rounds += 1;
                ask.push(topic);
            }
        }
        if self.unconfirmed.is_empty() {
            self.backoff = None;
        }
        (ask, gone_quiet)
    }

    /// CPAPI said no: the topic is back on the retry list and its silence
    /// count starts over.
    fn refused(&mut self, topic: &'static str) {
        self.unconfirmed.insert(topic);
        self.quiet.remove(topic);
        self.silent_rounds.insert(topic, 0);
    }

    /// A real frame arrived. Returns whether the topic was waiting on one
    /// (unconfirmed or quiet), i.e. whether this frame is the confirmation.
    fn confirmed(&mut self, topic: &'static str) -> bool {
        self.silent_rounds.remove(topic);
        let was_waiting = self.unconfirmed.remove(topic) | self.quiet.remove(topic);
        if self.unconfirmed.is_empty() {
            self.due = None;
            self.backoff = None;
        }
        was_waiting
    }
}

/// The wire command that establishes each standing subscription.
const fn subscribe_command(topic: &str) -> Option<&'static str> {
    match topic.as_bytes() {
        b"orders" => Some("sor+{}"),
        b"pnl" => Some("spl+{}"),
        _ => None,
    }
}

impl ConnectorActor {
    fn new(
        client: bezant::Client,
        cfg: ConnectorCfg,
        rings: Arc<RwLock<HashMap<String, TopicRing>>>,
        status: Arc<RwLock<StatusState>>,
        cmd_rx: mpsc::Receiver<ConnectorCmd>,
    ) -> Self {
        Self {
            client,
            cfg,
            rings,
            status,
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
            resubscribe: Resubscribe::default(),
            link: Link::default(),
            persist_failures: Arc::default(),
            reconnect_now: false,
        }
    }

    /// Supervise [`Self::run_links`]. Nothing used to watch the connector
    /// task: a panic in it (the byte-slicing `truncate` could do it) ended
    /// the task, left `/events/_status` claiming `connected: true` forever
    /// and every ring frozen. Now a panic is caught, the link is marked
    /// down, counted in `connector_restarts`, and the loop starts over with
    /// the rings, epoch and command channel intact.
    async fn run(mut self) {
        info!("events connector starting");
        loop {
            let outcome = AssertUnwindSafe(self.run_links()).catch_unwind().await;
            let reason = match outcome {
                Ok(()) => "connector loop returned".to_owned(),
                Err(panic) => panic
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| panic.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "non-string panic".to_owned()),
            };
            self.on_crash(&reason).await;
            sleep(self.cfg.backoff_min).await;
        }
    }

    /// The connector task died mid-flight. Whatever it held is gone — the
    /// socket was dropped while unwinding — so report the link down and
    /// open an outage, and the next connect records it like any other.
    async fn on_crash(&mut self, reason: &str) {
        tracing::error!(reason, "events connector crashed; restarting it");
        let was_connected = self.link.connected_at.take().is_some();
        if was_connected {
            self.link.outage = Some(Outage {
                since: now_iso(),
                failed_attempts: 0,
            });
        }
        self.resubscribe = Resubscribe::default();
        let mut s = self.status.write().await;
        s.connected = false;
        s.connector_restarts += 1;
    }

    /// Connect, run, back off, repeat — forever.
    async fn run_links(&mut self) {
        let mut backoff = self.cfg.backoff_min;
        loop {
            let result = self.connect_and_run().await;
            let lived = self.link.connected_at.take().map(|t| t.elapsed());
            self.set_disconnected().await;
            self.note_link_down(lived.is_some(), result.as_ref().err());

            // A connection that held for a while was a healthy one; the
            // next failure is a fresh incident, not the tail of a storm.
            if lived.is_some_and(|d| d >= self.cfg.backoff_reset_after) {
                backoff = self.cfg.backoff_min;
            }
            if std::mem::take(&mut self.reconnect_now) || self.idle(backoff).await {
                // Asked to reconnect: go now, and start the backoff over.
                backoff = self.cfg.backoff_min;
                continue;
            }
            backoff = (backoff * 2).min(self.cfg.backoff_max);
        }
    }

    /// Sleep out a backoff while still answering commands — they used to
    /// queue unanswered until the next connect, so `/events/marketdata`
    /// hung for the whole backoff. Market-data requests are booked for the
    /// next connect. Returns `true` if a reconnect was requested, cutting
    /// the sleep short.
    async fn idle(&mut self, backoff: Duration) -> bool {
        let wake = sleep(backoff);
        tokio::pin!(wake);
        loop {
            tokio::select! {
                () = &mut wake => return false,
                Some(cmd) = self.cmd_rx.recv() => match cmd {
                    ConnectorCmd::Reconnect { reply } => {
                        info!("events connector: reconnect requested while down; connecting now");
                        let _ = reply.send(false);
                        return true;
                    }
                    ConnectorCmd::EnsureMarketData { conid, reply } => {
                        self.active_marketdata_subs.insert(conid);
                        self.add_topic_to_status(format!("marketdata:{conid}")).await;
                        let _ = reply.send(Ok(()));
                    }
                },
            }
        }
    }

    /// Book-keeping after a connect attempt or a connection ends. Opens an
    /// outage when a live connection was lost, counts a failed attempt
    /// otherwise, and logs a failure at WARN only when it is a different
    /// condition from the last one logged.
    fn note_link_down(&mut self, was_connected: bool, error: Option<&bezant::Error>) {
        if was_connected {
            self.link.outage = Some(Outage {
                since: now_iso(),
                failed_attempts: 0,
            });
        } else if let Some(outage) = self.link.outage.as_mut() {
            outage.failed_attempts += 1;
        }
        let Some(e) = error else {
            if !self.reconnect_now {
                info!("events connector: ws closed cleanly, reconnecting");
            }
            return;
        };
        let kind = failure_kind(e);
        if self.link.last_failure.as_deref() == Some(kind.as_str()) {
            debug!(error = %e, kind, "events connector: still failing the same way, will retry");
        } else {
            warn!(error = %e, kind, "events connector: ws failed, will retry");
            self.link.last_failure = Some(kind);
        }
    }

    /// A connect just succeeded. The first in the process keeps the boot
    /// epoch; every later one ends an outage, so the epoch moves exactly
    /// once and exactly one gap event records it — however many attempts
    /// failed in between (a logged-out Gateway fails one a minute for
    /// hours).
    async fn on_connected(&mut self) {
        self.link.connected_at = Some(Instant::now());
        if self.link.last_failure.take().is_some() {
            info!("events connector: recovered");
        }
        if !self.link.ever_connected {
            self.link.ever_connected = true;
            return;
        }
        let outage = self.link.outage.take();
        self.bump_epoch_with_gap(GapReason::ReconnectedAfterDisconnect, outage.as_ref())
            .await;
    }

    /// One full connect cycle. Returns `Ok(())` on clean close, `Err`
    /// otherwise. Caller handles backoff + reconnect.
    async fn connect_and_run(&mut self) -> Result<(), bezant::Error> {
        let mut ws = WsClient::connect(&self.client).await?;

        // CPAPI WS quirk: subscribes sent *before* the server's initial
        // `system+success` "ready" frame are silently discarded. Pump
        // frames until that frame arrives (or 5s timeout) so our
        // `sor`/`spl` subscribes actually take effect — without this,
        // we get heartbeats forever and no order/PnL events.
        let ready = timeout(Duration::from_secs(5), pump_until_ready(&mut ws)).await;
        match ready {
            Ok(Ok(())) => debug!("events connector: server ready signal received"),
            Ok(Err(e)) => return Err(e),
            Err(_) => warn!(
                "events connector: didn't see server-ready frame within 5s; \
                 subscribing anyway (CPAPI may silently drop these)"
            ),
        }

        ws.subscribe_orders().await?;
        ws.subscribe_pnl().await?;
        // Neither is honoured until CPAPI says so with a real frame; a
        // refusal or silence schedules a retry (see `handle_topic_frame`).
        self.resubscribe = Resubscribe::default();
        for topic in ["orders", "pnl"] {
            self.resubscribe.unconfirmed.insert(topic);
            self.set_subscription(topic, SubscriptionState::Pending)
                .await;
        }
        self.schedule_resubscribe();
        // Re-establish any market data subs that were active before the
        // disconnect.
        for conid in self.active_marketdata_subs.clone() {
            if let Err(e) = ws
                .subscribe_market_data(conid, &MarketDataFields::default_l1())
                .await
            {
                warn!(conid, error = %e, "events connector: re-subscribe market data failed");
            }
        }

        self.on_connected().await;
        self.set_connected().await;
        info!("events connector: connected, orders + pnl subscribed");

        let result = self.dispatch_loop(&mut ws).await;
        self.set_disconnected().await;
        result
    }

    /// The hot loop: read frames, dispatch into rings, handle commands,
    /// detect heartbeat timeout. Returns when the socket closes or any
    /// fatal error occurs.
    async fn dispatch_loop(&mut self, ws: &mut WsClient) -> Result<(), bezant::Error> {
        let session = ws.session().to_owned();
        let mut session_check = tokio::time::interval(self.cfg.session_check_every);
        session_check.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        session_check.tick().await; // the first tick fires immediately; the socket was just opened on this session
        loop {
            // A pending resubscribe is a timer; no pending resubscribe is a
            // future that never resolves. `sleep_until` needs an instant
            // either way, so park it a year out when there is nothing due.
            let resub_at = self
                .resubscribe
                .due
                .unwrap_or_else(|| TokioInstant::now() + Duration::from_secs(365 * 24 * 3_600));
            tokio::select! {
                // Frame from the upstream WS — dispatch or detect close.
                msg = timeout(self.cfg.heartbeat_timeout, ws.next_message()) => {
                    match msg {
                        Err(_elapsed) => {
                            warn!(
                                timeout_secs = self.cfg.heartbeat_timeout.as_secs(),
                                "events connector: heartbeat timeout, killing socket"
                            );
                            return Err(bezant::Error::WsProtocol(
                                "heartbeat timeout".into(),
                            ));
                        }
                        Ok(Ok(None)) => {
                            info!("events connector: upstream ws closed");
                            return Ok(());
                        }
                        Ok(Ok(Some(frame))) => {
                            self.handle_frame(frame).await;
                        }
                        Ok(Err(e)) => {
                            return Err(e);
                        }
                    }
                }
                // Command from the REST side (lazy market data subs).
                Some(cmd) = self.cmd_rx.recv() => {
                    if self.handle_command(ws, cmd).await {
                        info!("events connector: reconnect requested; dropping the socket");
                        self.reconnect_now = true;
                        return Ok(());
                    }
                }
                // A standing subscription CPAPI refused (or never answered).
                () = sleep_until(resub_at), if self.resubscribe.due.is_some() => {
                    self.resubscribe(ws).await;
                }
                // Has the Gateway re-logged in underneath this socket?
                _ = session_check.tick() => {
                    if self.session_rolled_over(&session).await {
                        self.status.write().await.session_rollovers += 1;
                        warn!("events connector: gateway session changed under the socket (re-login); reconnecting so subscriptions bind to the live session");
                        return Err(bezant::Error::WsProtocol("gateway session rolled over".into()));
                    }
                }
            }
        }
    }

    /// `/tickle` reports the Gateway's current session id. If it is not the
    /// one this socket was opened under, the Gateway has re-authenticated
    /// and everything subscribed on this socket is dead — however healthy
    /// its heartbeats look. A tickle that fails proves nothing, so it does
    /// not count.
    async fn session_rolled_over(&self, socket_session: &str) -> bool {
        match self.client.tickle().await {
            Ok(t) => matches!(t.session.as_deref(), Some(live) if live != socket_session),
            Err(e) => {
                debug!(error = %e, "events connector: session check tickle failed; assuming unchanged");
                false
            }
        }
    }

    /// Ask again for every standing subscription CPAPI has not honoured,
    /// after priming the brokerage session with `/iserver/accounts`. CPAPI
    /// documents that call as the precondition for order queries, and the
    /// refusals cluster right after a (re-)login — before anything has made
    /// it. Best-effort: a failed prime still sends the subscribe.
    async fn resubscribe(&mut self, ws: &mut WsClient) {
        let (topics, gone_quiet) = self.resubscribe.take_round(self.cfg.quiet_after_rounds);
        for topic in gone_quiet {
            info!(
                topic,
                rounds = self.cfg.quiet_after_rounds,
                "events connector: no refusal after repeated subscribes; treating the topic as quiet but alive"
            );
            self.set_subscription(topic, SubscriptionState::Quiet).await;
        }
        if topics.is_empty() {
            return;
        }
        if let Err(e) = self
            .client
            .api()
            .get_brokerage_accounts(bezant::api::GetBrokerageAccountsRequest::default())
            .await
        {
            debug!(error = %e, "events connector: /iserver/accounts prime failed before resubscribe");
        }
        for topic in topics {
            let Some(cmd) = subscribe_command(topic) else {
                continue;
            };
            match ws.send_text(cmd.to_owned()).await {
                Ok(()) => info!(topic, "events connector: resubscribing"),
                Err(e) => warn!(topic, error = %e, "events connector: resubscribe send failed"),
            }
            self.set_subscription(topic, SubscriptionState::Pending)
                .await;
        }
        // Whatever CPAPI says next — a refusal, a snapshot, or nothing —
        // the next round is already booked. Confirmation cancels it.
        self.schedule_resubscribe();
    }

    /// Book the next resubscribe round, doubling the delay each time up to
    /// the ceiling. Idempotent while a round is already booked.
    fn schedule_resubscribe(&mut self) {
        if self.resubscribe.unconfirmed.is_empty() || self.resubscribe.due.is_some() {
            return;
        }
        let delay = self.resubscribe.backoff.unwrap_or(self.cfg.resubscribe_min);
        self.resubscribe.due = Some(TokioInstant::now() + delay);
        self.resubscribe.backoff = Some((delay * 2).min(self.cfg.resubscribe_max));
    }

    async fn set_subscription(&self, topic: &str, state: SubscriptionState) {
        self.status
            .write()
            .await
            .subscriptions
            .insert(topic.to_owned(), state);
    }

    /// A frame on a standing topic is either CPAPI honouring the subscribe
    /// (any real payload) or refusing it (`{"error": …}`). Only the former
    /// is an event.
    async fn handle_topic_frame(
        &mut self,
        topic: &'static str,
        value: serde_json::Value,
        now: String,
    ) {
        if let Some(err) = value.get("error") {
            let code = value.get("code").and_then(serde_json::Value::as_i64);
            warn!(
                topic,
                error = %err,
                code,
                "events connector: CPAPI refused the subscription; will retry with backoff"
            );
            {
                let mut s = self.status.write().await;
                s.subscribe_refusals += 1;
                s.last_message_at = Some(now);
            }
            self.set_subscription(topic, SubscriptionState::Refused)
                .await;
            self.resubscribe.refused(topic);
            self.schedule_resubscribe();
            return;
        }
        if self.resubscribe.confirmed(topic) {
            info!(
                topic,
                "events connector: subscription confirmed by first frame"
            );
            self.set_subscription(topic, SubscriptionState::Subscribed)
                .await;
        }
        self.push_to_topic(topic, value, now).await;
    }

    /// Decode + push a single WS frame into the appropriate ring.
    async fn handle_frame(&mut self, frame: WsMessage) {
        let now = now_iso();
        // Diagnostic: trace topic + first 200 chars of payload for every
        // frame so we can see CPAPI's actual topic strings for unrecognised
        // sor/spl variants. Cheap; gated to debug level so prod is silent.
        if let Some(v) = frame.as_value() {
            let snippet = serde_json::to_string(v).unwrap_or_default();
            let topic_str = v
                .get("topic")
                .and_then(|t| t.as_str())
                .unwrap_or("<no-topic>");
            debug!(
                variant = frame.topic(),
                topic = topic_str,
                payload = %truncate(&snippet, 200),
                "events connector: frame"
            );
        }
        match frame {
            WsMessage::Heartbeat | WsMessage::System(_) | WsMessage::Other(_) => {
                // Not interesting for downstream consumers; just record
                // last-message time so the connector status reflects life.
                self.touch_last_message(now).await;
            }
            WsMessage::Order(value) => {
                self.handle_topic_frame("orders", value, now).await;
            }
            WsMessage::Pnl(value) => {
                self.handle_topic_frame("pnl", value, now).await;
            }
            WsMessage::MarketData { conid, payload } => {
                let topic = format!("marketdata:{conid}");
                self.push_to_topic(&topic, payload, now).await;
            }
            WsMessage::Malformed { text, error } => {
                warn!(error = %error, sample = %truncate(&text, 200), "events connector: malformed frame");
                self.touch_last_message(now).await;
            }
            // `WsMessage` is `#[non_exhaustive]` — future variants
            // surface as unstructured "other" data so we don't silently
            // drop them.
            other => {
                debug!(
                    topic = other.topic(),
                    "events connector: unhandled ws message variant"
                );
                self.touch_last_message(now).await;
            }
        }
    }

    /// Returns `true` when the command asks for the socket to be dropped.
    async fn handle_command(&mut self, ws: &mut WsClient, cmd: ConnectorCmd) -> bool {
        match cmd {
            ConnectorCmd::Reconnect { reply } => {
                let _ = reply.send(true);
                return true;
            }
            ConnectorCmd::EnsureMarketData { conid, reply } => {
                if self.active_marketdata_subs.insert(conid) {
                    debug!(conid, "events connector: subscribing market data");
                    let result = ws
                        .subscribe_market_data(conid, &MarketDataFields::default_l1())
                        .await;
                    match result {
                        Ok(_) => {
                            self.add_topic_to_status(format!("marketdata:{conid}"))
                                .await;
                            let _ = reply.send(Ok(()));
                        }
                        Err(e) => {
                            // Roll back the optimistic insert — next caller
                            // can retry.
                            self.active_marketdata_subs.remove(&conid);
                            let _ = reply.send(Err(format!("subscribe failed: {e}")));
                        }
                    }
                } else {
                    // Already subscribed.
                    let _ = reply.send(Ok(()));
                }
            }
        }
        false
    }

    async fn push_to_topic(&self, topic: &str, payload: serde_json::Value, received_at: String) {
        let cap = match topic {
            "orders" => self.cfg.orders_capacity,
            "pnl" => self.cfg.pnl_capacity,
            t if t.starts_with("marketdata:") => self.cfg.marketdata_capacity,
            _ => 1_000,
        };
        let (epoch, base) = {
            let s = self.status.read().await;
            (s.reset_epoch, s.cursor_base)
        };

        let mut rings = self.rings.write().await;
        let ring = rings
            .entry(topic.to_string())
            .or_insert_with(|| TopicRing::with_base(topic, cap, epoch, base));
        let cursor = ring.push(payload.clone(), received_at.clone());
        let epoch = ring.reset_epoch();
        drop(rings);

        self.persist(ObservedEvent {
            cursor,
            topic: topic.to_string(),
            received_at: received_at.clone(),
            reset_epoch: epoch,
            payload,
        });

        // Update last_message_at + ensure topic shows in status.
        let mut s = self.status.write().await;
        s.last_message_at = Some(received_at);
        s.topics_subscribed.insert(topic.to_string());
    }

    /// Mirror an event to sqlite if configured. Best-effort — the in-memory
    /// ring remains the canonical fast-path read.
    ///
    /// A failed append used to vanish (the `spawn_blocking` result was
    /// dropped). It is now counted in `persist_failures` and logged at
    /// WARN on the 1st, 2nd, 4th, 8th… failure, so a full disk is loud
    /// once without a line per P&L tick.
    fn persist(&self, evt: ObservedEvent) {
        if let Some(log) = &self.cfg.event_log {
            let log = log.clone();
            let failures = self.persist_failures.clone();
            // sqlite writes are blocking; offload to a worker so we
            // don't block the connector loop on I/O.
            tokio::task::spawn_blocking(move || {
                if let Err(e) = log.append(&evt) {
                    let n = failures.fetch_add(1, Ordering::Relaxed) + 1;
                    if n.is_power_of_two() {
                        warn!(error = %e, topic = %evt.topic, failures = n, "events sqlite: append failed; history will be missing events");
                    }
                }
            });
        }
    }

    async fn touch_last_message(&self, now: String) {
        self.status.write().await.last_message_at = Some(now);
    }

    async fn add_topic_to_status(&self, topic: String) {
        self.status.write().await.topics_subscribed.insert(topic);
    }

    async fn set_connected(&self) {
        let mut s = self.status.write().await;
        s.connected = true;
        s.reconnect_count = s.reconnect_count.saturating_add(1);
        s.topics_subscribed.insert("orders".into());
        s.topics_subscribed.insert("pnl".into());
    }

    async fn set_disconnected(&self) {
        self.status.write().await.connected = false;
    }

    /// Increment `reset_epoch`, move every ring to it, and record the reset
    /// as one event on the `gap` topic.
    ///
    /// The gap goes ONLY to `gap`. It used to be pushed into every ring as
    /// well, where an `orders` reader found a frame that was not an order
    /// and a `pnl` reader one that was not P&L — while the epoch change on
    /// every ring already tells each reader its topic had a gap.
    async fn bump_epoch_with_gap(&self, reason: GapReason, outage: Option<&Outage>) {
        let (new_epoch, base) = {
            let mut s = self.status.write().await;
            s.reset_epoch = s.reset_epoch.saturating_add(1);
            (s.reset_epoch, s.cursor_base)
        };

        let now = now_iso();
        let mut payload = json!({
            "reason": match reason {
                GapReason::ReconnectedAfterDisconnect => "reconnected_after_disconnect",
                GapReason::ProcessRestart => "process_restart",
            },
            "previous_reset_epoch": new_epoch - 1,
            "new_reset_epoch": new_epoch,
        });
        if let Some(o) = outage {
            payload["disconnected_at"] = json!(o.since);
            payload["failed_attempts"] = json!(o.failed_attempts);
        }

        let mut rings = self.rings.write().await;
        for ring in rings.values_mut() {
            ring.set_reset_epoch(new_epoch);
        }
        let cursor = rings
            .entry("gap".to_string())
            .or_insert_with(|| TopicRing::with_base("gap", 256, new_epoch, base))
            .push(payload.clone(), now.clone());
        drop(rings);
        self.persist(ObservedEvent {
            cursor,
            topic: "gap".into(),
            received_at: now,
            reset_epoch: new_epoch,
            payload,
        });
    }
}

/// Pump WS frames until we see a `system` topic frame containing
/// `success` (CPAPI's username-ack — see the IBKR campus WS lesson).
/// That frame indicates the server has finished session bootstrap and
/// will accept and broadcast on `sor`/`spl` subscribes. Frames seen
/// before the ready signal are discarded — they're just connection
/// metadata (`act`, `sts`) we don't need to surface to consumers.
async fn pump_until_ready(ws: &mut WsClient) -> Result<(), bezant::Error> {
    while let Some(msg) = ws.next_message().await? {
        if let WsMessage::System(value) = &msg {
            if value.get("success").is_some() {
                return Ok(());
            }
        }
        // `act`, `sts`, anything else — ignore. We're only gating on
        // the `success` ack which CPAPI sends once per session.
    }
    // Stream closed before we saw ready.
    Err(bezant::Error::WsProtocol(
        "ws closed before server-ready frame".into(),
    ))
}

fn now_iso() -> String {
    use std::time::SystemTime;
    let now: chrono::DateTime<chrono::Utc> = SystemTime::now().into();
    now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// At most `max` bytes of `s`, cut on a char boundary. Slicing at a raw
/// byte offset panicked the connector task on any frame whose 200th byte
/// fell inside a multi-byte character — an accented company name in an
/// order frame was enough.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &s[..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn handle_frame_dispatches_to_correct_topic() {
        // Build an actor with a closed cmd channel — we won't drive run().
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        );

        actor
            .handle_frame(WsMessage::Order(json!({"orderId": 1})))
            .await;
        actor
            .handle_frame(WsMessage::Pnl(json!({"upnl": 1.0})))
            .await;
        actor
            .handle_frame(WsMessage::MarketData {
                conid: 265598,
                payload: json!({"31": "150.25"}),
            })
            .await;

        let rings = actor.rings.read().await;
        assert_eq!(rings.get("orders").unwrap().len(), 1);
        assert_eq!(rings.get("pnl").unwrap().len(), 1);
        assert_eq!(rings.get("marketdata:265598").unwrap().len(), 1);
    }

    fn test_actor() -> ConnectorActor {
        let (tx, cmd_rx) = mpsc::channel(1);
        std::mem::forget(tx);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        )
    }

    // The frame CPAPI actually sends — 17 of 20 subscribe attempts on one
    // Gateway over Aug–Sep 2026 got this, and every one was filed as an
    // order event while the subscribe was never retried.
    fn refusal() -> serde_json::Value {
        json!({"error": "unable to subscribe", "code": 500, "topic": "sor"})
    }

    #[tokio::test]
    async fn a_subscribe_refusal_is_not_an_event() {
        let mut actor = test_actor();
        actor.resubscribe.unconfirmed.insert("orders");

        actor.handle_frame(WsMessage::Order(refusal())).await;

        let rings = actor.rings.read().await;
        assert!(
            rings.get("orders").is_none(),
            "a refusal must not create or fill the orders ring — consumers would read it as an order frame"
        );
        let s = actor.status.read().await;
        assert_eq!(
            s.subscriptions.get("orders"),
            Some(&SubscriptionState::Refused)
        );
        assert_eq!(s.subscribe_refusals, 1);
        assert!(
            s.last_message_at.is_some(),
            "it is still a sign of life on the socket"
        );
    }

    #[tokio::test]
    async fn a_refusal_books_a_retry_and_the_first_real_frame_cancels_it() {
        let mut actor = test_actor();
        actor.resubscribe.unconfirmed.insert("orders");

        actor.handle_frame(WsMessage::Order(refusal())).await;
        assert!(actor.resubscribe.due.is_some(), "a retry must be scheduled");
        assert!(actor.resubscribe.unconfirmed.contains("orders"));

        // CPAPI honouring the subscribe looks like a snapshot: {"topic":"sor","args":[…]}.
        actor
            .handle_frame(WsMessage::Order(json!({"topic": "sor", "args": []})))
            .await;
        assert!(
            actor.resubscribe.due.is_none(),
            "confirmed: nothing left to retry"
        );
        assert!(actor.resubscribe.unconfirmed.is_empty());
        assert_eq!(
            actor.status.read().await.subscriptions.get("orders"),
            Some(&SubscriptionState::Subscribed)
        );
        assert_eq!(
            actor.rings.read().await.get("orders").unwrap().len(),
            1,
            "the snapshot IS an event"
        );
    }

    #[tokio::test]
    async fn retry_delay_doubles_to_the_ceiling_and_resets_on_confirmation() {
        let mut actor = test_actor();
        actor.cfg.resubscribe_min = Duration::from_secs(5);
        actor.cfg.resubscribe_max = Duration::from_secs(12);
        actor.resubscribe.unconfirmed.insert("pnl");

        actor.schedule_resubscribe();
        assert_eq!(actor.resubscribe.backoff, Some(Duration::from_secs(10)));
        actor.resubscribe.due = None; // as `resubscribe()` does when a round is sent
        actor.schedule_resubscribe();
        assert_eq!(
            actor.resubscribe.backoff,
            Some(Duration::from_secs(12)),
            "capped"
        );

        actor
            .handle_frame(WsMessage::Pnl(
                json!({"topic": "spl", "args": {"upnl": 1.0}}),
            ))
            .await;
        assert_eq!(
            actor.resubscribe.backoff, None,
            "the next socket starts from the minimum again"
        );
    }

    #[tokio::test]
    async fn one_topic_confirming_does_not_cancel_the_other_topic_retry() {
        let mut actor = test_actor();
        actor.resubscribe.unconfirmed.insert("orders");
        actor.resubscribe.unconfirmed.insert("pnl");
        actor.schedule_resubscribe();

        // pnl confirms (it nearly always does); orders is still refused.
        actor
            .handle_frame(WsMessage::Pnl(json!({"upnl": 1.0})))
            .await;
        assert!(
            actor.resubscribe.due.is_some(),
            "orders still needs its retry"
        );
        assert_eq!(
            actor
                .resubscribe
                .unconfirmed
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec!["orders"]
        );
    }

    /// `orders` with no live orders: CPAPI honours `sor+{}` in silence, so
    /// the topic sat `pending` and was re-asked every 5 minutes forever.
    #[tokio::test]
    async fn orders_goes_quiet_after_n_silent_rounds() {
        let mut r = Resubscribe::default();
        r.unconfirmed.insert("orders");
        for round in 1..=3 {
            let (ask, quiet) = r.take_round(3);
            assert_eq!(ask, vec!["orders"], "round {round} still asks");
            assert!(quiet.is_empty());
        }
        let (ask, quiet) = r.take_round(3);
        assert!(ask.is_empty(), "no fourth ask");
        assert_eq!(quiet, vec!["orders"]);
        assert!(r.unconfirmed.is_empty());
        assert_eq!(r.backoff, None, "nothing left to schedule");

        // The actor reports it, and a later order frame confirms it.
        let mut actor = test_actor();
        actor.resubscribe = r;
        actor
            .set_subscription("orders", SubscriptionState::Quiet)
            .await;
        actor
            .handle_frame(WsMessage::Order(json!({"orderId": 1})))
            .await;
        assert_eq!(
            actor.status.read().await.subscriptions.get("orders"),
            Some(&SubscriptionState::Subscribed)
        );
    }

    #[test]
    fn a_refusal_restarts_the_silence_count_and_leaves_quiet() {
        let mut r = Resubscribe::default();
        r.unconfirmed.insert("orders");
        r.take_round(2);
        r.take_round(2);
        r.refused("orders");
        // Two more asks are owed after the refusal before it can go quiet.
        assert_eq!(r.take_round(2).0, vec!["orders"]);
        assert_eq!(r.take_round(2).0, vec!["orders"]);
        assert_eq!(r.take_round(2).1, vec!["orders"]);
        // And a refusal while quiet puts it back on the retry list.
        r.refused("orders");
        assert!(r.unconfirmed.contains("orders"));
        assert!(!r.quiet.contains("orders"));
    }

    #[test]
    fn zero_rounds_means_ask_forever() {
        let mut r = Resubscribe::default();
        r.unconfirmed.insert("orders");
        for _ in 0..50 {
            assert_eq!(r.take_round(0).0, vec!["orders"]);
        }
    }

    #[test]
    fn truncate_cuts_on_a_char_boundary() {
        // 'é' is two bytes; a cut at an odd byte used to panic.
        let s = "é".repeat(150);
        let t = truncate(&s, 199);
        assert!(t.ends_with('…'));
        assert_eq!(t.trim_end_matches('…').len(), 198);
        assert_eq!(truncate("ok", 200), "ok");
        // Every cut point of a mixed string is safe.
        let mixed = "a€b😀c";
        for max in 0..mixed.len() {
            let _ = truncate(mixed, max);
        }
    }

    #[tokio::test]
    async fn a_crash_marks_the_link_down_and_opens_an_outage() {
        let mut actor = test_actor();
        actor.on_connected().await;
        actor.status.write().await.connected = true;
        let epoch = actor.status.read().await.reset_epoch;

        actor.on_crash("boom").await;

        {
            let s = actor.status.read().await;
            assert!(!s.connected);
            assert_eq!(s.connector_restarts, 1);
        }
        // The next connect is a reconnect with its one gap.
        actor.on_connected().await;
        assert_eq!(actor.status.read().await.reset_epoch, epoch + 1);
        assert_eq!(actor.rings.read().await.get("gap").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn the_supervisor_survives_a_panic_in_the_loop() {
        // The shape `run` relies on: a panic inside a future borrowing the
        // actor is caught, and the actor is usable afterwards.
        let mut actor = test_actor();
        let caught = AssertUnwindSafe(async {
            actor.link.ever_connected = true;
            panic!("inside the loop");
        })
        .catch_unwind()
        .await;
        assert!(caught.is_err());
        actor.on_crash("inside the loop").await;
        assert_eq!(actor.status.read().await.connector_restarts, 1);
    }

    #[tokio::test]
    async fn a_reconnect_request_cuts_the_backoff_short() {
        let (tx, cmd_rx) = mpsc::channel(4);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        );
        let (reply, ack) = oneshot::channel();
        tx.send(ConnectorCmd::Reconnect { reply }).await.unwrap();
        let started = Instant::now();
        assert!(actor.idle(Duration::from_secs(60)).await);
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(!ack.await.unwrap(), "no socket was up");
    }

    #[tokio::test]
    async fn market_data_requested_while_down_is_booked_not_hung() {
        let (tx, cmd_rx) = mpsc::channel(4);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        );
        let (reply, ack) = oneshot::channel();
        tx.send(ConnectorCmd::EnsureMarketData {
            conid: 265_598,
            reply,
        })
        .await
        .unwrap();
        assert!(!actor.idle(Duration::from_millis(50)).await);
        assert_eq!(ack.await.unwrap(), Ok(()));
        assert!(actor.active_marketdata_subs.contains(&265_598));
    }

    #[test]
    fn standing_topics_map_to_their_wire_commands() {
        assert_eq!(subscribe_command("orders"), Some("sor+{}"));
        assert_eq!(subscribe_command("pnl"), Some("spl+{}"));
        assert_eq!(subscribe_command("marketdata:1"), None);
    }

    #[tokio::test]
    async fn heartbeat_and_system_frames_dont_create_topics() {
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        );

        actor.handle_frame(WsMessage::Heartbeat).await;
        actor
            .handle_frame(WsMessage::System(json!({"msg": "ready"})))
            .await;

        let rings = actor.rings.read().await;
        assert!(rings.is_empty());

        // But last_message_at IS updated.
        let s = actor.status.read().await;
        assert!(s.last_message_at.is_some());
    }

    #[tokio::test]
    async fn the_first_connect_keeps_the_boot_epoch_and_leaves_no_gap() {
        let mut actor = test_actor();
        actor.status.write().await.reset_epoch = 42;

        actor.on_connected().await;

        assert!(actor.rings.read().await.is_empty(), "no gap on first boot");
        assert_eq!(actor.status.read().await.reset_epoch, 42);
    }

    /// The epoch used to bump BEFORE every connect attempt, so a Gateway
    /// that was logged out for an hour (one failed attempt a minute) moved
    /// it sixty times and left sixty gap events.
    #[tokio::test]
    async fn failed_connects_do_not_move_the_epoch_and_one_outage_is_one_gap() {
        let mut actor = test_actor();
        actor.status.write().await.reset_epoch = 42;
        actor.on_connected().await;

        // The connection drops, then five attempts fail.
        actor.note_link_down(true, Some(&bezant::Error::WsProtocol("closed".into())));
        for _ in 0..5 {
            actor.note_link_down(false, Some(&bezant::Error::NotAuthenticated));
        }
        assert_eq!(
            actor.status.read().await.reset_epoch,
            42,
            "failed attempts leave the epoch alone"
        );
        assert!(actor.rings.read().await.get("gap").is_none());

        actor.on_connected().await;

        assert_eq!(actor.status.read().await.reset_epoch, 43);
        let rings = actor.rings.read().await;
        let gap = rings.get("gap").unwrap();
        assert_eq!(gap.len(), 1, "one outage, one gap");
        match gap.read_since(0, 10) {
            ReadResult::Ok { events, .. } => {
                assert_eq!(events[0].payload["failed_attempts"], json!(5));
                assert_eq!(events[0].payload["previous_reset_epoch"], json!(42));
                assert_eq!(events[0].payload["new_reset_epoch"], json!(43));
                assert!(events[0].payload["disconnected_at"].is_string());
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn the_run_loop_does_not_bump_the_epoch_on_failed_connects() {
        // The real loop against a Gateway that refuses connections: the
        // epoch must stay at its boot seed however many attempts fail.
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://127.0.0.1:1/v1/api").unwrap();
        let cfg = ConnectorCfg {
            backoff_min: Duration::from_millis(1),
            backoff_max: Duration::from_millis(2),
            ..ConnectorCfg::default()
        };
        let status = Arc::new(RwLock::new(StatusState {
            reset_epoch: 1_000,
            ..StatusState::default()
        }));
        let rings = Arc::new(RwLock::new(HashMap::new()));
        let actor = ConnectorActor::new(client, cfg, rings.clone(), status.clone(), cmd_rx);
        let task = tokio::spawn(actor.run());
        sleep(Duration::from_millis(200)).await;
        task.abort();

        assert_eq!(status.read().await.reset_epoch, 1_000);
        assert!(!status.read().await.connected);
        assert!(rings.read().await.is_empty(), "no gap events either");
    }

    #[tokio::test]
    async fn gap_events_land_only_in_the_gap_topic() {
        let mut actor = test_actor();
        actor.on_connected().await;
        actor.handle_frame(WsMessage::Order(json!({"id": 1}))).await;
        actor
            .handle_frame(WsMessage::Pnl(json!({"upnl": 1.0})))
            .await;
        let before = actor.status.read().await.reset_epoch;

        actor.note_link_down(true, None);
        actor.on_connected().await;

        let rings = actor.rings.read().await;
        assert_eq!(rings.get("orders").unwrap().len(), 1, "no gap in orders");
        assert_eq!(rings.get("pnl").unwrap().len(), 1, "no gap in pnl");
        assert_eq!(rings.get("gap").unwrap().len(), 1);
        // But every ring reports the new epoch, so its readers see the reset.
        for topic in ["orders", "pnl", "gap"] {
            assert_eq!(rings.get(topic).unwrap().reset_epoch(), before + 1);
        }
    }

    #[test]
    fn failure_kinds_group_repeats_of_the_same_condition() {
        assert_eq!(
            failure_kind(&bezant::Error::NotAuthenticated),
            failure_kind(&bezant::Error::NotAuthenticated)
        );
        assert_ne!(
            failure_kind(&bezant::Error::NotAuthenticated),
            failure_kind(&bezant::Error::NoSession)
        );
    }

    #[test]
    fn boot_seeds_rise_with_the_clock_and_stay_js_safe() {
        let now_ms = 1_790_000_000_000; // 2026
        let (e1, b1) = boot_seeds(now_ms, None);
        let (e2, b2) = boot_seeds(now_ms + 1, None);
        assert!(e2 > e1 && b2 > b1);
        // A process that ran a day and saw a million events per topic
        // still hands out cursors below the next boot's base.
        let (_, b_next) = boot_seeds(now_ms + 86_400_000, None);
        assert!(b1 + 1_000_000 < b_next);
        for v in [e1, b1] {
            assert!(v <= JS_MAX_SAFE_INTEGER);
        }
        // Far future: clamped, never past 2^53.
        let (e, b) = boot_seeds(u64::MAX / 2, None);
        assert!(e <= JS_MAX_SAFE_INTEGER && b <= JS_MAX_SAFE_INTEGER);
    }

    #[test]
    fn boot_seeds_stay_above_the_log_when_the_clock_is_behind() {
        // The Pi booted before NTP synced: the clock reads an hour behind
        // the last run, whose log holds these values.
        let last_run = boot_seeds(1_790_000_000_000, None);
        let hw = (last_run.1 + 42, last_run.0 + 3);
        let (epoch, base) = boot_seeds(1_790_000_000_000 - 3_600_000, Some(hw));
        assert!(epoch > hw.1);
        assert!(base > hw.0);
    }

    #[tokio::test]
    async fn ensure_market_data_dedups_repeat_calls() {
        // We can't easily test the WS sub call without a mock socket,
        // but we can verify the dedup logic at the actor level by
        // hand-mutating the active_marketdata_subs set.
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor::new(
            client,
            ConnectorCfg::default(),
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
        );

        assert!(actor.active_marketdata_subs.insert(265_598));
        // Re-insert returns false (already present).
        assert!(!actor.active_marketdata_subs.insert(265_598));
        assert_eq!(actor.active_marketdata_subs.len(), 1);
    }
}
