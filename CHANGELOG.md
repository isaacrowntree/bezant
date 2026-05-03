# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] — 2026-05-03

The "polish before crates.io" release. v0.2 hardened the proxy and
deploy pattern; v0.3 promotes the typed surface, observability, and
ergonomic gaps that survived. Six commits, organised into five
phases:

### Added

- **Typed `Error` variants.** ~25 `Error::Other(String)` call sites
  promoted into 11 typed variants:
  `UpstreamStatus { endpoint, status, body_preview }`, `Unknown`,
  `UrlNotABase`, `MissingQuery`, `Header`, `SymbolNotFound`,
  `BadConid`, `WsHandshake`, `WsTransport`, `WsProtocol`,
  `ResponseBuild`, plus a structured `Decode { endpoint, status, message }`.
  Callers can branch on the cause for retry / recovery instead of
  substring-matching strings.
- **`Error::is_retryable()`** — backoff loops can decide on a typed
  predicate. Transient transport errors, upstream 5xx, 429, NoSession
  and WS transport are flagged retryable; everything else (caller
  input, auth, data-shape) is not.
- **`bezant::prelude`** module re-exports the common surface (`Client`,
  `Result`, `Error`, `SymbolCache`, `KeepaliveHandle`, `AuthStatus`,
  `TickleResponse`, `Position`). `use bezant::prelude::*;` for the
  typical bot use case.
- **Per-request correlation IDs.** `tower_http::request_id::SetRequestIdLayer`
  + `PropagateRequestIdLayer`; UUID minted per request, echoed in
  the response, recorded in the `http` parent span.
- **`#[tracing::instrument]`** on every typed handler in
  `bezant-server::routes`, plus the keepalive task gets its own
  `bezant_keepalive` span via `tracing::Instrument`.
- **Graceful shutdown.** `axum::serve(...).with_graceful_shutdown(shutdown_signal())`
  drains in-flight requests on SIGTERM/SIGINT, then explicitly
  awaits `keepalive.stop()` so the tickle task closes cleanly.
- **`tower::limit::ConcurrencyLimitLayer(256)`** caps simultaneous
  handlers — a misbehaving caller can't exhaust upstream connections
  or get the IBKR account locked by hammering rate limits.
- **`KeepaliveHandle` `impl Drop`** — sends the shutdown signal so
  a forgotten handle doesn't keep tickling forever. Doc previously
  claimed "drop-to-stop" but the impl wasn't there.
- **WebSocket `Subscription` handle.** `WsClient::subscribe_*` now
  return a `Subscription` that callers cancel via
  `Subscription::cancel(&mut ws).await` — no more tracking
  (topic, conid) pairs by hand. `cancel_payload()` exposes the raw
  bytes for callers using `WsClient::split` halves.
- **`WsMessage::topic()` + `as_value()`** accessors for routing on
  message type without pattern-matching every variant.
- **`WsClient::split`** returns concrete `WsSink`/`WsRecv` type
  aliases (`futures_util::SplitSink/SplitStream` over the TLS
  stream) — callers can store the halves in struct fields without
  `Box<dyn …>`.
- **`bezant-cli --output {json,table}`** flag with `comfy-table`
  rendering for tabular endpoints (accounts, summary, positions,
  orders, health, quote). Non-tabular endpoints fall back to
  pretty-printed JSON.
- **`bezant quote SYMBOL`** subcommand — symbol → conid via cache
  → snapshot for default level-1 fields.
- **`bezant orders ACCOUNT`** subcommand — live + recently-filled
  orders; normalises both `{"orders":[...]}` and bare-array Gateway
  shapes.
- **`bezant-spec` post-normalisation invariant tests** — 14 Rust
  tests pin the postconditions each of the 13 Python normaliser
  steps establishes. CI `spec-normalise-drift` job re-runs the
  Python normaliser against the vendored output and asserts
  byte-identical output (enforces idempotency permanently).

### Changed

- **Reqwest pool tuning.** `connect_timeout(5s)` (so a dead Gateway
  surfaces fast for liveness probes), `pool_max_idle_per_host(32)`
  (was unbounded; leak risk under bursty traffic),
  `pool_idle_timeout(90s)`, `tcp_keepalive(30s)`.
