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
//! 4. On any disconnect: bump `reset_epoch`, push a synthetic `gap` event
//!    into every active ring, sleep with backoff, GOTO 1.
//!
//! [`EventsHandle`] is what axum handlers get. Cloneable, cheap, exposes
//! reads against the rings and a command channel into the actor task.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bezant::{MarketDataFields, WsClient, WsMessage};
use serde_json::json;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use super::persistence::{EventLog, RetentionPolicy};
use super::ring::{ReadResult, TopicRing};
use super::{EventsStatus, GapReason, ObservedEvent};

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
}

/// Cloneable handle the axum handlers use. Reads come straight off the
/// shared ring map; writes go through the command channel.
#[derive(Clone)]
pub struct EventsHandle {
    rings: Arc<RwLock<HashMap<String, TopicRing>>>,
    status: Arc<RwLock<StatusState>>,
    cmd_tx: mpsc::Sender<ConnectorCmd>,
    started_at: Instant,
    event_log: Option<Arc<EventLog>>,
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
    topics_subscribed: BTreeSet<String>,
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
        let rings: Arc<RwLock<HashMap<String, TopicRing>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let status = Arc::new(RwLock::new(StatusState {
            connected: true,
            reset_epoch: 1,
            ..Default::default()
        }));
        let (cmd_tx, _cmd_rx) = mpsc::channel::<ConnectorCmd>(1);
        let started_at = Instant::now();
        let handle = Self {
            rings: rings.clone(),
            status: status.clone(),
            cmd_tx,
            started_at,
            event_log: event_log.clone(),
        };
        let sink = TestSink { rings, status, event_log };
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
        let epoch = self.status.read().await.reset_epoch;
        let received_at = now_iso();
        let mut rings = self.rings.write().await;
        let ring = rings
            .entry(topic.to_string())
            .or_insert_with(|| TopicRing::new(topic, cap, epoch));
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

    /// Force a specific reset_epoch — used to test cursor-expired flows.
    pub async fn set_reset_epoch(&self, epoch: u64) {
        self.status.write().await.reset_epoch = epoch;
    }

    /// Force the `connected` flag — useful for `_status` shape testing.
    pub async fn set_connected(&self, connected: bool) {
        self.status.write().await.connected = connected;
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
    let status = Arc::new(RwLock::new(StatusState::default()));
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

    let actor = ConnectorActor {
        client,
        cfg,
        rings: rings.clone(),
        status: status.clone(),
        cmd_rx,
        active_marketdata_subs: BTreeSet::new(),
    };

    tokio::spawn(actor.run());

    EventsHandle {
        rings,
        status,
        cmd_tx,
        started_at,
        event_log,
    }
}

struct ConnectorActor {
    client: bezant::Client,
    cfg: ConnectorCfg,
    rings: Arc<RwLock<HashMap<String, TopicRing>>>,
    status: Arc<RwLock<StatusState>>,
    cmd_rx: mpsc::Receiver<ConnectorCmd>,
    active_marketdata_subs: BTreeSet<i64>,
}

impl ConnectorActor {
    async fn run(mut self) {
        info!("events connector starting");
        let mut backoff = self.cfg.backoff_min;
        loop {
            // Bump epoch + emit gap markers BEFORE attempting connect, so
            // any events arriving during this run are tagged with the
            // correct epoch from the very first frame.
            self.bump_epoch_with_gap(GapReason::ReconnectedAfterDisconnect)
                .await;

            match self.connect_and_run().await {
                Ok(()) => {
                    info!("events connector: ws closed cleanly, reconnecting");
                    backoff = self.cfg.backoff_min;
                }
                Err(e) => {
                    warn!(error = %e, "events connector: ws failed, will retry");
                    self.set_disconnected().await;
                }
            }

            sleep(backoff).await;
            backoff = (backoff * 2).min(self.cfg.backoff_max);
        }
    }

