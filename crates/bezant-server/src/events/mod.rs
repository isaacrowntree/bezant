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
    /// Server-assigned cursor, strictly increasing per topic for the life
    /// of the process and seeded from the boot time so a restart does not
    /// rewind it. Always below 2^53. Use it as the `since=` parameter on the
    /// next poll.
    pub cursor: u64,
    /// Topic name — `"orders"`, `"pnl"`, `"marketdata:265598"`, `"gap"`.
    pub topic: String,
    /// RFC 3339 timestamp at which the connector pushed this into the ring.
    pub received_at: String,
    /// The epoch this event was captured under. Seeded from the boot time
    /// (Unix ms) and incremented on every reconnect, so it changes on both a
    /// reconnect and a restart. Clients use it to detect a gap.
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
    /// Current `reset_epoch`. Seeded from the boot time (Unix ms) and
    /// bumped once per successful reconnect — never on a failed attempt.
    pub reset_epoch: u64,
    /// Topics currently subscribed at the upstream WS. Always includes
    /// `"orders"` and `"pnl"`; market data topics appear when a client
    /// has polled `/events/marketdata?conid=…` recently.
    pub topics_subscribed: Vec<String>,
    /// Per-topic ring buffer occupancy. Useful for "are we close to
    /// wraparound?" capacity planning.
    pub buffer_sizes: BTreeMap<String, usize>,
    /// Whether CPAPI has actually honoured each standing subscription
    /// (`orders`, `pnl`). `topics_subscribed` above records what we ASKED
    /// for; this records what we GOT. The difference is the whole story of
    /// a fill that never arrived: CPAPI answers `sor+{}` with
    /// `{"error":"unable to subscribe"}` more often than not, and a socket
    /// that outlives a Gateway re-login keeps heartbeating with every
    /// subscription dead. A consumer confirming fills should treat anything
    /// but `subscribed` on `orders` as "the stream will not tell you".
    pub subscriptions: BTreeMap<String, SubscriptionState>,
    /// How many subscribe refusals CPAPI has sent since the process started.
    pub subscribe_refusals: u64,
    /// How many times the connector tore its socket down because the
    /// Gateway's session id changed underneath it (a re-login).
    pub session_rollovers: u64,
}

/// Where a standing subscription (`orders`, `pnl`) stands at the upstream WS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionState {
    /// Subscribe sent; CPAPI has neither honoured nor refused it yet.
    Pending,
    /// CPAPI has sent at least one real frame on the topic this connection.
    Subscribed,
    /// CPAPI answered the subscribe with an error; the connector is retrying.
    Refused,
    /// Asked repeatedly (`BEZANT_EVENTS_QUIET_AFTER_ROUNDS`) with neither a
    /// frame nor a refusal. CPAPI honours `sor+{}` in silence when there are
    /// no live orders, so this is subscribed-as-far-as-anyone-can-tell: the
    /// connector stops re-asking. A refusal puts it back to `refused`; the
    /// first real frame makes it `subscribed`.
    Quiet,
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