- **`AuthStatus` and `TickleResponse`** marked `#[non_exhaustive]`
  so adding a field in a point release isn't a SemVer break.
- **`ClientBuilder::default()`** returns a builder pointed at
  `DEFAULT_BASE_URL` for the most common case.
- **`reqwest::StatusCode`** re-exported from `bezant-core` so
  callers using `Client::http()` don't need `reqwest` in their
  own `Cargo.toml`.
- **`AppError::into_response`** logs every mapped 4xx/5xx at
  `warn!`/`error!` so production debuggability doesn't depend on
  every handler emitting its own span event. Branches on
  `reqwest::Error::is_timeout()` / `is_connect()` for distinct
  504 / 503 / 502 status codes.
- **`/debug/probe` per-step `tokio::time::timeout(5s)`** — a hung
  Gateway no longer takes the whole probe with it.
- **`/debug/probe` body_preview redacts** `session`, `ssoConclusion`,
  and any key containing `token`/`secret` (case-insensitive)
  before exposing them. Prevents debug-token holders from scraping
  live IBKR session material via the probe surface.
- **`bezant-cli` deprecates `--reject-invalid-certs`** in favour of
  `BEZANT_VERIFY_TLS` (matches `bezant-server`). The double-negative
  was easy to leave invalid-cert acceptance on in production.
- **`bezant-cli` `paginated_positions`** emits a stderr warning when
  `MAX_POSITION_PAGES` is hit so the caller knows results may be
  truncated. Silently hitting the cap was a coverage gap.

### Tests

- Total **132+** tests across the workspace (was 97 at v0.2 release):
  - 5 inline error tests (`Send + Sync`, `is_retryable` matrix,
    Display formatting).
  - 2 keepalive tests (`stop` cleanly, `Drop` sends signal).
  - 4 redaction tests (token-key fields, nested objects, non-JSON
    pass-through).
  - 3 WS message accessor + Subscription round-trip tests.
  - 14 spec-normaliser post-condition tests.
  - 4 new CLI tests (quote, orders, `--output table` table form,
    `--output table` JSON fallback).

### Security

- Bearer/Basic `Authorization` headers no longer forwarded to
  CPGateway by `passthrough_any`. CPGateway doesn't consume them;
  forwarding is pure attack surface.
- Caller-controlled `X-Forwarded-*` / `Forwarded` / `X-Real-IP` no
  longer forwarded — caller could otherwise spoof their apparent
  source IP downstream.
- `TraceLayer`'s span records request *path* not *uri* — the URI
  carries `?token=…` for `/debug/*` calls and we don't want it in
  span fields / log shippers.

## [0.2.0] — 2026-05-03

This release hardens the production deploy story: a residential-Pi +
Cloudflare Zero Trust + WARP pattern that bypasses IBKR's Akamai
datacenter-IP rejection. See the new "Production deployment" section
in the README.

### Added
- **`/debug/probe`** diagnostic endpoint walks
  `auth/status` → `ssodh/init` → `tickle` → `portfolio/accounts`
  against the Gateway and pins the first diverging step in a top-level
  `verdict` (`ok`, `auth_status_failed`, `needs_login`, `ssodh_failed`,
  `tickle_failed`, `accounts_failed`). Skips `ssodh_init` when the
  session is already bridged so the probe is non-destructive.
- **`/debug/jar`** lists shared cookie-jar entries by name + value
  length (never raw values).
- **`BEZANT_DEBUG_TOKEN`** env var gates both `/debug/*` endpoints.
  Off → 404; on → callers must present the token via
  `X-Bezant-Debug-Token` header or `?token=…` query string.
  Constant-time comparison against the configured token.
- **`BEZANT_VERIFY_TLS`** flips on Gateway TLS cert verification
  (defaults to off because the Gateway ships with a self-signed
  cert). Replaces the double-negative `BEZANT_REJECT_INVALID_CERTS`
  whose env-var bool parsing was a footgun.
- **`BEZANT_EDGE_COOKIE_PREFIXES`** allows extending the built-in
  edge-cookie filter (Cloudflare Access, AWS ALB OIDC, OAuth2 Proxy,
  Vercel, Pomerium) with custom prefixes for bespoke Zero-Trust fronts.
