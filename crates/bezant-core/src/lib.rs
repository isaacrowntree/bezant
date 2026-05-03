//! # bezant
//!
//! Ergonomic async client for the IBKR Client Portal Web API.
//!
//! Bezant wraps the auto-generated [`bezant-api`] crate with the sugar you
//! actually need to build an automated trading bot:
//!
//! - [`Client`] — a thin wrapper around the Gateway with saner TLS defaults
//!   (self-signed certs, timeout, user-agent, session cookies).
//! - [`Client::spawn_keepalive`] — a drop-to-stop background task that
//!   tickles `/tickle` so the 5-minute CPAPI session never expires.
//! - [`Client::health`] — one call that returns [`Error::NotAuthenticated`]
//!   or [`Error::NoSession`] instead of an opaque HTTP status.
//!
//! For raw access to every one of the ~154 CPAPI endpoints, use the
//! underlying [`bezant-api`] crate directly. The two are fully interoperable
//! — [`Client::api`] hands you the generated client.
//!
//! ## Quickstart
//!
//! ```no_run
//! # async fn run() -> bezant::Result<()> {
//! use std::time::Duration;
//!
//! let client = bezant::Client::new("https://localhost:5000/v1/api")?;
//! let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
//! client.health().await?;  // errors early if the user hasn't logged in
//!
//! // drop into the generated client for real work:
//! let _ = client
//!     .api()
//!     .get_all_accounts(bezant::api::GetAllAccountsRequest::default())
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! [`bezant-api`]: https://docs.rs/bezant-api

#![deny(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

mod auth;
mod client;
mod error;
mod helpers;
mod jar;
pub mod ws;

pub use auth::{AuthStatus, KeepaliveHandle, TickleResponse};
pub use client::{Client, ClientBuilder, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use helpers::{
    ContractSummary, Position, SymbolCache, MAX_POSITION_PAGES, POSITIONS_PAGE_SIZE,
};
pub use jar::NameKeyedJar;
pub use ws::{MarketDataFields, WsClient, WsMessage};

/// Re-export of the auto-generated API crate for callers that want raw access.
pub use bezant_api as api;

/// Re-export of [`url::Url`] so callers can name the return type of
/// [`Client::base_url`] without adding `url` to their own `Cargo.toml`.
pub use url::Url;

/// Re-export of [`reqwest::StatusCode`] — callers using
/// [`Client::http`] frequently need it and otherwise have to add
/// `reqwest` to their own `Cargo.toml` just to spell the type.
pub use reqwest::StatusCode;

/// Glob-importable prelude for the typical bot use case:
/// `Client`, `ClientBuilder`, `Result`, `Error`, `SymbolCache`,
/// `KeepaliveHandle`. Optimised for `use bezant::prelude::*;`.
///
/// ```no_run
/// use bezant::prelude::*;
///
/// # async fn run() -> Result<()> {
/// let client = Client::new("https://localhost:5000/v1/api")?;
/// let cache = SymbolCache::new(client);
/// let aapl = cache.conid_for("AAPL").await?;
/// println!("AAPL = {aapl}");
/// # Ok(())
/// # }
/// ```
pub mod prelude {
    pub use crate::{
        AuthStatus, Client, ClientBuilder, Error, KeepaliveHandle, Position, Result, SymbolCache,
        TickleResponse,
    };
}
