# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_Nothing yet — see [0.1.0](#010--2026-04-22) for the initial surface._

## [0.1.0] — 2026-04-22

Initial public release.

### Added — crates
- `bezant-spec` — vendored IBKR Client Portal Web API OpenAPI spec (3.0
  source + 3.1-upgraded derivative) + 13-step normaliser + refresh tooling.
- `bezant-api` — auto-generated Rust client for 155 CPAPI paths
  (167 typed methods, 1030 types) emitted by `oas3-gen` from the
  normalised 3.1 spec.
- `bezant` (from `bezant-core`) — ergonomic async facade with `Client`,
  auth, keepalive, health, pagination, `SymbolCache`, and `WsClient`
  WebSocket streaming (cookie auth reused from the REST session, typed
  subscribe helpers for market data / orders / PnL).
- `bezant-server` — axum HTTP sidecar exposing the facade over plain
  REST for consumers in any language, with a catch-all passthrough for
  the Gateway's interactive login.
- `bezant-cli` — command-line tool wrapping the facade
  (`bezant health`, `bezant accounts`, `bezant positions`,
  `bezant conid`, `bezant tickle`).
- `bezant-mcp` — Model Context Protocol server exposing CPAPI as tools
  for LLM clients (Claude Desktop, Cursor, Continue, …).
- TypeScript client generated via `openapi-generator-cli` from the same
  spec for Node / Deno / browser consumers.
- Combined Docker image (`docker/combined/`) that runs the Gateway and
  `bezant-server` together behind a tini-supervised entrypoint for
  single-service deploys (Railway, fly.io, bare VMs). Standalone images
  for each process are also published.

### Added — ergonomics
- `Client::spawn_keepalive` — drop-to-stop background task tickling
  `/tickle` so the 5-minute CPAPI session never expires.
- `Client::auth_status` + `Client::health` — typed distinction between
  `NotAuthenticated`, `NoSession`, and generic errors (`auth_status`
  also translates the Gateway's pre-login 302 redirect — the spec
  claims 401 but the real Gateway never emits it).
- `Client::all_positions` — auto-paginated positions across all pages.
- `Client::cookie_jar()` — exposes the shared reqwest cookie jar so
  reverse proxies can inject incoming browser cookies and keep typed
  API calls authenticated.
- `#[tracing::instrument]` spans across every facade method.

### Added — repo / release hygiene
- Runnable examples under `crates/bezant-core/examples/` — `health`,
  `list_positions`, `stream_quotes` — copy-paste ready against the
  bundled Docker gateway via env vars.
- `[package.metadata.docs.rs]` on every library crate — docs.rs builds
  with `--cfg docsrs` for future feature-gate markers.
- Centralised lint floor via `[workspace.lints]` —
  `unsafe_code = forbid`, `missing_docs = warn`,
  `rust_2018_idioms` / `unreachable_pub` on warn — inherited by every
  hand-written crate.
- CI: fmt, clippy (warnings as errors), tests on stable + beta
  (ubuntu + macOS), MSRV check at Rust 1.89, TypeScript client build,
  `cargo-deny` audit, docs build to GitHub Pages.
- Snapshot tests driven by real IBKR example payloads in the spec.
- 34 tests across the workspace (unit, integration, snapshot).

### Notes
- MSRV: Rust **1.89** (driven by transitive deps — `oas3-gen-support`,
  `progenitor`, `serde_with`, `time`).
- Rust codegen pivoted from `progenitor` to `oas3-gen` after
  `progenitor` produced 49 compile errors on IBKR's spec; `oas3-gen`
  builds cleanly after the 13-step normalisation pipeline.
- Dual MIT / Apache-2.0 licensing throughout; the vendored IBKR spec
  itself remains IBKR's IP and is included under fair-use conventions
  for interoperability.
