//! `bezant-mcp` — Model Context Protocol server for the IBKR Client
//! Portal Web API. See the crate-level docs at `lib.rs` for design notes.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use bezant_mcp::BezantMcp;
use clap::Parser;
use rmcp::{transport::stdio, ServiceExt};
use tracing::info;

/// CLI for `bezant-mcp`.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Base URL of the IBKR Client Portal Gateway.
    #[arg(
        long,
        env = "IBKR_GATEWAY_URL",
        default_value = bezant::DEFAULT_BASE_URL
    )]
    gateway_url: String,

    /// Enforce TLS cert validation. Off by default — the Gateway ships
    /// a self-signed cert.
    #[arg(long, env = "BEZANT_REJECT_INVALID_CERTS")]
    reject_invalid_certs: bool,

    /// Session keepalive interval in seconds.
    #[arg(long, env = "BEZANT_MCP_KEEPALIVE_SECS", default_value_t = 60)]
    keepalive_secs: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bezant_mcp=info,bezant=info".into()),
        )
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let args = Args::parse();
    info!(
        gateway = %args.gateway_url,
        keepalive = args.keepalive_secs,
        "bezant-mcp starting"
    );

    let client = bezant::Client::builder(&args.gateway_url)
        .accept_invalid_certs(!args.reject_invalid_certs)
        .build()
        .context("building bezant client")?;

    let _keepalive = client.spawn_keepalive(Duration::from_secs(args.keepalive_secs));

    let cache = Arc::new(bezant::SymbolCache::new(client.clone()));
    let handler = BezantMcp::new(client, cache);

    let service = handler
        .serve(stdio())
        .await
        .context("starting MCP service")?;
    service.waiting().await.context("MCP service crashed")?;
    Ok(())
}
