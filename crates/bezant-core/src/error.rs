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
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The base URL passed to [`Client::new`][crate::Client::new] could not be parsed.
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),

    /// HTTP transport failure (DNS, connection, TLS, timeouts).
    #[error("http transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// A JSON response body could not be decoded into the expected type —
    /// typically a sign the Gateway sent an HTML error page on top of a
    /// 2xx status (Akamai error pages, maintenance banners, etc.).
    #[error("response body decode error: {0}")]
    Decode(String),

    /// Caller-provided input was malformed (bad query parameter, oversize
    /// body, unparseable URL, etc.). Distinct from [`Error::Other`] so
    /// HTTP wrappers can map it to 400 rather than 500.
    #[error("bad request: {0}")]
    BadRequest(String),

    /// An error bubbled up from the generated API layer. The inner
    /// boxed error is whatever the generated client raised — this keeps
    /// `bezant-core`'s public API free of a versioned `anyhow` type.
    #[error("api error: {0}")]
    Api(#[source] DynError),

    /// The Gateway reports we are not authenticated — the user needs to log
    /// in via the Gateway's browser UI.
    #[error("gateway is not authenticated — log in at the Gateway URL first")]
    NotAuthenticated,

    /// The Gateway is reachable but has no active session (e.g. was just
    /// booted, or session was invalidated by another login).
    #[error("gateway has no active session")]
    NoSession,

    /// A catch-all for unexpected conditions that don't fit other variants.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Construct a free-form error.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}

// `anyhow::Error` keeps this conversion convenient for internal callers
// without leaking the anyhow type through the `Error` enum itself —
// downstream crates match on `Error::Api` and see a plain boxed error.
impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Api(e.into())
    }
}
