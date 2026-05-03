//! Error types for bezant.

use thiserror::Error;

/// Result alias using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Boxed error suitable for round-tripping heterogeneous upstream errors.
type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Errors that can arise when talking to the Client Portal Gateway.
///
/// The enum is `#[non_exhaustive]` — match on the variants you care about
/// and handle the rest via a catch-all so adding new variants in a point
/// release is not a breaking change.
///
/// # Retry hints
///
/// Use [`Error::is_retryable`] in retry loops rather than pattern-matching
/// every variant. Transient transport failures, upstream 5xx, and
/// `NoSession` are flagged retryable; caller-input errors and auth
/// failures are not.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The base URL passed to [`Client::new`][crate::Client::new] could not be parsed.
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),

    /// A URL we tried to manipulate isn't suitable as a base — either
    /// the scheme can't have a hierarchical path (`data:`, `mailto:`)
    /// or the URL has no host. Distinct from [`Error::InvalidBaseUrl`]
    /// because it surfaces *during* a call rather than at builder time.
    #[error("URL is not a base: {url}")]
    UrlNotABase {
        /// The URL whose base manipulation failed.
        url: String,
    },

    /// HTTP transport failure (DNS, connection, TLS, timeouts).
    #[error("http transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// CPGateway returned a non-2xx status. Carries the endpoint name,
    /// the upstream status code, and (where cheap) a short body preview.
    /// Replaces a swathe of older `Error::Other("upstream 500")`-style
    /// sites — callers can now branch on `.status` for retry logic.
    #[error("upstream {endpoint} returned {status}")]
    UpstreamStatus {
        /// The endpoint that returned the bad status, e.g. `iserver/auth/status`.
        endpoint: &'static str,
        /// The HTTP status code received.
        status: u16,
        /// First few hundred bytes of the response body, if cheap to capture.
        body_preview: Option<String>,
    },

    /// CPGateway returned a status code our typed client doesn't model.
    /// Useful canary for OpenAPI spec drift — when a previously-undocumented
    /// status appears in production, this surfaces the endpoint name so
    /// telemetry can flag it without losing every detail to a string.
    #[error("unknown response variant from {endpoint}")]
    Unknown {
        /// The endpoint whose response shape was unexpected.
        endpoint: &'static str,
    },

    /// A JSON response body could not be decoded into the expected type —
    /// typically a sign the Gateway sent an HTML error page on top of a
    /// 2xx status (Akamai error pages, maintenance banners, etc.).
    #[error("decode error from {endpoint} (status {status}): {message}")]
    Decode {
        /// The endpoint whose body failed to decode.
        endpoint: String,
        /// The HTTP status code on the response that failed to decode.
        status: u16,
        /// The underlying serde error, stringified.
        message: String,
    },

    /// Caller-provided input was malformed (bad query parameter, oversize
    /// body, unparseable URL, etc.). Distinct from [`Error::Other`] so
    /// HTTP wrappers can map it to 400 rather than 500.
    #[error("bad request: {0}")]
    BadRequest(String),

    /// A required query parameter was absent. Distinct from
    /// [`Error::BadRequest`] so HTTP wrappers can map to a more specific
    /// error code and so callers can hint the user about what's missing.
    #[error("missing query parameter: {name}")]
    MissingQuery {
        /// The query-parameter name that was expected but absent.
        name: &'static str,
    },

    /// HTTP header construction failed — the value contained bytes the
    /// `http` crate refuses to put on the wire (control chars, non-visible
    /// ASCII, CR/LF). Carries the header name + the underlying parse error.
    #[error("invalid {name} header value: {source}")]
    Header {
        /// The header name we tried to construct.
        name: &'static str,
        /// The underlying value-validation failure.
        #[source]
        source: reqwest::header::InvalidHeaderValue,
    },

    /// An error bubbled up from the generated API layer. The inner
    /// boxed error is whatever the generated client raised — this keeps
    /// `bezant-core`'s public API free of a versioned `anyhow` type.
    #[error("api error: {0}")]
    Api(#[source] DynError),

    /// Symbol → conid lookup returned no contracts. Distinct from
    /// [`Error::UpstreamStatus`] because the upstream returned a
    /// well-formed empty result, not a failure.
    #[error("no contracts for symbol '{symbol}'")]
    SymbolNotFound {
        /// The symbol that returned no contracts.
        symbol: String,
    },

    /// A contract IBKR returned for a symbol carried a conid that
    /// couldn't be parsed as `i32` — surfaces as a typed signal so
    /// callers don't have to string-match on `Error::Other`.
    #[error("invalid conid '{raw}' for symbol '{symbol}': {source}")]
    BadConid {
        /// The symbol whose contract carried a bad conid.
        symbol: String,
        /// The raw string IBKR returned where a number was expected.
        raw: String,
        /// The underlying parse error.
        #[source]
        source: std::num::ParseIntError,
    },

    /// WebSocket handshake (HTTP upgrade) failed. Carries the URL we
    /// were upgrading to plus the underlying tungstenite error so a
    /// caller can branch on TLS vs DNS vs auth failures.
    #[error("ws handshake to {url}: {source}")]
    WsHandshake {
        /// The WebSocket URL the handshake targeted.
        url: String,
        /// The underlying tungstenite error.
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },

    /// WebSocket transport (read/write/close) failed mid-stream.
    #[error("ws transport: {source}")]
    WsTransport {
        /// The underlying tungstenite error.
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },

    /// WebSocket protocol violation or bezant-side serialisation issue.
    #[error("ws protocol: {0}")]
    WsProtocol(String),

    /// Failed to construct an HTTP response — body assembly or header
    /// validation. Server-side internal; shouldn't be observable in
    /// normal operation.
    #[error("response build: {0}")]
    ResponseBuild(String),

    /// The Gateway reports we are not authenticated — the user needs to log
    /// in via the Gateway's browser UI.
    #[error("gateway is not authenticated — log in at the Gateway URL first")]
    NotAuthenticated,

    /// The Gateway is reachable but has no active session (e.g. was just
    /// booted, or session was invalidated by another login).
    #[error("gateway has no active session")]
    NoSession,

    /// A catch-all for unexpected conditions that don't fit other variants.
    /// Prefer adding a typed variant — this is the documented escape hatch
    /// of last resort, and is mapped to a generic 500 at the HTTP boundary.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Construct a free-form error.
    ///
    /// Prefer a typed variant — `Error::Other` maps to a generic 500 at
    /// the HTTP boundary and gives callers no programmatic handle.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }

    /// Hint for retry loops: returns `true` iff this error *might*
    /// succeed on retry. Use this in backoff loops instead of
    /// pattern-matching every variant by hand.
    ///
    /// Retryable:
    /// * Transport timeouts / connect failures / general request errors
    /// * Upstream 5xx and 429 (rate-limited)
    /// * `NoSession` (session may come back)
    /// * WebSocket transport failures (reconnect)
    ///
    /// Not retryable:
    /// * Caller-input errors (`BadRequest`, `MissingQuery`,
    ///   `InvalidBaseUrl`, `UrlNotABase`)
    /// * Auth failures (`NotAuthenticated`)
    /// * `SymbolNotFound`, `BadConid`, `Header` (data-shape problems)
    /// * `Decode` / `Api` / `Unknown` (semantically broken response)
    /// * `Other` (unknown — be conservative, don't retry)
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http(e) => e.is_timeout() || e.is_connect() || e.is_request(),
            Self::UpstreamStatus { status, .. } => *status >= 500 || *status == 429,
            Self::NoSession => true,
            Self::WsTransport { .. } | Self::WsHandshake { .. } => true,
            _ => false,
        }
    }
}

