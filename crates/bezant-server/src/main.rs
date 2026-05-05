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
//! | Method        | Path                                       | What it returns                                  |
//! |---------------|--------------------------------------------|--------------------------------------------------|
//! | GET           | `/health`                                  | Gateway auth + session status (typed)            |
//! | GET           | `/accounts`                                | Account list (`GET /portfolio/accounts`)         |
//! | GET           | `/accounts/{id}/summary`                   | Account summary                                  |
//! | GET           | `/accounts/{id}/positions?page=N`          | Current positions (paginated)                    |
//! | GET           | `/accounts/{id}/ledger`                    | Cash ledger by currency                          |
//! | GET / POST    | `/accounts/{id}/orders`                    | List live orders / submit new orders             |
//! | DELETE        | `/accounts/{id}/orders/{order_id}`         | Cancel a live order                              |
//! | GET           | `/contracts/search?symbol=…&secType=STK`   | Symbol → conid resolution                        |
//! | GET           | `/market/snapshot?conids=…&fields=…`       | Real-time market data snapshot                   |
//! | GET           | `/debug/jar` (token-gated)                 | Cookie names + lengths in the shared jar         |
//! | GET           | `/debug/probe` (token-gated)               | Walks `auth/status` → `accounts` for diagnosis   |
//! | *fallback*    | any other path                             | Verbatim passthrough to the Gateway (with cookie + Origin/Referer fixups). Drives the interactive `/sso/Login` flow. |
//!
//! Debug endpoints are off by default — enable with `BEZANT_DEBUG_TOKEN`
//! and authenticate via `?token=…` query or `X-Bezant-Debug-Token` header.
//! Raw CPAPI access is always available via the library crate for apps
//! willing to link Rust.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use axum::http::{HeaderName, Request, StatusCode};
use clap::Parser;
use tokio::signal;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, info_span, warn, Level};

