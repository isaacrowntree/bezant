//! `bezant` — command-line tool for the IBKR Client Portal Web API.
//!
//! Wraps [`bezant::Client`] with a clap-powered CLI so ops / shell users can
//! poke the Gateway without pulling in the Rust crate as a dependency.
//!
//! Output format defaults to JSON (machine-readable, pipe-friendly). Pass
//! `--output table` for a human-readable table on subcommands where one
//! makes sense (positions, accounts, orders); the table renderer falls
//! back to JSON for endpoints whose shape isn't tabular (`tickle`,
//! `health`).
//!
//! Every subcommand prints to stdout on success and a non-zero exit on
//! failure. Errors are printed to stderr as a one-line `bezant: <message>`
//! followed by indented `caused by:` lines for each layer of the error chain.
//!
//! ## Environment
//!
//! | Env var               | Default                         | Meaning                                    |
//! |-----------------------|---------------------------------|--------------------------------------------|
//! | `IBKR_GATEWAY_URL`    | `https://localhost:5000/v1/api` | Base URL of the Client Portal Gateway      |
//! | `BEZANT_VERIFY_TLS`   | `false`                         | Set to `true` to enforce cert validation   |

use std::process::ExitCode;

use bezant::Client;
use clap::{Parser, Subcommand, ValueEnum};
use comfy_table::{Cell, ContentArrangement, Table};
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

    /// Pretty-print JSON output (default: compact). Ignored when
    /// `--output table` is set.
    #[arg(long, global = true)]
    pretty: bool,

    /// Output format. `json` (the default) is machine-readable and
    /// pipe-friendly; `table` renders human-readable rows for tabular
    /// endpoints (positions, accounts, orders) and falls back to JSON
    /// for non-tabular ones.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json, global = true)]
    output: OutputFormat,

    /// Verify the Gateway's TLS certificate. Off by default because
    /// the Gateway ships with a self-signed cert. Replaces the older
    /// `--reject-invalid-certs` (whose double negative made it easy
    /// to accidentally leave invalid-cert acceptance on in
    /// production).
    #[arg(long, env = "BEZANT_VERIFY_TLS", global = true)]
    verify_tls: bool,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    /// Compact JSON (the default). Use `--pretty` to indent it.
    Json,
    /// Human-readable table. Falls back to JSON for non-tabular endpoints.
    Table,
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

    /// Live + recently-filled orders for one account.
    Orders {
        /// Account id (e.g. `DU123456`).
        account: String,
    },

    /// Real-time market-data snapshot for a symbol. Resolves the
    /// symbol to its conid via contract search, then queries the
    /// snapshot endpoint for the default level-1 fields.
    Quote {
        /// Ticker (e.g. `AAPL`).
        symbol: String,
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
            // Print the top-level error plus every `source()` link in
            // the anyhow chain — TLS / connect failures frequently
            // carry useful detail in the wrapped causes, and the
            // single-line `{e:#}` format doesn't surface them.
            let mut chain = e.chain();
            if let Some(first) = chain.next() {
                eprintln!("bezant: {first}");
            }
            for cause in chain {
                eprintln!("  caused by: {cause}");
            }
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let client = Client::builder(&cli.gateway_url)
        .accept_invalid_certs(!cli.verify_tls)
        .build()?;
    // Pull the output knobs into locals before the `match cli.cmd`
    // moves command-specific fields out of `cli` — keeps the
    // per-arm `render(...)` calls borrow-clean.
    let output = cli.output;
    let pretty = cli.pretty;

    match cli.cmd {
        Cmd::Health => {
            let status = client.auth_status().await?;
            let body = serde_json::json!({
                "authenticated": status.authenticated,
                "connected": status.connected,
                "competing": status.competing,
                "message": status.message,
            });
            // Health is a flat key/value object — table render is just
            // a 2-column "field / value" view.
            render(output, pretty, &body, |body| Some(kv_table(body)))?;
        }
        Cmd::Tickle => {
            let tickle = client.tickle().await?;
            let body = serde_json::json!({ "session": tickle.session });
            render(output, pretty, &body, |_| None)?; // no useful tabular form
        }
        Cmd::Accounts => {
            let raw = fetch_json(&client, &["portfolio", "accounts"]).await?;
            render(output, pretty, &raw, |body| {
                array_table(body, &["accountId", "accountTitle", "currency", "type"])
            })?;
        }
        Cmd::Summary { account } => {
            let raw = fetch_json(&client, &["portfolio", account.as_str(), "summary"]).await?;
            render(output, pretty, &raw, |body| Some(kv_table(body)))?;
        }
        Cmd::Positions { account } => {
            let positions = paginated_positions(&client, &account).await?;
            let body = Value::Array(positions);
            render(output, pretty, &body, |body| {
                array_table(
                    body,
                    &[
                        "ticker", "conid", "position", "avgCost", "mktPrice", "mktValue",
                        "currency",
                    ],
                )
            })?;
        }
        Cmd::Orders { account } => {
            // CPAPI: GET /iserver/account/orders?accountId=…
            let mut url = client.base_url().clone();
            {
                let mut segs = url
                    .path_segments_mut()
                    .map_err(|()| anyhow::anyhow!("base url cannot be a base"))?;
                segs.push("iserver").push("account").push("orders");
            }
            url.query_pairs_mut().append_pair("accountId", &account);
            let resp = client.http().get(url).send().await?;
            let body = decode_json(resp).await?;
            // Response may be wrapped as `{"orders": [...]}` or a bare
            // array depending on Gateway version. Normalise to the array.
            let orders = body
                .get("orders")
                .and_then(Value::as_array)
                .cloned()
                .map_or(body.clone(), Value::Array);
            render(output, pretty, &orders, |body| {
                array_table(
                    body,
                    &[
                        "orderId",
                        "ticker",
                        "side",
                        "totalSize",
                        "filledQuantity",
                        "status",
                        "orderType",
                    ],
                )
            })?;
        }
        Cmd::Quote { symbol } => {
            let cache = bezant::SymbolCache::new(client.clone());
            let conid = cache.conid_for(&symbol).await?;
            // CPAPI: GET /iserver/marketdata/snapshot?conids=N&fields=…
            let mut url = client.base_url().clone();
            {
                let mut segs = url
                    .path_segments_mut()
                    .map_err(|()| anyhow::anyhow!("base url cannot be a base"))?;
                segs.push("iserver").push("marketdata").push("snapshot");
            }
            url.query_pairs_mut()
                .append_pair("conids", &conid.to_string())
                .append_pair("fields", "31,84,86,87,85,88");
            let resp = client.http().get(url).send().await?;
            let body = decode_json(resp).await?;
            // Snapshot returns an array of one object keyed by field code.
            // Map to friendly names for the table.
            let snapshot = body.get(0).cloned().unwrap_or(body.clone());
            let quote = serde_json::json!({
                "symbol": symbol,
                "conid": conid,
                "last": snapshot.get("31"),
                "bid": snapshot.get("84"),
                "ask": snapshot.get("86"),
                "volume": snapshot.get("87"),
                "bid_size": snapshot.get("88"),
                "ask_size": snapshot.get("85"),
            });
            render(output, pretty, &quote, |body| Some(kv_table(body)))?;
        }
        Cmd::Conid { symbol } => {
            let cache = bezant::SymbolCache::new(client);
            let conid = cache.conid_for(&symbol).await?;
            let body = serde_json::json!({ "symbol": symbol, "conid": conid });
            render(output, pretty, &body, |_| None)?; // no useful tabular form
        }
    }
    Ok(())
}

