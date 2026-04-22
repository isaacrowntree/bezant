# Rust crate (`bezant`)

Full rustdoc is deployed alongside this book — see
[Rust API reference](../reference/rustdoc.md).

## Feature tour

### Typed client with sane defaults

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    let client = bezant::Client::builder("https://localhost:5000/v1/api")
        .timeout(Duration::from_secs(30))
        .accept_invalid_certs(true)       // Gateway uses a self-signed cert
        .user_agent("my-bot/0.1")
        .build()?;

    // keeps /tickle firing every 60s in the background
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));

    // returns typed errors: NotAuthenticated, NoSession, Http, Api, ...
    let status = client.health().await?;
    println!("gateway: {status:?}");
    Ok(())
}
```

### Paginated positions helper

Skip writing the `/positions/{page}` loop yourself:

```rust,no_run
# use bezant::Client;
# async fn demo(client: Client) -> bezant::Result<()> {
let positions: Vec<bezant::Position> = client.all_positions("DU123456").await?;
println!("{} open positions", positions.len());
# Ok(()) }
```

### Symbol → conid cache

```rust,no_run
# use bezant::Client;
# async fn demo(client: Client) -> bezant::Result<()> {
let cache = bezant::SymbolCache::new(client);
let aapl = cache.conid_for("AAPL").await?;   // network call
let aapl2 = cache.conid_for("AAPL").await?;  // cached
assert_eq!(aapl, aapl2);
# Ok(()) }
```

### WebSocket streaming

```rust,no_run
# use bezant::{Client, WsClient, MarketDataFields, WsMessage};
# async fn demo(client: Client) -> bezant::Result<()> {
let mut ws = WsClient::connect(&client).await?;
ws.subscribe_market_data(265598 /* AAPL */, &MarketDataFields::default_l1()).await?;

while let Some(msg) = ws.next_message().await? {
    match msg {
        WsMessage::MarketData { conid, payload } => println!("{conid}: {payload}"),
        WsMessage::Order(o) => println!("order update: {o}"),
        _ => {}
    }
}
# Ok(()) }
```

### Raw access to every CPAPI endpoint

The ergonomic facade covers the 80% use case. For the long tail (155 endpoints)
drop straight into the generated client:

```rust,no_run
# use bezant::Client;
# async fn demo(client: Client) -> bezant::Result<()> {
let resp = client
    .api()
    .get_portfolio_summary(bezant::api::GetPortfolioSummaryRequest {
        path: bezant::api::GetPortfolioSummaryRequestPath {
            account_id: "DU123456".into(),
        },
    })
    .await?;
# Ok(()) }
```

## Error handling

`bezant::Error` is `#[non_exhaustive]` and covers:

| Variant | Meaning |
|---|---|
| `InvalidBaseUrl` | The base URL passed to `Client::new` didn't parse |
| `Http` | Transport failure (DNS, TLS, timeouts) |
| `Api` | Anything the generated client bubbled up |
| `NotAuthenticated` | Gateway returned 401 — user hasn't logged in |
| `NoSession` | Gateway is reachable but reports `connected: false` |
| `Other(String)` | Misc failures that don't fit above |

Client code should pattern-match on the variants it cares about and use
`_ => ...` for the rest (important because the enum is `#[non_exhaustive]`).

## Runnable examples

Clone the repo and try the bundled examples against the local Docker
gateway without writing any code:

```sh
docker compose up -d
# open https://localhost:5000 once in your browser to log in

cargo run -p bezant-core --example health
IBKR_ACCOUNT_ID=DU123456 cargo run -p bezant-core --example list_positions
IBKR_SYMBOL=AAPL        cargo run -p bezant-core --example stream_quotes
```

Source: [`crates/bezant-core/examples/`](https://github.com/isaacrowntree/bezant/tree/main/crates/bezant-core/examples).
