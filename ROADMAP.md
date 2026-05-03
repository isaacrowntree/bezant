# Roadmap

## v0.1 — alpha ✅ shipped

End-to-end rebalancing-bot use case.

- [x] Vendor + normalise IBKR OpenAPI spec (`bezant-spec`)
- [x] Codegen all ~154 CPAPI endpoints via oas3-gen (`bezant-api`)
- [x] Ergonomic facade: Client, auth, keepalive, health (`bezant`)
- [x] HTTP sidecar exposing the facade over REST (`bezant-server`)
- [x] Docker image bundling IBKR Gateway + bezant-server
- [x] 6 wiremock-backed facade tests + 2 spec tests + 1 doctest
- [x] GitHub Actions CI (fmt, clippy, check, test)
- [x] Dual MIT / Apache-2.0 license

## v0.2 — streaming + ergonomics 🚧 in progress

Goal: make Bezant fully usable for a real trading bot (not just paper-mode reads).

### Stream + DX
- [x] **WebSocket client** (`bezant::WsClient`): cookie auth from the REST session, typed subscribe helpers for market data / orders / PnL, raw message stream escape hatch, split-for-concurrency helper.
- [ ] **Tracing spans** on every facade method so users can drop in their subscriber of choice.
- [ ] **Pagination helpers** — `all_positions()`, `all_orders()` that walk every page.
- [ ] **Symbol → conid cache** — one call per ticker per day, stored in the Client.
- [ ] **Retry middleware** — exponential backoff on 5xx + 429.
- [ ] **Typed error variants** for common IBKR failure modes (insufficient funds, market closed, restricted account).

### Codegen investments
- [ ] **Snapshot tests from spec examples** — extract every `examples.*.value` from the OpenAPI spec, generate a test per example that deserialises into the matching Rust type. Catches upstream schema drift automatically.
- [ ] **Upstream bug reports** — file tickets with IBKR for each spec quirk in `scripts/normalize-spec.py`; aim to delete normalisation steps as upstream fixes them.

## v0.3 — ecosystem 🔭 planned

Broader adoption play.

### Additional clients (all driven from the same vendored spec)
- [ ] **`clients/typescript`** — auto-generated via `openapi-generator-cli -g typescript-fetch`. Lets non-Rust callers use the typed client directly without the HTTP sidecar.
- [ ] **`bezant-cli`** — `bezant get-accounts`, `bezant positions --account DU123456`, etc. Fills the real gap of "no mature IBKR CLI". Generated subcommands from the spec plus hand-written aliases for the ergonomic operations.
- [ ] **Python bindings via pyo3** — `pip install bezant` for quant scripts. Thin layer over the Rust facade.

### LLM integration
- [ ] **`bezant-mcp`** — Model Context Protocol server exposing every CPAPI endpoint as an MCP tool. Lets Claude / GPT-4 / any MCP-compatible client call IBKR directly. Unique niche — no existing OpenAPI → MCP generator.

### Robustness
- [ ] **Live-account integration tests** gated behind `BEZANT_PAPER_ACCOUNT_ID` env var, run nightly.
- [ ] **`bezant::ws::TickerManager`** — auto-reconnect on disconnect, re-subscribes existing topics, exposed as a background actor.
- [ ] **OAuth 1.0a / 2.0 auth** as IBKR finalises public rollout.

## v1.0 — stable

- Stable public API. SemVer discipline.
- Production-grade docs (mdbook?) + examples.
- Reference rebalancing bot as a published companion crate.
- Options / futures / forex / fixed income convenience builders.

## Contributing

PRs welcome. If you hit a new spec quirk that isn't in
`scripts/normalize-spec.py`, please open an issue with the failing operation
ID or schema name and ideally the minimal reproducer — that lets us expand
both the normaliser **and** the upstream bug report against IBKR.
