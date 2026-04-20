# bezant

Ergonomic async Rust client for the Interactive Brokers Client Portal Web
API, layered over the auto-generated [`bezant-api`](https://crates.io/crates/bezant-api)
crate.

See the workspace root for the full picture:
<https://github.com/isaacrowntree/bezant>.

## Quick feature tour

- `Client::new` / `builder()` — sane defaults for talking to the Gateway
  (self-signed certs, 30s timeout, cookie jar, useful user-agent).
- `spawn_keepalive` — drop-to-stop background task that tickles `/tickle`.
- `health()` / `auth_status()` — typed check against `/iserver/auth/status`
  returning `NotAuthenticated` / `NoSession` as distinct errors.
- `all_positions` — auto-paginated helper across `/portfolio/{id}/positions/{page}`.
- `SymbolCache` — ticker → conid resolver with per-session memoisation.
- `WsClient` — WebSocket streaming using the REST session's cookie, with
  typed subscribe helpers for market data / orders / PnL.
- `api()` — escape hatch onto the full generated client for endpoints this
  facade doesn't wrap explicitly.

## Example

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    let client = bezant::Client::new("https://localhost:5000/v1/api")?;
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
    client.health().await?;

    let positions = client.all_positions("DU123456").await?;
    println!("{} open positions", positions.len());
    Ok(())
}
```

## License

Dual-licensed under MIT or Apache-2.0 at your option.