// `bezant-api`'s generated client returns `anyhow::Result<T>` from every
// typed call, so this `From` impl is load-bearing for `?` propagation
// in `auth.rs` / `helpers.rs`. Tracked for future cleanup: redrive
// helpers off the generated client's Result shape directly so anyhow
// can be made optional without breaking internal `?`.
impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Api(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time assertion that `Error` is `Send + Sync` so it can
    /// cross task boundaries. Regression guard against future variants
    /// accidentally embedding non-Send state.
    #[test]
    fn error_is_send_sync() {
        const fn assert<T: Send + Sync>() {}
        assert::<Error>();
    }

    #[test]
    fn is_retryable_flags_transient_errors() {
        assert!(Error::NoSession.is_retryable());
        assert!(Error::UpstreamStatus {
            endpoint: "x",
            status: 500,
            body_preview: None
        }
        .is_retryable());
        assert!(Error::UpstreamStatus {
            endpoint: "x",
            status: 503,
            body_preview: None
        }
        .is_retryable());
        assert!(Error::UpstreamStatus {
            endpoint: "x",
            status: 429,
            body_preview: None
        }
        .is_retryable());
    }

    #[test]
    fn is_retryable_rejects_caller_errors() {
        assert!(!Error::NotAuthenticated.is_retryable());
        assert!(!Error::BadRequest("bad".into()).is_retryable());
        assert!(!Error::MissingQuery { name: "x" }.is_retryable());
        assert!(!Error::InvalidBaseUrl("nope".into()).is_retryable());
        assert!(!Error::UrlNotABase {
            url: "data:".into()
        }
        .is_retryable());
        assert!(!Error::SymbolNotFound {
            symbol: "ZZZ".into()
        }
        .is_retryable());
        // 4xx upstream errors aren't retryable either — the request is
        // rejected for input reasons, not transient capacity.
        assert!(!Error::UpstreamStatus {
            endpoint: "x",
            status: 401,
            body_preview: None
        }
        .is_retryable());
        assert!(!Error::UpstreamStatus {
            endpoint: "x",
            status: 404,
            body_preview: None
        }
        .is_retryable());
        // Catch-all `Other` is conservatively non-retryable — we don't
        // know what we don't know.
        assert!(!Error::Other("mystery".into()).is_retryable());
    }

    #[test]
    fn display_includes_endpoint_and_status() {
        let e = Error::UpstreamStatus {
            endpoint: "iserver/auth/status",
            status: 503,
            body_preview: None,
        };
        let s = e.to_string();
        assert!(s.contains("iserver/auth/status"), "got: {s}");
        assert!(s.contains("503"), "got: {s}");
    }

    #[test]
    fn display_symbol_not_found_carries_symbol() {
        let e = Error::SymbolNotFound {
            symbol: "PLTRX".into(),
        };
        let s = e.to_string();
        assert!(s.contains("PLTRX"), "got: {s}");
    }
}