/// Walk pages of `/portfolio/{id}/positions/{page}` until a short page
/// arrives or we hit the safety cap. Emits a stderr warning when the
/// cap fires so the caller knows results were truncated — silently
/// hitting `MAX_POSITION_PAGES` was a coverage gap flagged in review.
async fn paginated_positions(client: &Client, account: &str) -> anyhow::Result<Vec<Value>> {
    let mut all = Vec::new();
    for page in 0..bezant::MAX_POSITION_PAGES {
        let page_str = page.to_string();
        let raw = fetch_json(client, &["portfolio", account, "positions", &page_str]).await?;
        let arr = raw.as_array().cloned().unwrap_or_default();
        let n = arr.len();
        all.extend(arr);
        if n < bezant::POSITIONS_PAGE_SIZE {
            return Ok(all);
        }
    }
    eprintln!(
        "bezant: warning — hit MAX_POSITION_PAGES ({}) for account {}; results may be truncated",
        bezant::MAX_POSITION_PAGES,
        account
    );
    Ok(all)
}

/// Render `body` either as JSON (always works) or via `to_table` if the
/// caller supplied a tabular projection AND `--output table` was passed.
/// `to_table` returns `None` when the endpoint shape doesn't have a
/// useful table form (e.g. `tickle` returns just a session id).
fn render(
    output: OutputFormat,
    pretty: bool,
    body: &Value,
    to_table: impl Fn(&Value) -> Option<Table>,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Json => print_json(body, pretty),
        OutputFormat::Table => match to_table(body) {
            Some(table) => {
                println!("{table}");
                Ok(())
            }
            // Table not meaningful for this shape — fall back to JSON
            // (pretty-printed because the user asked for human readable).
            None => print_json(body, true),
        },
    }
}

/// Render a JSON object as a 2-column "field / value" table.
fn kv_table(body: &Value) -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![Cell::new("field"), Cell::new("value")]);
    if let Some(obj) = body.as_object() {
        for (k, v) in obj {
            table.add_row(vec![Cell::new(k), Cell::new(value_cell(v))]);
        }
    }
    table
}

/// Render a JSON array of objects as a table whose columns are the
/// supplied field names. Returns None when the body isn't an array
/// (caller falls back to JSON).
fn array_table(body: &Value, columns: &[&str]) -> Option<Table> {
    let arr = body.as_array()?;
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(columns.iter().map(Cell::new).collect::<Vec<_>>());
    for item in arr {
        let row = columns
            .iter()
            .map(|c| {
                let v = item.get(*c).cloned().unwrap_or(Value::Null);
                Cell::new(value_cell(&v))
            })
            .collect::<Vec<_>>();
        table.add_row(row);
    }
    Some(table)
}

/// Stringify a JSON value for table display. Strings without quotes,
/// nulls as a literal "-" (so empty cells stand out), nested
/// objects/arrays compact-JSON'd to keep rows readable.
fn value_cell(v: &Value) -> String {
    match v {
        Value::Null => "-".to_owned(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Array(_) | Value::Object(_) => v.to_string(),
    }
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
    decode_json(resp).await
}

async fn decode_json(resp: reqwest::Response) -> anyhow::Result<Value> {
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
