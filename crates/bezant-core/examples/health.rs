//! Minimal health probe against a running IBKR Client Portal Gateway.
//!
//! Run against the bundled Docker gateway:
//!
//! ```text
//! docker compose up -d
//! # log in once at https://localhost:5000
//! cargo run -p bezant-core --example health
//! ```

use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let gateway_url = std::env::var("IBKR_GATEWAY_URL")
        .unwrap_or_else(|_| "https://localhost:5000/v1/api".to_string());

    let client = bezant::Client::builder(&gateway_url)
        // The Gateway ships with a self-signed cert by default.
        .accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .build()?;

    match client.auth_status().await {
        Ok(status) => {
            println!(
                "gateway ok — authenticated={} connected={} competing={} message={:?}",
                status.authenticated, status.connected, status.competing, status.message
            );
        }
        Err(bezant::Error::NotAuthenticated) => {
            eprintln!("gateway reachable but no session — log in at {gateway_url}");
            std::process::exit(2);
        }
        Err(e) => return Err(e.into()),
    }
    Ok(())
}