- Per-arch native Docker builds (`ubuntu-24.04-arm` for arm64) cut
  multi-arch image build time from ~20min to ~5min by skipping
  QEMU emulation. Manifests stitched in a merge job.

### Changed
- `bezant-server` proxy now strips the full RFC 7230 §6.1 hop-by-hop
  header set on both request and response sides, plus `authorization`
  and `x-forwarded-*` / `forwarded` / `x-real-ip` (caller-controlled
  client-IP claims that CPGateway doesn't consume).
- Cloudflare Access cookies (`CF_Authorization`, `CF_AppSession`) are
  filtered out of inbound cookie replay so they never reach IBKR
  upstream — Akamai 401s the SSODH bridge call when it sees an
  unrecognised cookie alongside the IBKR session cookies. Generalised
  to a built-in prefix list covering the major Zero-Trust providers.
- `passthrough_any`'s upstream body read is now capped at 25 MiB via a
  streaming counter (was unbounded; OOM risk under a hostile upstream).
  Inbound side is capped at 10 MiB declaratively via
  `RequestBodyLimitLayer`.
- `bezant-server` main.rs now stacks production middleware:
  `TimeoutLayer(35s)` (>reqwest's 30s), `RequestBodyLimitLayer(10MiB)`,
  and a privacy-preserving `TraceLayer` whose spans record the request
  *path* never *uri* (to avoid logging `?token=…` query strings).
- `forward()`'s empty-body fallback for upstream chunked-decode errors
  is scoped to 1xx/204/304/3xx; on 2xx/4xx/5xx a decode failure
  surfaces as a real upstream error.
- Content-Type rewrite + missing-Content-Type default no longer fire on
  responses where the body must be empty (RFC 9110 §8.3) nor on empty-
  body 2xx/4xx/5xx responses.
- Cookie-injection log demoted from `info!` to `debug!`; path query
  string stripped from log lines.
- `bezant-core` adds `Error::BadRequest(String)` for caller-input
  failures; `bezant-server` maps it to HTTP 400 instead of 500.
- `Error::Decode` carried by `auth_status` now includes the offending
  URL and HTTP status alongside the serde error.
- Probe verdict logic now reads the **full** auth_status body (not the
  512-byte preview) to decide `_authenticated`, so a response whose
  `authenticated` field lands past the preview window doesn't silently
  trigger the destructive ssodh path.
- Cargo packaging metadata: `documentation` key on every published
  crate, per-crate `LICENSE-MIT`/`LICENSE-APACHE` files (cargo
  publish only includes per-crate dirs), `[lints] workspace = true`
  on every member, `include` directive on `bezant-spec` to control
  package size.

### Fixed
- `forward()`'s `had_content_type` flag was set before the response
  header was appended; if `HeaderValue::from_bytes` rejected the
  upstream value the response went out with no Content-Type at all.
- Multiple `Set-Cookie` headers from the Gateway now round-trip
  reliably.
- `forward()` no longer relies on `(StatusCode, HeaderMap, Vec<u8>)`'s
  `IntoResponse` adapter, which unconditionally inserted
  `application/octet-stream`.

### Security
- **HIGH:** `/debug/jar` no longer returns raw cookie values
  unauthenticated. The cookie jar holds live IBKR session cookies; an
  attacker reaching the bind address could resume the IBKR session
  and trade the account. Now name + value-length only, gated by
  `BEZANT_DEBUG_TOKEN`.
- **MEDIUM:** Bearer/Basic `Authorization` headers no longer forwarded
  to CPGateway. CPGateway doesn't consume them; forwarding lets a
  caller probe whatever auth scheme upstream might (incorrectly)
  honour.
- **MEDIUM:** Caller-controlled `X-Forwarded-For` / `Forwarded` /
  `X-Real-IP` no longer forwarded — caller could spoof their apparent
  source IP to anything that audits on those headers downstream.

### Tests
- 38 wiremock-driven integration tests in
  `crates/bezant-server/tests/routes.rs` covering the regressions
  above plus probe verdict matrix, debug-token gating
  (404/401/header/query/length-only), Cloudflare-cookie filtering,
  multi-cookie replay, hop-by-hop strip, 5xx propagation, and
  Content-Type-on-204 RFC compliance. All wiremock-driven, no
  IBKR involvement.

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
