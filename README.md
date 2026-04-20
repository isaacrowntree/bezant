# bezant

**Typed async access to the Interactive Brokers Client Portal Web API — Rust-first, with HTTP, CLI, MCP, and TypeScript surfaces auto-generated from the same vendored OpenAPI spec.**

[![CI](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml/badge.svg)](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml)
[![Docs](https://github.com/isaacrowntree/bezant/actions/workflows/docs.yml/badge.svg)](https://isaacrowntree.github.io/bezant/)
[![Audit](https://github.com/isaacrowntree/bezant/actions/workflows/audit.yml/badge.svg)](https://github.com/isaacrowntree/bezant/actions/workflows/audit.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV: 1.85](https://img.shields.io/badge/MSRV-1.85-orange.svg)](rust-toolchain.toml)

> 📖 **Docs:** <https://isaacrowntree.github.io/bezant/> · 📦 **Issues:** <https://github.com/isaacrowntree/bezant/issues>

---

Bezant turns IBKR's 155-endpoint CPAPI into **five ergonomic surfaces** that all
ship from the same vendored OpenAPI 3.1 spec:

| Crate / package | Purpose |
|---|---|
| [`bezant`](crates/bezant-core) | Ergonomic async Rust facade — `Client`, keepalive, health, WebSocket streaming, pagination, symbol cache, typed errors |
| [`bezant-api`](crates/bezant-api) | Auto-generated Rust client covering **155 paths / 167 methods / 1030 types** |
| [`bezant-server`](crates/bezant-server) | HTTP sidecar — exposes CPAPI as plain REST+JSON so any language can consume it |
| [`bezant-cli`](crates/bezant-cli) | `bezant` CLI — `bezant health`, `bezant positions DU123456`, `bezant conid AAPL`, etc. |
| [`bezant-mcp`](crates/bezant-mcp) | Model Context Protocol server — exposes IBKR as MCP tools for Claude / Cursor / Continue |
| [`bezant-client`](clients/typescript) | TypeScript client — generated from the same spec for Node / Deno / browser |

All five are driven from one vendored spec. Re-run `./scripts/codegen.sh`
when IBKR ships a new revision and every surface updates together.

## Quickstart (Docker)

```sh
git clone https://github.com/isaacrowntree/bezant
cd bezant
docker compose up
```

- <https://localhost:5000> — log in to the IBKR Gateway once in a browser
- <http://localhost:8080/health> — verify the sidecar sees an authenticated session

```sh
curl http://localhost:8080/accounts
curl "http://localhost:8080/accounts/DU123456/positions?page=0"
```

Full quickstart for every surface: <https://isaacrowntree.github.io/bezant/quickstart.html>

## Rust

```toml
[dependencies]
bezant = { git = "https://github.com/isaacrowntree/bezant", package = "bezant-core" }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    let client = bezant::Client::new("https://localhost:5000/v1/api")?;
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
    client.health().await?;

    // ergonomic helpers on top of the generated client
    let positions = client.all_positions("DU123456").await?;
    let aapl = bezant::SymbolCache::new(client).conid_for("AAPL").await?;
    println!("{} positions; AAPL = conid {aapl}", positions.len());
    Ok(())
}
```

## Highlights

- **Typed, async, reqwest-based client** covering every CPAPI endpoint.
- **Session keepalive** built in — no more 5-minute session expiries.
- **WebSocket streaming** with cookie auth reused from the REST session
  and typed subscribe helpers for market data / orders / PnL.
- **Enterprise-grade spec normalisation** (13 steps) works around real
  IBKR quirks — missing / duplicate `operationId`s, malformed
  `security[]`, integer fields with float example values, and more.
  Documented in detail at [Spec normalisation](https://isaacrowntree.github.io/bezant/internals/normalisation.html).
- **Snapshot tests** driven by real IBKR example payloads — catches
  upstream spec drift before our users do.
- **34 tests** across the workspace, all green in CI (unit, integration,
  snapshot, end-to-end against wiremock-mocked gateways).
- **Dual MIT / Apache-2.0** licensing following the Rust ecosystem convention.

## Repository layout

```
crates/
  bezant-spec/     — vendored IBKR OpenAPI spec + refresh tooling
  bezant-api/      — auto-generated Rust client (don't hand-edit)
  bezant-core/     — ergonomic facade
  bezant-server/   — axum HTTP sidecar
  bezant-cli/      — clap-powered CLI
  bezant-mcp/      — MCP server
clients/
  typescript/      — auto-generated TS client
scripts/           — spec pipeline (normalise → upgrade → codegen)
docs/              — mdbook source for the docs site
xtask/             — dev-only tools (spec probing / bisection)
.github/           — CI, release, docs, audit workflows
```

## Contributing

We welcome contributions — especially spec-normaliser improvements and
new client languages. See [CONTRIBUTING.md](CONTRIBUTING.md) for the dev
workflow, or open a discussion on GitHub if you want to talk architecture.

## Status

**Alpha — v0.1.** Works end-to-end against IBKR paper accounts. API
surface will evolve until v1.0. See the
[ROADMAP](ROADMAP.md) for what's next.

## Security

See [SECURITY.md](SECURITY.md) for disclosure policy. **Do not open a
public issue for vulnerabilities** — email the maintainer instead.

## License

Dual-licensed under your choice of:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

Contributions are assumed to be offered under the same terms.

The **vendored IBKR OpenAPI spec** itself is Interactive Brokers' IP;
included under fair-use conventions for interoperability.

## Not affiliated with Interactive Brokers

Bezant is an independent open-source project. Trading involves substantial
risk; this software is provided **without warranty**. See the license text.
