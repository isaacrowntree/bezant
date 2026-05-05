//! Event capture + REST surface for streaming CPAPI topics.
//!
//! `bezant-server` runs an internal [`bezant::WsClient`] that subscribes
//! to order, PnL, and (lazily) market-data feeds. Decoded frames go into
//! per-topic ring buffers behind cursor-based REST endpoints
//! (`/events/{topic}?since=<cursor>`), so polling consumers can read every
//! event the socket has seen since their last visit — no events lost
//! between client polls, no need to keep a long-lived WS open from the
//! consumer side.
//!
//! See the architecture sketch:
//!
//! ```text
//! CPGateway WS  ─► bezant_core::WsClient  ─► Connector (tokio task)
//!                                              │
//!                                              ▼
//!                                       per-topic TopicRing
//!                                              │
//!                                              ▼
//!                                  GET /events/{topic}?since=<cursor>
//! ```

pub mod connector;
pub mod persistence;
pub mod ring;

use std::collections::BTreeMap;

use serde::Serialize;

pub use connector::{spawn_connector, ConnectorCfg, EventsHandle, TestSink};
pub use persistence::{EventLog, RetentionPolicy};
pub use ring::{ReadResult, TopicRing};

/// One captured event. Wire-shape returned by `/events/{topic}` endpoints.
#[derive(Clone, Debug, Serialize)]
pub struct ObservedEvent {
    /// Server-assigned monotonic cursor within `(topic, reset_epoch)`.
    /// Use it as the `since=` parameter on the next poll.
    pub cursor: u64,
    /// Topic name — `"orders"`, `"pnl"`, `"marketdata:265598"`, `"gap"`.
    pub topic: String,
    /// RFC 3339 timestamp at which the connector pushed this into the ring.
    pub received_at: String,
    /// Increments every time the underlying WS reconnects or the server
    /// restarts. Clients use it to detect "the cursor space reset under me".
    pub reset_epoch: u64,
    /// The decoded JSON frame. Shape depends on the topic — see the
    /// generated `bezant-client` TS types for the typed views.
    pub payload: serde_json::Value,
}

/// Snapshot of the connector's state, returned by `GET /events/_status`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct EventsStatus {
    /// `true` if the underlying WS is currently connected.
    pub connected: bool,
    /// RFC 3339 timestamp of the last frame the connector received, if any.
    pub last_message_at: Option<String>,
    /// How many times the connector has reconnected since the process
    /// started. Bumps on every successful reconnect, not on each retry.
    pub reconnect_count: u64,
    /// Wall-clock seconds since the connector task spawned.
    pub uptime_seconds: u64,
    /// Current `reset_epoch` — bumps on each reconnect or process restart.
    pub reset_epoch: u64,
    /// Topics currently subscribed at the upstream WS. Always includes
    /// `"orders"` and `"pnl"`; market data topics appear when a client
    /// has polled `/events/marketdata?conid=…` recently.
    pub topics_subscribed: Vec<String>,
    /// Per-topic ring buffer occupancy. Useful for "are we close to
    /// wraparound?" capacity planning.
    pub buffer_sizes: BTreeMap<String, usize>,
}

/// Reason a synthetic [`ObservedEvent`] of topic `"gap"` was injected.
/// Surfaced to the client so they know they missed something.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The connector reconnected — anything that happened while
    /// disconnected is permanently gone from the upstream feed.
    ReconnectedAfterDisconnect,
    /// Process restarted — ring buffers were freshly initialised.
    ProcessRestart,
}
