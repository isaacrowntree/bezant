//! Library half of `bezant-server`.
//!
//! The binary in `main.rs` is a thin shell around the reusable pieces here:
//! [`router`] builds an [`axum::Router`] wired to the CPAPI pass-through
//! handlers, and [`AppState`] is the shared bag of state every handler
//! needs. Integration tests build the same router against wiremock
//! instead of a real gateway.

#![deny(missing_docs)]

mod error;
pub mod events;
mod routes;
mod state;

pub use error::{AppError, ErrorBody};
pub use events::{spawn_connector, ConnectorCfg, EventsHandle};
pub use routes::router;
pub use state::AppState;
