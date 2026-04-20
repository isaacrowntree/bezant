//! Error types for bezant.

use thiserror::Error;

/// Result alias using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can arise when talking to the Client Portal Gateway.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The base URL passed to [`Client::new`][crate::Client::new] could not be parsed.
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),

    /// HTTP transport failure (DNS, connection, TLS, timeouts).
    #[error("http transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// An error bubbled up from the generated API layer.
    #[error(transparent)]
    Api(#[from] anyhow::Error),

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
