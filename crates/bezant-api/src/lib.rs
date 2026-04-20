//! # bezant-api
//!
//! Auto-generated Rust client for the
//! [IBKR Client Portal Web API][cpapi]. Generated from the OpenAPI 3.1 spec
//! vendored in [`bezant-spec`] via [`oas3-gen`].
//!
//! You can consume this crate directly for raw, 1:1 access to every endpoint
//! and type. For a smaller, ergonomic facade — keepalive tasks, pagination
//! helpers, symbol caching — use the [`bezant`] crate on top.
//!
//! ## Regeneration
//!
//! Regenerate with `./scripts/codegen.sh` — it upgrades the vendored 3.0 spec
//! to 3.1, then runs `oas3-gen client-mod`.
//!
//! [cpapi]: https://www.interactivebrokers.com/campus/ibkr-api-page/cpapi-v1/
//! [`bezant-spec`]: https://docs.rs/bezant-spec
//! [`bezant`]: https://docs.rs/bezant
//! [`oas3-gen`]: https://github.com/eklipse2k8/oas3-gen

// Generated code — we don't hold it to our hand-written clippy standards.
// Every crate that consumes `bezant-api` has these allows applied transitively
// to silence the noise.
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::cargo)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod generated;

pub use generated::*;
