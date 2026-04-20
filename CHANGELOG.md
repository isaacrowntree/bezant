# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `bezant-cli` — command-line tool wrapping the facade (`bezant health`,
  `bezant accounts`, `bezant positions`, `bezant conid`, `bezant tickle`).
- `bezant-mcp` — Model Context Protocol server exposing CPAPI as tools for
  LLM clients (Claude Desktop, Cursor, Continue, etc.).
- `bezant::WsClient` — WebSocket streaming with cookie auth from the REST
  session, typed subscribe helpers for market data / orders / PnL.
- `bezant::SymbolCache` — memoised ticker → conid resolver.
- `bezant::Client::all_positions` — auto-paginated positions across all pages.
- `#[tracing::instrument]` spans across every facade method for observability.
- Snapshot tests driven by real IBKR example payloads in the spec.
- Spec normaliser step to widen `integer` fields that carry float example
  values — catches a real class of IBKR spec bugs automatically.
- TypeScript client generated via `openapi-generator-cli` from the same spec.
- Docker image + `docker-compose.yml` bundling the Gateway + sidecar.
- 34 tests across the workspace (unit, integration, snapshot).

### Changed
- Pivoted Rust codegen from `progenitor` to `oas3-gen` after `progenitor`
  produced 49 compile errors on IBKR's spec; `oas3-gen` builds cleanly
  after the spec normalisation pipeline.
- Upgraded vendored spec from OpenAPI 3.0 to 3.1 to enable `oas3-gen`.

## [0.1.0] — 2026-04-21

Initial release.

### Added
- `bezant-spec` — vendored IBKR Client Portal Web API OpenAPI spec + 13-step
  normaliser + refresh tooling.
- `bezant-api` — auto-generated Rust client for 155 CPAPI paths (167 typed
  methods, 1030 types).
- `bezant` (core) — ergonomic facade with `Client`, auth, keepalive, health.
- `bezant-server` — axum HTTP sidecar exposing the facade over REST.
- GitHub Actions CI (fmt + clippy + check + test).
- Dual MIT / Apache-2.0 licensing.
