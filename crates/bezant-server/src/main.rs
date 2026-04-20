//! `bezant-server` — HTTP sidecar exposing the Bezant IBKR client over REST.
//!
//! Designed to run alongside the IBKR Client Portal Gateway (in the same
//! pod / docker-compose stack). Your application — in any language — talks
//! to bezant-server over plain HTTP/JSON instead of wrangling reqwest +
//! cookies + the CPAPI quirks directly.
//!
//! ```text
//! ┌──────────────┐  HTTP/JSON  ┌────────────────┐  HTTPS+cookie  ┌─────────┐
//! │  your bot    │───────────► │ bezant-server  │───────────────►│ IBKR    │
//! │ (any lang)   │             │  (this binary) │                │ Gateway │
//! └──────────────┘             └────────────────┘                └─────────┘
//! ```
//!
//! ## Endpoints
//!
//! | Method | Path                                  | What it returns                  |
//! |--------|---------------------------------------|----------------------------------|
//! | GET    | `/health`                             | Gateway auth + session status    |
//! | GET    | `/accounts`                           | Account list (`GET /portfolio/accounts`) |
//! | GET    | `/accounts/:account_id/summary`       | Account summary                  |
//! | GET    | `/accounts/:account_id/positions`     | Current positions (paginated)    |
//!
//! More endpoints will come in v0.2. Raw CPAPI access is always available
//! via the library crate for apps willing to link Rust.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing::info;

mod error;
mod routes;
mod state;

/// CLI for running the Bezant HTTP sidecar.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Address to bind the HTTP server on.
    #[arg(long, env = "BEZANT_BIND", default_value = "0.0.0.0:8080")]
    bind: SocketAddr,

    /// Base URL of the IBKR Client Portal Gateway.
    #[arg(long, env = "IBKR_GATEWAY_URL", default_value = bezant::DEFAULT_BASE_URL)]
    gateway_url: String,

    /// How often to send a `/tickle` to the Gateway to keep the session alive.
    #[arg(long, env = "BEZANT_KEEPALIVE_SECS", default_value_t = 60)]
    keepalive_secs: u64,

    /// Reject self-signed TLS certs when talking to the Gateway. Defaults to
    /// allowing them (the Gateway ships with a self-signed cert by default).
    #[arg(long, env = "BEZANT_REJECT_INVALID_CERTS")]
    reject_invalid_certs: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bezant_server=info,bezant=info,tower_http=info".into()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();

    info!(
        bind = %args.bind,
        gateway = %args.gateway_url,
        keepalive = args.keepalive_secs,
        "bezant-server starting"
    );

    let client = bezant::Client::builder(&args.gateway_url)
        .accept_invalid_certs(!args.reject_invalid_certs)
        .build()
        .context("building bezant client")?;

    // Keepalive runs for the lifetime of the server.
    let _keepalive = client.spawn_keepalive(Duration::from_secs(args.keepalive_secs));

    let state = state::AppState::new(client);
    let app = routes::router(state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .with_context(|| format!("binding {}", args.bind))?;
    info!(addr = %listener.local_addr()?, "bezant-server listening");

    axum::serve(listener, app.into_make_service())
        .await
        .context("server crashed")?;

    Ok(())
}
