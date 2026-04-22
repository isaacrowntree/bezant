//! WebSocket streaming client for the IBKR Client Portal Web API.
//!
//! The CPAPI exposes a WebSocket endpoint at `/v1/api/ws` that multiplexes
//! real-time market data, order updates, PnL snapshots, and more. Bezant
//! wraps the raw socket with:
//!
//! - A single-call [`WsClient::connect`] that derives the WS URL from a
//!   REST [`crate::Client`], reuses its session cookie, and returns a
//!   duplex handle.
//! - Typed subscribe/unsubscribe helpers for the common topics
//!   ([`WsClient::subscribe_market_data`],
//!   [`WsClient::subscribe_orders`], [`WsClient::subscribe_pnl`]).
//! - A [`WsClient::raw_stream`] escape hatch returning every decoded JSON
//!   message so you can handle message types we haven't modelled yet.
//!
//! # Topic format
//!
//! CPAPI's wire format is `TOPIC+{json}`. The first letter selects the
//! action: `s` subscribe, `u` unsubscribe. Examples:
//!
//! ```text
//! smd+265598+{"fields":["31","84","86"]}   // subscribe to AAPL L1 quote
//! umd+265598+{}                             // unsubscribe
//! sor+{}                                    // subscribe to order updates
//! spl+{}                                    // subscribe to PnL updates
//! ```
//!
//! See [IBKR's WebSocket lesson][ibkr-ws] for the full catalogue.
//!
//! [ibkr-ws]: https://www.interactivebrokers.com/campus/trading-lessons/websockets/

use std::time::Duration;

use futures_util::{Sink, SinkExt, Stream, StreamExt};
use serde::Serialize;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, warn};
use url::Url;

use crate::client::Client;
use crate::error::{Error, Result};

/// Raw WebSocket stream type the client multiplexes.
pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// A live Bezant WebSocket connection. Clone cheaply to share across tasks —
/// actually no, WebSocket sinks aren't cheap to split arbitrarily. Keep one
/// owner per connection and [`WsClient::split`] if you need a read/write
/// halving.
#[derive(Debug)]
pub struct WsClient {
    stream: WsStream,
}

/// A decoded CPAPI frame. Most messages fall into one of the variants below,
/// but the CPAPI occasionally emits payloads we haven't modelled — those end
/// up in [`WsMessage::Other`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WsMessage {
    /// Heartbeat pings sent by the server periodically.
    Heartbeat,
    /// System / session status messages (e.g. `"topic": "system"`).
    System(Value),
    /// Market data tick for a subscribed contract.
    MarketData {
        /// The contract id this tick is for.
        conid: i64,
        /// The full decoded payload (field codes are string-keyed).
        payload: Value,
    },
    /// Order update (working → filled, cancellations, etc).
    Order(Value),
    /// PnL / account summary update.
    Pnl(Value),
    /// Any message whose `topic` we didn't recognise.
    Other(Value),
    /// The socket emitted a frame we couldn't decode — a text body that
    /// wasn't valid JSON, or a binary frame converted lossily to UTF-8.
    /// The decoder's error is captured alongside the original text so
    /// callers can telemeter parse rates.
    Malformed {
        /// The raw (possibly lossy) text the socket delivered.
        text: String,
        /// Human-readable reason the decoder gave up — serde JSON error
        /// for malformed payloads, a "non-UTF-8 binary frame" marker for
        /// binary decoder losses.
        error: String,
    },
}

/// Market-data field codes used when subscribing. See
/// [`bezant_api::GetMdSnapshotRequestQuery`] for the documented set on the
/// REST side — every code listed there works on the WebSocket too.
///
/// Kept as an opaque newtype so we can change the internal representation
/// in a point release without breaking downstream callers.
#[derive(Debug, Clone)]
pub struct MarketDataFields(Vec<String>);

impl MarketDataFields {
    /// Reasonable default: last price, bid, ask, last size, bid size, ask size.
    #[must_use]
    pub fn default_l1() -> Self {
        Self::from_codes(["31", "84", "86", "85", "88", "87"])
    }

    /// Build a new [`MarketDataFields`] from any iterator of code strings.
    pub fn from_codes<I, S>(codes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self(codes.into_iter().map(Into::into).collect())
    }

    /// Borrow the underlying field codes — handy when forwarding the same
    /// set to multiple subscribes, or serialising for logging.
    #[must_use]
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl<S> FromIterator<S> for MarketDataFields
where
    S: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        Self::from_codes(iter)
    }
}

