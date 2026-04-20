//! `bezant` — command-line tool for the IBKR Client Portal Web API.
//!
//! Wraps [`bezant::Client`] with a clap-powered CLI so ops / shell users can
//! poke the Gateway without pulling in the Rust crate as a dependency.
//! Output is JSON by default; pass `--pretty` for indented JSON.
//!
//! Every subcommand prints JSON to stdout on success and a non-zero exit on
//! failure. Errors are printed to stderr as a one-line `bezant: <message>`.
//!
//! ## Environment
//!
//! | Env var                     | Default                                | Meaning                                    |
//! |-----------------------------|----------------------------------------|--------------------------------------------|
//! | `IBKR_GATEWAY_URL`          | `https://localhost:5000/v1/api`        | Base URL of the Client Portal Gateway      |
//! | `BEZANT_REJECT_INVALID_CERTS` | unset (certs are accepted by default) | Set to `1`/`true` to enforce cert validation |

use std::process::ExitCode;

use bezant::Client;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Base URL of the Gateway.
    #[arg(
        long,
        env = "IBKR_GATEWAY_URL",
        default_value = bezant::DEFAULT_BASE_URL,
        global = true
    )]
    gateway_url: String,

    /// Pretty-print JSON output (default: compact).
    #[arg(long, global = true)]
    pretty: bool,

    /// Enforce TLS cert validation. Off by default because the Gateway
    /// ships with a self-signed cert.
    #[arg(long, env = "BEZANT_REJECT_INVALID_CERTS", global = true)]
    reject_invalid_certs: bool,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check whether the Gateway session is live and authenticated.
    Health,

    /// List every account the Gateway is aware of.
    Accounts,

    /// Portfolio summary for one account.
    Summary {
        /// Account id (e.g. `DU123456`).
        account: String,
    },

    /// All positions for one account (paginated → flattened).
    Positions {
        /// Account id (e.g. `DU123456`).
        account: String,
    },

    /// Resolve a ticker symbol to an IBKR `conid` via contract search.
    Conid {
        /// Ticker (e.g. `AAPL`, `BRK B`).
        symbol: String,
    },

    /// Send a single tickle to the Gateway to extend the session.
    Tickle,
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("bezant: {e:#}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let client = Client::builder(&cli.gateway_url)
        .accept_invalid_certs(!cli.reject_invalid_certs)
        .build()?;

    match cli.cmd {
        Cmd::Health => {
            let status = client.auth_status().await?;
            print_json(
                &serde_json::json!({
                    "authenticated": status.authenticated,
                    "connected": status.connected,
                    "competing": status.competing,
                    "message": status.message,
                }),
                cli.pretty,
            )?;
        }
        Cmd::Tickle => {
            let tickle = client.tickle().await?;
            print_json(
                &serde_json::json!({
                    "session": tickle.session,
                }),
                cli.pretty,
            )?;
        }
        Cmd::Accounts => {
            let resp = client
                .api()
                .get_all_accounts(bezant_api::GetAllAccountsRequest::default())
                .await?;
            match resp {
                bezant_api::GetAllAccountsResponse::Ok(_) => {
                    // `Account200Response2` is Deserialize-only, so we re-fetch
                    // as a raw JSON value via the facade's underlying HTTP
                    // client. This keeps CLI output uniform.
                    let raw = fetch_json(&client, &["portfolio", "accounts"]).await?;
                    print_json(&raw, cli.pretty)?;
                }
                bezant_api::GetAllAccountsResponse::Unauthorized => {
                    anyhow::bail!("gateway is not authenticated — log in via the browser first");
                }
                bezant_api::GetAllAccountsResponse::InternalServerError => {
                    anyhow::bail!("upstream 500")
                }
                bezant_api::GetAllAccountsResponse::ServiceUnavailable => {
                    anyhow::bail!("gateway service unavailable")
                }
                bezant_api::GetAllAccountsResponse::Unknown => {
                    anyhow::bail!("unknown upstream response")
                }
            }
        }
        Cmd::Summary { account } => {
            let raw = fetch_json(&client, &["portfolio", account.as_str(), "summary"]).await?;
            print_json(&raw, cli.pretty)?;
        }
        Cmd::Positions { account } => {
            // Use the typed helper so pagination happens under the hood; we
            // serialise the resulting Vec<IndividualPosition> back to JSON
            // for output. IndividualPosition is Deserialize-only, so route
            // through the raw HTTP path instead to preserve upstream
            // field naming.
            let mut all = Vec::new();
            for page in 0..100u32 {
                let page_str = page.to_string();
                let raw = fetch_json(
                    &client,
                    &["portfolio", account.as_str(), "positions", &page_str],
                )
                .await?;
                let arr = raw.as_array().cloned().unwrap_or_default();
                let n = arr.len();
                all.extend(arr);
                if n < 30 {
                    break;
                }
            }
            print_json(&Value::Array(all), cli.pretty)?;
        }
        Cmd::Conid { symbol } => {
            let cache = bezant::SymbolCache::new(client);
            let conid = cache.conid_for(&symbol).await?;
            print_json(
                &serde_json::json!({
                    "symbol": symbol,
                    "conid": conid,
                }),
                cli.pretty,
            )?;
        }
    }
    Ok(())
}

/// Fetch a JSON body by building a path relative to the Gateway base URL.
async fn fetch_json(client: &Client, path_segments: &[&str]) -> anyhow::Result<Value> {
    let mut url = client.base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| anyhow::anyhow!("base url cannot be a base"))?;
        for s in path_segments {
            segs.push(s);
        }
    }
    let resp = client.http().get(url).send().await?;
    let status = resp.status();
    let bytes = resp.bytes().await?;
    if !status.is_success() {
        let body = String::from_utf8_lossy(&bytes);
        anyhow::bail!("HTTP {status}: {body}");
    }
    let v: Value = serde_json::from_slice(&bytes)?;
    Ok(v)
}

fn print_json(value: &Value, pretty: bool) -> anyhow::Result<()> {
    let out = if pretty {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    };
    println!("{out}");
    Ok(())
}
