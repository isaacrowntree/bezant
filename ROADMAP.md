# Roadmap

## v0.1 — alpha (current)

**Goal:** covers the rebalancing-bot use case end-to-end.

- [x] Vendor + normalise IBKR OpenAPI spec (`bezant-spec`)
- [x] Codegen all ~155 CPAPI endpoints (`bezant-api` via oas3-gen)
- [x] Ergonomic facade with Client, auth, keepalive (`bezant`)
- [ ] Unit tests against mocked HTTP (wiremock)
- [ ] Integration smoke test against IBKR paper account
- [ ] `bezant-server` — axum HTTP sidecar exposing the facade as REST
- [ ] Docker image bundling IBKR Client Portal Gateway + bezant-server
- [ ] `scripts/refresh-spec.sh` complete
- [ ] CI: clippy, fmt, tests on push
- [ ] Publish to crates.io

## v0.2 — streaming + observability

- [ ] **WebSocket support**: mirror the CPAPI's `/ws` streaming feed for
      market data, order updates, PnL snapshots. Built on top of
      `tokio-tungstenite`; reuses the session cookie from `Client`.
- [ ] **Tracing spans**: instrument every endpoint with `tracing` spans so
      users can drop in their favourite subscriber (opentelemetry, loki, etc.)
- [ ] **Retry middleware**: opinionated exponential backoff on 5xx + 429,
      pluggable via a trait.
- [ ] **Symbol cache**: ergonomic helper wrapping the `search_contract` +
      `conid` resolution dance. Respects IBKR's rate limits.
- [ ] **Paginated helpers**: `all_positions()`, `all_orders()` — progenitor-style
      streams that fetch every page so callers don't manage page indices.
- [ ] **Error context**: typed enum variants for IBKR's common failure modes
      (insufficient funds, account restricted, market closed, etc.).

## v0.3 — ecosystem

- [ ] **Python bindings** via `pyo3` — `pip install bezant` for quant scripts.
- [ ] **CLI tool**: `bezant get positions --account DU123456` for one-shot
      scripts, a sibling to `stripe` / `gh` CLIs.
- [ ] **OAuth 2.0 client-credentials flow** as IBKR rolls it out publicly
      (beta at time of writing).
- [ ] **Contract types**: options, futures, forex, fixed income, crypto — all
      generated but with convenience builders for the common cases.
- [ ] **Bracket + algo orders**: ergonomic wrappers around IBKR's Adaptive /
      TWAP / VWAP algos.
- [ ] **Upstream bug reports**: systematically file tickets with IBKR for
      every spec quirk in `scripts/normalize-spec.py`; aim to delete
      normalization steps as upstream fixes them.

## v1.0 — stable

- Stable public API. SemVer discipline.
- Production-grade docs + examples.
- Interop tests against both paper and live accounts in CI.
- Published reference implementation of an index-rebalancing bot on top.

## Contributing

PRs welcome. If you hit a new spec quirk that isn't in
`scripts/normalize-spec.py`, please open an issue with the failing
operation ID or schema name and ideally the minimal reproducer.