impl WsClient {
    /// Open a WebSocket connection to the Gateway that `client` is pointed at.
    ///
    /// Internally:
    /// 1. Issues a `/tickle` HTTP call to mint a session cookie.
    /// 2. Derives the `wss://…/ws` URL from the REST base URL.
    /// 3. Attaches `Cookie: api={"session":"…"}` to the WS handshake.
    /// 4. Returns a connected [`WsClient`].
    ///
    /// # Errors
    /// Any tickle / handshake / TLS failure surfaces as [`Error`].
    #[tracing::instrument(skip(client), level = "debug")]
    pub async fn connect(client: &Client) -> Result<Self> {
        let tickle = client.tickle().await?;
        let session = tickle
            .session
            .ok_or_else(|| Error::other("tickle returned no session id"))?;
        let ws_url = ws_url_from_base(client.base_url())?;
        let cookie = format!(r#"api={{"session":"{session}"}}"#);

        debug!(%ws_url, "bezant: opening websocket");
        let mut request = ws_url
            .as_str()
            .into_client_request()
            .map_err(|e| Error::other(format!("ws request build: {e}")))?;
        request.headers_mut().insert(
            "Cookie",
            cookie
                .parse()
                .map_err(|e| Error::other(format!("cookie header: {e}")))?,
        );
        request.headers_mut().insert(
            "User-Agent",
            format!("bezant/{}", env!("CARGO_PKG_VERSION"))
                .parse()
                .map_err(|e| Error::other(format!("user-agent header: {e}")))?,
        );

        let (stream, _) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|e| Error::other(format!("ws connect: {e}")))?;

        Ok(Self { stream })
    }

    /// Subscribe to level-1 market data for a single contract id. Use
    /// [`MarketDataFields::default_l1`] if you just want the common fields.
    ///
    /// # Errors
    /// Any send failure surfaces as [`Error::other`].
    #[tracing::instrument(skip(self, fields), fields(conid = conid), level = "debug")]
    pub async fn subscribe_market_data(
        &mut self,
        conid: i64,
        fields: &MarketDataFields,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            fields: &'a [String],
        }
        let body = Body {
            fields: fields.as_slice(),
        };
        let payload = format!(
            "smd+{conid}+{}",
            serde_json::to_string(&body)
                .map_err(|e| Error::other(format!("serialise fields: {e}")))?
        );
        self.send_text(payload).await
    }

    /// Unsubscribe from a previously-subscribed market data feed.
    ///
    /// # Errors
    /// Any send failure surfaces as [`Error::other`].
    pub async fn unsubscribe_market_data(&mut self, conid: i64) -> Result<()> {
        self.send_text(format!("umd+{conid}+{{}}")).await
    }

    /// Subscribe to order status updates.
    ///
    /// # Errors
    /// Any send failure surfaces as [`Error::other`].
    pub async fn subscribe_orders(&mut self) -> Result<()> {
        self.send_text("sor+{}".into()).await
    }

    /// Subscribe to PnL updates.
    ///
    /// # Errors
    /// Any send failure surfaces as [`Error::other`].
    pub async fn subscribe_pnl(&mut self) -> Result<()> {
        self.send_text("spl+{}".into()).await
    }

    /// Send a raw text frame. Useful for subscribing to topics Bezant doesn't
    /// yet model — follow the `topic+{json}` format.
    ///
    /// # Errors
    /// Any send failure surfaces as [`Error::other`].
    pub async fn send_text(&mut self, payload: String) -> Result<()> {
        self.stream
            .send(Message::text(payload))
            .await
            .map_err(|e| Error::other(format!("ws send: {e}")))
    }

    /// Pull the next decoded message. `None` means the socket closed.
    ///
    /// # Errors
    /// Any read failure surfaces as [`Error::other`].
    pub async fn next_message(&mut self) -> Result<Option<WsMessage>> {
        while let Some(raw) = self.stream.next().await {
            let frame = raw.map_err(|e| Error::other(format!("ws recv: {e}")))?;
            match frame {
                Message::Text(text) => return Ok(Some(classify(text.as_str()))),
                Message::Binary(bytes) => {
                    // CPAPI occasionally sends binary frames for heartbeats.
                    // Convert lossily — invalid UTF-8 becomes U+FFFD, which
                    // will either parse (empty `{}` survives) or be reported
                    // as [`WsMessage::Malformed`] with the JSON error. We
                    // deliberately don't surface a separate "BinaryLost"
                    // variant: the underlying socket is documented as text
                    // JSON and any non-UTF-8 payload is upstream weirdness.
                    let s = String::from_utf8_lossy(&bytes).to_string();
                    return Ok(Some(classify(&s)));
                }
                Message::Ping(data) => {
                    // Be a well-behaved client: echo the ping.
                    if let Err(e) = self.stream.send(Message::Pong(data)).await {
                        warn!(error = %e, "bezant: pong send failed");
                    }
                }
                Message::Pong(_) => {}
                Message::Frame(_) => {}
                Message::Close(_) => return Ok(None),
            }
        }
        Ok(None)
    }

    /// Return a [`Stream`] of [`WsMessage`]s that yields until the socket
    /// closes. Consuming this yields exclusive access to the reader; use
    /// [`WsClient::next_message`] on the client itself if you also need to
    /// send frames on the same task.
    pub fn raw_stream(self) -> impl Stream<Item = Result<WsMessage>> + Unpin {
        Box::pin(futures_util::stream::unfold(
            self.stream,
            |mut s| async move {
                loop {
                    match s.next().await {
                        None => return None,
                        Some(Err(e)) => {
                            return Some((Err(Error::other(format!("ws recv: {e}"))), s))
                        }
                        Some(Ok(Message::Text(t))) => {
                            return Some((Ok(classify(t.as_str())), s));
                        }
                        Some(Ok(Message::Binary(b))) => {
                            let text = String::from_utf8_lossy(&b).to_string();
                            return Some((Ok(classify(&text)), s));
                        }
                        Some(Ok(Message::Ping(p))) => {
                            let _ = s.send(Message::Pong(p)).await;
                        }
                        Some(Ok(Message::Close(_))) => return None,
                        Some(Ok(_)) => {}
                    }
                }
            },
        ))
    }

    /// Split the client into independent sink + stream halves so one task can
    /// send and another can receive concurrently.
    ///
    /// Returned types wrap tokio-tungstenite's halves directly — callers who
    /// want advanced control (close frames, custom flushing) can use the
    /// underlying types from that crate.
    pub fn split(
        self,
    ) -> (
        impl Sink<Message, Error = tokio_tungstenite::tungstenite::Error>,
        impl Stream<Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
    ) {
        self.stream.split()
    }

    /// How long to wait between application-level pings if you implement a
    /// ticker task on top. Chosen to match CPAPI's 5-minute session timeout
    /// with a safety margin.
    #[must_use]
    pub const fn recommended_keepalive() -> Duration {
        Duration::from_secs(60)
    }
}

