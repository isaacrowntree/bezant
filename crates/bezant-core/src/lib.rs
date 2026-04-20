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
//! For raw access to every one of the ~155 CPAPI endpoints, use the
//! underlying [`bezant-api`] crate directly. The two are fully interoperable
//! — [`Client::api`] hands you the generated client.
//!
//! ## Quickstart
//!
//! ```no_run
//! # async fn run() -> bezant::Result<()> {
//! use std::time::Duration;
//!
//! let client = bezant::Client::new("https://localhost:5000")?;
//! let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
//! client.health().await?;  // errors early if the user hasn't logged in
//!
//! // drop into the generated client for real work:
//! let accounts = client.api().get_accounts().await?;
//! # Ok(())
//! # }
//! ```
//!
//! [`bezant-api`]: https://docs.rs/bezant-api

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod auth;
mod client;
mod error;

pub use auth::{AuthStatus, KeepaliveHandle, TickleResponse};
pub use client::{Client, ClientBuilder, DEFAULT_BASE_URL};
pub use error::{Error, Result};

/// Re-export of the auto-generated API crate for callers that want raw access.
pub use bezant_api as api;
