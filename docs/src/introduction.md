# bezant

> **Typed async access to the Interactive Brokers Client Portal Web API —
> Rust-first, with HTTP / CLI / MCP / TypeScript surfaces auto-generated
> from the same vendored OpenAPI spec.**

[![crates.io](https://img.shields.io/crates/v/bezant-core?label=bezant-core)](https://crates.io/crates/bezant-core)
[![docs.rs](https://img.shields.io/docsrs/bezant-core)](https://docs.rs/bezant-core)
[![CI](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml/badge.svg)](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](license.md)

Bezant turns IBKR's 154-endpoint CPAPI into **five ergonomic surfaces**
that all ship from the same vendored OpenAPI 3.1 spec:

| Crate / package | Install | What it's for |
|---|---|---|
| [`bezant-core`](https://crates.io/crates/bezant-core) | `cargo add bezant-core` | Typed async Rust client. Keepalive, WebSocket streaming, pagination, symbol cache, [11 typed error variants](https://docs.rs/bezant-core/latest/bezant/enum.Error.html), `is_retryable()` predicate, `Subscription` cancel handles |
| [`bezant-server`](https://crates.io/crates/bezant-server) | `cargo install bezant-server` | axum HTTP sidecar exposing CPAPI as plain REST+JSON for any language. Production-hardened: CF Access cookie filtering, edge-cookie strip, request-ID propagation, graceful shutdown, concurrency cap |
| [`bezant-cli`](https://crates.io/crates/bezant-cli) | `cargo install bezant-cli` | `bezant accounts`, `bezant positions DU123`, `bezant quote AAPL`, `bezant orders DU123` — `--output {json,table}` |
| [`bezant-mcp`](https://crates.io/crates/bezant-mcp) | `cargo install bezant-mcp` | Model Context Protocol server — Claude / Cursor / Continue can call IBKR tools |
| [TypeScript client](../clients/typescript) | `npm install` from repo | Auto-generated TS client for Node / Deno / browser |

All five drive off the same vendored [IBKR OpenAPI spec][spec]. Re-run
`./scripts/codegen.sh` when IBKR ships a new revision and every surface
updates together — verified by 14 normaliser-invariant tests + a
CI drift check.

## What's special about it

- **Production-grade IBKR deploy story.** Out of the box, every cloud
  IBKR API client hits the same wall: `api.ibkr.com` (Akamai-fronted)
  rejects datacenter egress IPs. bezant ships [a documented
  Cloudflare Zero Trust + residential-Pi recipe](ops/pi-cloudflare.md)
  that bypasses it without code changes — same image runs on Railway
  *or* a Pi at home with no fork.
- **Single-tenant proxy by design.** `bezant-server` is honest about
  its trust model: one shared cookie jar, one IBKR account. No
  surprising fan-out semantics, no opaque session sharing.
- **Edge-aware cookie handling.** Drops `CF_Authorization`,
  `CF_AppSession`, AWS ALB OIDC, OAuth2 Proxy, Vercel JWT, Pomerium
  cookies before they reach IBKR — Akamai 401s on unrecognised
  cookies and we don't want your bot to inherit that surprise.
- **Per-request observability.** Every typed handler is
  `#[tracing::instrument]`'d, every request gets a UUID `x-request-id`
  echoed in the response, every mapped 4xx/5xx logs at the boundary
  with the typed error variant.
- **Diagnostic probe.** `/debug/probe` (token-gated) walks
  `auth_status → ssodh_init → tickle → accounts` and pins the first
  diverging step in a top-level verdict. Built specifically to
  discriminate "proxy regression" vs "upstream IBKR rejection" so
  you don't waste hours debugging code that's working.

## Where to go next

| Goal | Page |
|---|---|
| **Get something running locally** | [Quickstart](quickstart.md) |
| **Understand the layered design** | [Architecture overview](architecture.md) |
| **Deploy to production** | [Cloudflare Zero Trust + Pi](ops/pi-cloudflare.md) |
| **Use the Rust crate** | [Rust crate](usage/rust.md) |
| **Use the HTTP sidecar from non-Rust** | [HTTP sidecar](usage/server.md) |
| **Use the CLI** | [Command-line](usage/cli.md) |
| **Wire up an MCP client** | [MCP server](usage/mcp.md) |
| **Refresh the spec / regen clients** | [Codegen pipeline](internals/codegen.md) |
| **Contribute** | [Contributing](contributing.md) |

## Status

**Alpha — v0.3.** Production-deployed against IBKR live + paper accounts;
the public API surface will continue to evolve until v1.0. See the
[ROADMAP](reference/roadmap.md) for what's shipped and what's next.

## Not affiliated with Interactive Brokers

Bezant is an independent open-source project. Trading involves substantial
risk; this software is provided **without warranty**. See the [license](license.md).

[spec]: https://api.ibkr.com/gw/api/v3/api-docs