/// Derive the WebSocket URL from a REST base URL.
///
/// `https://host:port/v1/api`       →  `wss://host:port/v1/api/ws`
/// `http://host:port/v1/api`        →  `ws://host:port/v1/api/ws`
fn ws_url_from_base(base: &Url) -> Result<Url> {
    let mut ws = base.clone();
    match ws.scheme() {
        "https" => ws
            .set_scheme("wss")
            .map_err(|()| Error::other("failed to set wss scheme"))?,
        "http" => ws
            .set_scheme("ws")
            .map_err(|()| Error::other("failed to set ws scheme"))?,
        s => return Err(Error::other(format!("unsupported base scheme '{s}'"))),
    }
    {
        let mut segs = ws
            .path_segments_mut()
            .map_err(|()| Error::other("url is not a base"))?;
        segs.push("ws");
    }
    Ok(ws)
}

/// Decode a text frame into a typed [`WsMessage`].
fn classify(text: &str) -> WsMessage {
    if text == "{}" || text.is_empty() {
        return WsMessage::Heartbeat;
    }
    let value: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            return WsMessage::Malformed {
                text: text.to_owned(),
                error: e.to_string(),
            }
        }
    };

    let topic = value
        .get("topic")
        .and_then(Value::as_str)
        .unwrap_or_default();

    // Market data topics are `smd+<conid>` (subscribe ack / tick).
    if let Some(rest) = topic.strip_prefix("smd+") {
        if let Ok(conid) = rest.parse::<i64>() {
            return WsMessage::MarketData {
                conid,
                payload: value,
            };
        }
    }

    match topic {
        "system" => WsMessage::System(value),
        "sor" | "ortd" | "ord" => WsMessage::Order(value),
        "spl" | "pnl" | "ssd" | "ssl" => WsMessage::Pnl(value),
        _ => WsMessage::Other(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_url_flips_https_to_wss_and_appends_ws() {
        let base = Url::parse("https://localhost:5000/v1/api").unwrap();
        let ws = ws_url_from_base(&base).unwrap();
        assert_eq!(ws.as_str(), "wss://localhost:5000/v1/api/ws");
    }

    #[test]
    fn ws_url_flips_http_to_ws() {
        let base = Url::parse("http://localhost:8080/v1/api").unwrap();
        let ws = ws_url_from_base(&base).unwrap();
        assert_eq!(ws.as_str(), "ws://localhost:8080/v1/api/ws");
    }

    #[test]
    fn classify_identifies_market_data_by_topic() {
        let raw = r#"{"topic":"smd+265598","31":"150.25","_updated":1700000000}"#;
        match classify(raw) {
            WsMessage::MarketData { conid, .. } => assert_eq!(conid, 265_598),
            other => panic!("expected MarketData, got {other:?}"),
        }
    }

    #[test]
    fn classify_empty_brace_is_heartbeat() {
        assert!(matches!(classify("{}"), WsMessage::Heartbeat));
    }

    #[test]
    fn classify_system_topic() {
        let raw = r#"{"topic":"system","msg":"ready"}"#;
        assert!(matches!(classify(raw), WsMessage::System(_)));
    }

    #[test]
    fn classify_malformed_text() {
        assert!(matches!(
            classify("not-json"),
            WsMessage::Malformed { .. }
        ));
    }
}