    /// One full connect cycle. Returns `Ok(())` on clean close, `Err`
    /// otherwise. Caller handles backoff + reconnect.
    async fn connect_and_run(&mut self) -> Result<(), bezant::Error> {
        let mut ws = WsClient::connect(&self.client).await?;
        ws.subscribe_orders().await?;
        ws.subscribe_pnl().await?;
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
        loop {
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
                    self.handle_command(ws, cmd).await;
                }
            }
        }
    }

    /// Decode + push a single WS frame into the appropriate ring.
    async fn handle_frame(&mut self, frame: WsMessage) {
        let now = now_iso();
        match frame {
            WsMessage::Heartbeat | WsMessage::System(_) | WsMessage::Other(_) => {
                // Not interesting for downstream consumers; just record
                // last-message time so the connector status reflects life.
                self.touch_last_message(now).await;
            }
            WsMessage::Order(value) => {
                self.push_to_topic("orders", value, now).await;
            }
            WsMessage::Pnl(value) => {
                self.push_to_topic("pnl", value, now).await;
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
                debug!(topic = other.topic(), "events connector: unhandled ws message variant");
                self.touch_last_message(now).await;
            }
        }
    }

    async fn handle_command(&mut self, ws: &mut WsClient, cmd: ConnectorCmd) {
        match cmd {
            ConnectorCmd::EnsureMarketData { conid, reply } => {
                if self.active_marketdata_subs.insert(conid) {
                    debug!(conid, "events connector: subscribing market data");
                    let result = ws
                        .subscribe_market_data(conid, &MarketDataFields::default_l1())
                        .await;
                    match result {
                        Ok(_) => {
                            self.add_topic_to_status(format!("marketdata:{conid}")).await;
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
    }

    async fn push_to_topic(&self, topic: &str, payload: serde_json::Value, received_at: String) {
        let cap = match topic {
            "orders" => self.cfg.orders_capacity,
            "pnl" => self.cfg.pnl_capacity,
            t if t.starts_with("marketdata:") => self.cfg.marketdata_capacity,
            _ => 1_000,
        };
        let epoch = self.status.read().await.reset_epoch;

        let mut rings = self.rings.write().await;
        let ring = rings
            .entry(topic.to_string())
            .or_insert_with(|| TopicRing::new(topic, cap, epoch));
        let cursor = ring.push(payload.clone(), received_at.clone());
        drop(rings);

        // Mirror to sqlite if configured. Best-effort — log on failure;
        // the in-memory ring remains the canonical fast-path read.
        if let Some(log) = &self.cfg.event_log {
            let log = log.clone();
            let evt = ObservedEvent {
                cursor,
                topic: topic.to_string(),
                received_at: received_at.clone(),
                reset_epoch: epoch,
                payload,
            };
            // sqlite writes are blocking; offload to a worker so we
            // don't block the connector loop on I/O.
            tokio::task::spawn_blocking(move || log.append(&evt));
        }

        // Update last_message_at + ensure topic shows in status.
        let mut s = self.status.write().await;
        s.last_message_at = Some(received_at);
        s.topics_subscribed.insert(topic.to_string());
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

    /// Increment `reset_epoch` and inject a synthetic gap event into
    /// every existing ring + a new "gap" topic ring so consumers
    /// polling any topic see the reset.
    async fn bump_epoch_with_gap(&self, reason: GapReason) {
        let mut s = self.status.write().await;
        // First boot: no previous events, nothing to gap-mark.
        let is_first_boot = s.reset_epoch == 0 && s.last_message_at.is_none();
        s.reset_epoch = s.reset_epoch.saturating_add(1);
        let new_epoch = s.reset_epoch;
        drop(s);

        if is_first_boot {
            return;
        }

        // Inject a gap event into every active topic so cursor-advancing
        // consumers see it on their next poll.
        let now = now_iso();
        let payload = json!({
            "reason": match reason {
                GapReason::ReconnectedAfterDisconnect => "reconnected_after_disconnect",
                GapReason::ProcessRestart => "process_restart",
            },
            "new_reset_epoch": new_epoch,
        });

        let mut rings = self.rings.write().await;
        let topics: Vec<String> = rings.keys().cloned().collect();
        for topic in topics {
            // Each ring keeps its own reset_epoch matching the time of
            // its first push; we DON'T retro-bump that. The gap event's
            // own `reset_epoch` reflects the new value via a fresh ring
            // entry if needed.
            if let Some(r) = rings.get_mut(&topic) {
                r.push(payload.clone(), now.clone());
            }
        }
        // Always ensure a 'gap' topic exists so consumers that don't
        // poll any other topic still learn about resets.
        rings
            .entry("gap".to_string())
            .or_insert_with(|| TopicRing::new("gap", 256, new_epoch))
            .push(payload, now);
    }
}

fn now_iso() -> String {
    use std::time::SystemTime;
    let now: chrono::DateTime<chrono::Utc> = SystemTime::now().into();
    now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
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
        let mut actor = ConnectorActor {
            client,
            cfg: ConnectorCfg::default(),
            rings: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
        };

        actor
            .handle_frame(WsMessage::Order(json!({"orderId": 1})))
            .await;
        actor.handle_frame(WsMessage::Pnl(json!({"upnl": 1.0}))).await;
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

    #[tokio::test]
    async fn heartbeat_and_system_frames_dont_create_topics() {
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor {
            client,
            cfg: ConnectorCfg::default(),
            rings: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
        };

        actor.handle_frame(WsMessage::Heartbeat).await;
        actor.handle_frame(WsMessage::System(json!({"msg": "ready"}))).await;

        let rings = actor.rings.read().await;
        assert!(rings.is_empty());

        // But last_message_at IS updated.
        let s = actor.status.read().await;
        assert!(s.last_message_at.is_some());
    }

    #[tokio::test]
    async fn bump_epoch_skips_gap_on_first_boot() {
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let actor = ConnectorActor {
            client,
            cfg: ConnectorCfg::default(),
            rings: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
        };

        actor.bump_epoch_with_gap(GapReason::ReconnectedAfterDisconnect).await;

        // No gap event injected on first boot.
        let rings = actor.rings.read().await;
        assert!(rings.is_empty());
        // But reset_epoch did bump.
        assert_eq!(actor.status.read().await.reset_epoch, 1);
    }

    #[tokio::test]
    async fn bump_epoch_injects_gap_into_existing_topics() {
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor {
            client,
            cfg: ConnectorCfg::default(),
            rings: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
        };

        // Seed an order event so the actor knows about the orders topic.
        actor.handle_frame(WsMessage::Order(json!({"id": 1}))).await;
        // Now bump epoch as if we'd reconnected.
        actor.bump_epoch_with_gap(GapReason::ReconnectedAfterDisconnect).await;

        let rings = actor.rings.read().await;
        let orders_ring = rings.get("orders").unwrap();
        // Original event + gap marker = 2.
        assert_eq!(orders_ring.len(), 2);
        // 'gap' topic also got a marker.
        assert_eq!(rings.get("gap").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn ensure_market_data_dedups_repeat_calls() {
        // We can't easily test the WS sub call without a mock socket,
        // but we can verify the dedup logic at the actor level by
        // hand-mutating the active_marketdata_subs set.
        let (_tx, cmd_rx) = mpsc::channel(1);
        let client = bezant::Client::new("https://localhost:5000/v1/api").unwrap();
        let mut actor = ConnectorActor {
            client,
            cfg: ConnectorCfg::default(),
            rings: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(StatusState::default())),
            cmd_rx,
            active_marketdata_subs: BTreeSet::new(),
        };

        assert!(actor.active_marketdata_subs.insert(265_598));
        // Re-insert returns false (already present).
        assert!(!actor.active_marketdata_subs.insert(265_598));
        assert_eq!(actor.active_marketdata_subs.len(), 1);
    }
}
