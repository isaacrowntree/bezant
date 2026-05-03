# Roadmap

## v0.1 — alpha ✅ shipped (2026-04-21)

End-to-end rebalancing-bot use case.

- [x] Vendor + normalise IBKR OpenAPI spec (`bezant-spec`)
- [x] Codegen all 154 CPAPI endpoints via oas3-gen (`bezant-api`)
- [x] Ergonomic facade: Client, auth, keepalive, health (`bezant-core`)
- [x] HTTP sidecar exposing the facade over REST (`bezant-server`)
- [x] Docker image bundling IBKR Gateway + bezant-server
- [x] WebSocket client with cookie auth + typed subscribe helpers
- [x] Pagination helpers + symbol → conid cache
- [x] Tracing instrumentation across the facade
- [x] CLI (`bezant-cli`) + MCP server (`bezant-mcp`) + TypeScript client
- [x] Snapshot tests driven by spec example payloads
- [x] GitHub Actions CI (fmt, clippy, test, MSRV, audit, multi-arch Docker)
- [x] Dual MIT / Apache-2.0 license

## v0.2 — production hardening ✅ shipped (2026-05-03)

Goal: deployable to a real production trading bot, not just localhost dev.

- [x] **Cloudflare Zero Trust + residential-Pi deploy guide** — bypasses
      IBKR's Akamai datacenter-IP rejection, the silent killer of cloud-
      hosted CPAPI deploys
- [x] **`NameKeyedJar`** cookie store — replaces reqwest's path-aware
      jar to fix duplicate `JSESSIONID` accumulation that CPGateway rejects
- [x] **Edge-cookie filter** — drops `CF_Authorization` / `CF_AppSession` /
      AWS ALB / OAuth2 Proxy / Pomerium / Vercel cookies before they
      poison the upstream call (Akamai 401s on unrecognised cookies)
- [x] **`/debug/probe` + `/debug/jar`** diagnostics, gated by
      `BEZANT_DEBUG_TOKEN` (constant-time compare, names+lengths only,
      never raw values)
- [x] Strip `Authorization` / `X-Forwarded-*` / `Forwarded` / `X-Real-IP`
      at the proxy boundary
- [x] Multi-arch Docker builds on native arm64 GitHub runners
      (~5 min vs ~20 min QEMU)

## v0.3 — typed surface + observability ✅ shipped (2026-05-03)

Goal: library-quality ergonomics + production-debuggable runtime.

- [x] **11 typed `Error` variants** replacing `Error::Other(String)` —
      `UpstreamStatus`, `Unknown`, `UrlNotABase`, `MissingQuery`, `Header`,
      `SymbolNotFound`, `BadConid`, `WsHandshake`, `WsTransport`,
      `WsProtocol`, `ResponseBuild`
- [x] **`Error::is_retryable()`** for backoff loops
- [x] **`bezant::prelude`** for the typical bot use case
- [x] **`#[non_exhaustive]`** on `AuthStatus` + `TickleResponse` so
      future fields aren't SemVer breaks
- [x] **Per-request correlation IDs** (`SetRequestIdLayer` +
      `PropagateRequestIdLayer`) + handler `#[tracing::instrument]` +
      keepalive task span
- [x] **Graceful shutdown** (SIGTERM/SIGINT drain + awaited
      `keepalive.stop()`) + `ConcurrencyLimitLayer(256)` + reqwest
      pool tuning (`pool_max_idle_per_host`, `tcp_keepalive`,
      `connect_timeout`, `pool_idle_timeout`)
- [x] **`KeepaliveHandle::Drop`** sends shutdown signal so a forgotten
      handle doesn't keep tickling
- [x] **WebSocket `Subscription` handle** — RAII cancel via
      `Subscription::cancel(&mut ws).await` instead of caller-tracked
      conids; `WsClient::split` returns concrete `WsSink`/`WsRecv`;
      `WsMessage::topic()`/`as_value()` accessors
- [x] **`/debug/probe` per-step timeout (5s)** + body-preview redaction
      (`session`/`token`/`secret` keys) + non-destructive ssodh skip
- [x] **`bezant-cli --output {json,table}`** + `quote SYMBOL` +
      `orders ACCOUNT` + cap warning on `MAX_POSITION_PAGES`
- [x] **14 spec-normaliser invariant tests** + CI drift-check job
- [x] **Published to crates.io** at v0.3.0

## v0.4 — feature flags + auto-reconnect 🔭 planned

Goal: smooth out remaining rough edges; expand for non-Rust ecosystems.

### Library
- [ ] **Feature flags** on `bezant-core` (`ws`, `keepalive-tokio`) so
      callers don't pay for tokio-tungstenite if they only want REST
- [ ] **Async runtime decoupling** — `spawn_keepalive` accepts a
      runtime handle so async-std / smol consumers can use the crate
- [ ] **`bezant::ws::TickerManager`** — auto-reconnect on disconnect,
      re-subscribes existing topics, exposed as a background actor
- [ ] **Retry middleware** with exponential backoff on `is_retryable()`
- [ ] **Typed error variants** for common IBKR failure modes
      (insufficient funds, market closed, restricted account)

### MCP + ecosystem
- [ ] **`bezant-mcp` market data + orders** tools (currently read-only),
      gated behind `--allow-orders` so registration itself is opt-in
- [ ] **MCP resources** for accounts/positions so Claude can include
      state in context without explicit tool calls
- [ ] **Python bindings via pyo3** — `pip install bezant` for quant
      scripts

### Robustness
- [ ] **Live-account integration tests** gated behind a feature flag,
      opt-in via env var
- [ ] **OAuth 1.0a / 2.0 auth** when IBKR opens it to retail accounts
- [ ] **Anyhow-free `bezant-core`** — redrive `helpers.rs` / `auth.rs`
      off the generated client's typed Result so anyhow can become
      optional

## v1.0 — stable

- Stable public API. SemVer discipline.
- Production-grade docs + examples for every surface.
- Reference rebalancing bot as a published companion crate.
- Options / futures / forex / fixed income convenience builders.

## Contributing

PRs welcome. If you hit a new spec quirk that isn't in
`scripts/normalize-spec.py`, please open an issue with the failing operation
ID or schema name and ideally the minimal reproducer — that lets us expand
both the normaliser **and** the upstream bug report against IBKR.
