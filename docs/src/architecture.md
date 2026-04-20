# Architecture overview

Bezant is five surfaces over one vendored spec. Understanding how they
compose is the key to picking the right one for your use case.

```
┌────────────────────────────────────────────────────────────────────────┐
│  vendored OpenAPI 3.1 spec (bezant-spec)                               │
│  ─ normalise (13 steps in scripts/normalize-spec.py)                   │
│  ─ upgrade 3.0 → 3.1                                                   │
└──────────┬───────────────────────────────────┬─────────────────────────┘
           │                                   │
           │ oas3-gen                          │ openapi-generator-cli
           ▼                                   ▼
┌─────────────────────┐                ┌─────────────────────┐
│  bezant-api (Rust)  │                │  TypeScript client  │
│  167 methods        │                │  npm / Deno / fetch │
│  1030 types         │                └─────────────────────┘
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  bezant (facade)    │  keepalive · health · pagination · SymbolCache
│                     │  · WsClient · tracing spans · typed errors
└──────────┬──────────┘
           │
           ▼
     ┌─────┴─────┬──────────┬──────────┐
     ▼           ▼          ▼          ▼
┌─────────┐ ┌────────┐ ┌────────┐ ┌──────────┐
│ your    │ │ bezant │ │ bezant │ │ bezant   │
│ Rust bot│ │-cli    │ │-server │ │-mcp      │
└─────────┘ └────────┘ └────────┘ └──────────┘
                         (HTTP)     (MCP)
                           │         │
                           ▼         ▼
                        any lang    LLMs
```

## What lives where

- **`bezant-spec`** — the IBKR OpenAPI spec as IBKR publishes it, plus the
  normalisation script. **Nothing else touches the raw spec directly.**
- **`bezant-api`** — auto-generated Rust client. Don't hand-edit; re-run
  `./scripts/codegen.sh` to refresh.
- **`bezant` (core)** — the ergonomic layer you actually want to use from
  Rust. Wraps `bezant-api` but adds session management, pagination,
  WebSockets, typed errors.
- **`bezant-server`** — an axum HTTP sidecar. Mostly an *untyped*
  pass-through (it forwards the Gateway's JSON verbatim) — this is
  deliberate, see [Why pass-through](ops/cloud.md#why-pass-through).
- **`bezant-cli`** — clap wrapper over the facade. No TCP listener; spawns
  one Gateway connection per invocation.
- **`bezant-mcp`** — rmcp-backed server exposing CPAPI as MCP tools.

## Why so many surfaces?

Because the *same* spec gives us two axes of generation for free:

1. **Transport axis** — Rust native (`bezant-api`), HTTP REST (`bezant-server`),
   stdio MCP (`bezant-mcp`), CLI (`bezant-cli`), TypeScript fetch
   (`clients/typescript`).
2. **Abstraction axis** — raw CPAPI access (`bezant-api`) vs ergonomic
   facade (`bezant`) vs pass-through proxy (`bezant-server`).

Each surface picks a point on these axes that suits a specific consumer:

- Rust bot directly linking the library → `bezant` + `bezant-api`
- Node / Python bot hitting HTTP → `bezant-server`
- Shell / cron jobs → `bezant-cli`
- LLM chat workflows → `bezant-mcp`
- Browser / Deno → TypeScript client

## The spec is the contract

Anything that needs to change the wire format **changes the vendored
spec**, then re-runs codegen. The generated crate is never hand-edited.
This keeps all five surfaces in lock-step and means an IBKR spec update
propagates to every surface with one command.