use bezant_server::{router, spawn_connector, AppState, ConnectorCfg};

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

    /// Verify the Gateway's TLS certificate. Defaults to **off** because
    /// the Gateway ships with a self-signed cert; set
    /// `BEZANT_VERIFY_TLS=true` once you've installed a trusted cert.
    ///
    /// `false`/unset → accept any cert (suitable for local Docker setups).
    /// `true` → reject self-signed and otherwise-invalid certs.
    ///
    /// Replaces the old `--reject-invalid-certs` flag, whose double
    /// negative made it easy to accidentally leave invalid-cert
    /// acceptance on in production.
    #[arg(long, env = "BEZANT_VERIFY_TLS", default_value_t = false)]
    verify_tls: bool,

    /// Enable the diagnostic `/debug/*` endpoints, gated by this token.
    /// Callers must present the same token via `?token=…` query string
    /// or the `X-Bezant-Debug-Token` header on every `/debug/*` call.
    /// Without this flag, the endpoints return 404. The cookie jar
    /// holds live IBKR session cookies — pick a long random token
    /// (>=32 bytes from `/dev/urandom`) and treat it like a
    /// credential.
    #[arg(long, env = "BEZANT_DEBUG_TOKEN")]
    debug_token: Option<String>,

    /// Run an internal WebSocket consumer that captures order/PnL/market
    /// data events into ring buffers exposed via `/events/*`. When off,
    /// `/events/*` routes return 503. Default off so existing deploys
    /// behave unchanged; enable explicitly via `BEZANT_EVENTS_ENABLED=1`.
    #[arg(long, env = "BEZANT_EVENTS_ENABLED", default_value_t = false)]
    events_enabled: bool,

    /// Capacity of the orders event ring (P0 default 1000).
    #[arg(long, env = "BEZANT_EVENTS_ORDERS_CAP", default_value_t = 1_000)]
    events_orders_cap: usize,

    /// Capacity of the PnL event ring (P0 default 5000).
    #[arg(long, env = "BEZANT_EVENTS_PNL_CAP", default_value_t = 5_000)]
    events_pnl_cap: usize,

    /// Per-conid capacity of the market-data event ring.
    #[arg(long, env = "BEZANT_EVENTS_MARKETDATA_CAP", default_value_t = 2_000)]
    events_marketdata_cap: usize,

    /// If set, persist captured events to a sqlite database at this
    /// path. The `/events/{topic}/history` route serves reads from
    /// this store (in addition to the in-memory ring buffers).
    /// Without this, history is bounded by ring capacity only.
    #[arg(long, env = "BEZANT_EVENTS_DB_PATH")]
    events_db_path: Option<String>,
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
        .accept_invalid_certs(!args.verify_tls)
        // bezant-server acts as a reverse proxy: 3xx responses from the
        // Gateway must reach the browser as 3xx so it can follow the
        // Location header itself — otherwise redirected HTML arrives at
        // the original URL and relative asset paths break.
        .follow_redirects(false)
        .build()
        .context("building bezant client")?;

    // Keepalive runs for the lifetime of the server. Holding the
    // handle (rather than `let _ = …`) so we can `.stop().await` it
    // cleanly during graceful shutdown.
    let keepalive = client.spawn_keepalive(Duration::from_secs(args.keepalive_secs));

    let state_base = match args.debug_token {
        Some(token) => {
            info!("debug endpoints enabled (token gating active)");
            AppState::with_debug_token(client.clone(), token)
        }
        None => AppState::new(client.clone()),
    };

    let state = if args.events_enabled {
        let event_log = match &args.events_db_path {
            Some(path) => {
                info!(path = %path, "opening events sqlite log");
                let log = bezant_server::events::EventLog::open(path)
                    .context("opening events sqlite db")?;
                Some(std::sync::Arc::new(log))
            }
            None => {
                info!("BEZANT_EVENTS_DB_PATH not set — /events/{{topic}}/history will return 503");
                None
            }
        };

        info!(
            orders_cap = args.events_orders_cap,
            pnl_cap = args.events_pnl_cap,
            marketdata_cap = args.events_marketdata_cap,
            persistence = event_log.is_some(),
            "events capture enabled (/events/* routes active)"
        );
        let cfg = ConnectorCfg {
            orders_capacity: args.events_orders_cap,
            pnl_capacity: args.events_pnl_cap,
            marketdata_capacity: args.events_marketdata_cap,
            event_log,
            ..ConnectorCfg::default()
        };
        let handle = spawn_connector(client, cfg);
        state_base.with_events(handle)
    } else {
        info!("events capture disabled (/events/* routes will return 503)");
        state_base
    };
    // Build the production middleware stack here so the integration tests
    // (which call `router(state)` directly) get the same proxy semantics
    // and the layers stay in lockstep with the binary.
    //
    // * `TimeoutLayer(35s)` bounds inbound request lifetime — slightly
    //   above reqwest's 30s upstream timeout so an upstream call that
    //   times out surfaces as a typed error, not a silent layer kill.
    // * `RequestBodyLimitLayer(10MiB)` is the declarative replacement for
    //   the inline cap in `passthrough_any`; also applies to JSON-extractor
    //   handlers that otherwise inherit axum's default 2 MiB only.
    // * `TraceLayer` uses a custom `MakeSpan` that records *path* not
    //   *uri*: the URI carries `?token=…` for `/debug/*` calls and we
    //   don't want it in span fields / log shippers.
    let trace = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            // Path-only — never the query string (which can carry the
            // debug token). request_id is always present (the
            // SetRequestIdLayer below mints one if the caller didn't
            // supply x-request-id) so child spans / log lines correlate.
            let request_id = req
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("-");
            info_span!(
                "http",
                method = %req.method(),
                path = %req.uri().path(),
                request_id = %request_id,
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(DefaultOnResponse::new().level(Level::DEBUG));

    // x-request-id propagation: SetRequestIdLayer mints a UUID per
    // request if the caller didn't supply x-request-id; PropagateRequestIdLayer
    // copies the header to the response so downstream loadbalancers /
    // observability tools can stitch the trace together.
    //
    // ConcurrencyLimitLayer caps simultaneous handlers so a misbehaving
    // caller can't exhaust upstream connections (and IBKR's per-account
    // rate limits — getting locked out by hammering them is worse than
    // back-pressuring the caller). 256 is generous for a typical bot
    // workload; tune via fork if you need more.
    let req_id_header = HeaderName::from_static("x-request-id");
    let app = router(state)
        .layer(PropagateRequestIdLayer::new(req_id_header.clone()))
        .layer(trace)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::GATEWAY_TIMEOUT,
            Duration::from_secs(35),
        ))
        .layer(ConcurrencyLimitLayer::new(256))
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        // SetRequestIdLayer is applied last so it runs first on the
        // request side — every other layer sees the header set.
        .layer(SetRequestIdLayer::new(req_id_header, MakeRequestUuid));

    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .with_context(|| format!("binding {}", args.bind))?;
    info!(addr = %listener.local_addr()?, "bezant-server listening");

    // Graceful shutdown: SIGTERM (k8s pod stop, Railway redeploy) +
    // SIGINT (operator Ctrl-C) trigger axum's drain. Then we
    // explicitly await `keepalive.stop()` so the tickle task closes
    // its connection cleanly instead of being killed mid-request.
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server crashed")?;

    info!("server drained; stopping keepalive");
    if let Err(e) = keepalive.stop().await {
        warn!(error = %e, "keepalive stop returned error");
    }
    info!("bezant-server shutdown complete");

    Ok(())
}

/// Wait for SIGTERM (any platform) or Ctrl-C (any platform), whichever
/// fires first. On non-unix targets (Windows) only Ctrl-C is wired,
/// because SIGTERM doesn't exist there.
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = signal::ctrl_c().await {
            warn!(error = %e, "ctrl-c signal handler failed to install");
            // Park forever rather than tight-looping on a broken signal source.
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(e) => {
                warn!(error = %e, "SIGTERM handler failed to install");
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("shutdown: SIGINT received"),
        _ = terminate => info!("shutdown: SIGTERM received"),
    }
}
