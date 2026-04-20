# bezant

**Async Rust client for the Interactive Brokers Client Portal Web API.**

Bezant gives Rust applications a typed, ergonomic way to talk to IBKR's
Client Portal Gateway — positions, market data, orders, account summary,
the works. It's built on top of the official OpenAPI 3.0 spec IBKR publishes
at <https://api.ibkr.com/gw/api/v3/api-docs>.

## Status

**Alpha — v0.1 pre-release.** Works end-to-end against the IBKR paper account.
Not on crates.io yet. API may evolve.

## What's in the box

| Crate / package | Role |
|---|---|
| [`bezant-spec`](crates/bezant-spec) | Vendored + normalised copy of the IBKR OpenAPI spec |
| [`bezant-api`](crates/bezant-api) | Auto-generated Rust client covering all ~155 endpoints (167 typed methods, 1030 types) |
| [`bezant`](crates/bezant-core) | Ergonomic Rust facade — keepalive, WebSocket streaming, pagination, symbol cache |
| [`bezant-server`](crates/bezant-server) | HTTP sidecar exposing the CPAPI over plain REST so any language can consume it |
| [`bezant-client` (TS)](clients/typescript) | Auto-generated TypeScript client for Node / Deno / browser consumers |

## Quickstart

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    // Point at the Gateway running on localhost (Docker image, TWS, or official).
    let client = bezant::Client::new("https://localhost:5000/v1/api")?;

    // Keep the CPAPI session alive in the background.
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));

    // Returns early with a typed error if the user hasn't logged in via the
    // Gateway's browser UI.
    client.health().await?;

    // All ~155 CPAPI endpoints are available as typed methods on the
    // underlying generated client.
    let accounts = client
        .api()
        .get_portfolio_accounts(bezant::api::GetPortfolioAccountsRequest::default())
        .await?;
    println!("{accounts:#?}");

    Ok(())
}
```

## One-command setup via Docker

```sh
docker compose up
```

Brings up the IBKR Client Portal Gateway on <https://localhost:5000>
(log in once via browser) + `bezant-server` on <http://localhost:8080>.
Then e.g.:

```sh
curl http://localhost:8080/health
curl http://localhost:8080/accounts
curl "http://localhost:8080/accounts/DU123456/positions?page=0"
```

## Prerequisites

1. An IBKR account (paper or live).
2. The IBKR **Client Portal Gateway** running locally. The official Java app
   is fine; Bezant will also ship a Docker image bundling it.
3. Log in to the Gateway once via the browser (`https://localhost:5000`);
   Bezant will keep the session alive from there.

## Architecture

```
┌─────────────────┐
│ your code       │  use bezant::Client
└────────┬────────┘
         │
┌────────▼────────┐
│ bezant (facade) │  keepalive, health, retries, typed errors
└────────┬────────┘
         │
┌────────▼────────┐
│ bezant-api      │  auto-generated from the OpenAPI spec
│  (1030 types,   │  via oas3-gen (see ROADMAP)
│   167 methods)  │
└────────┬────────┘
         │ HTTPS + cookie session
┌────────▼────────┐
│ IBKR CP Gateway │  official Java app, runs on :5000
└────────┬────────┘
         │
    IBKR servers
```

## Regenerating the API crate

The vendored spec needs a few upstream quirks ironed out before any code
generator can consume it. The pipeline lives in `scripts/`:

```sh
# 1. fetch + normalise upstream (uses Python's stdlib + jq)
./scripts/refresh-spec.sh        # re-download from api.ibkr.com
./scripts/codegen.sh             # normalise → upgrade to 3.1 → run oas3-gen
```

`codegen.sh` applies 13 spec normalisations (see `scripts/normalize-spec.py`
for the full list of upstream quirks — most of which should ideally become
bug reports to IBKR) and upgrades the 3.0 spec to 3.1 before feeding it to
[oas3-gen](https://github.com/eklipse2k8/oas3-gen).

## License

Dual-licensed under either:

- [MIT](LICENSE-MIT)
- [Apache 2.0](LICENSE-APACHE)

at your option, following the Rust ecosystem convention.

The **vendored OpenAPI spec itself** is Interactive Brokers' intellectual
property, included here under fair-use conventions for interoperability with
their public API.

## Not affiliated with Interactive Brokers

Bezant is an independent open-source project. Trading involves substantial
risk; this software is provided without warranty. See the license text.
