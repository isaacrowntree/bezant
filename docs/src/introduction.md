# bezant

> Typed async access to the Interactive Brokers Client Portal Web API —
> Rust-first, with HTTP / CLI / MCP / TypeScript surfaces generated from the
> same vendored OpenAPI spec.

Bezant turns IBKR's 155-endpoint CPAPI into:

| Surface | What it's for |
|---|---|
| `bezant-core` (Rust) | Typed async client for Rust trading bots |
| `bezant-server` | HTTP sidecar so any language can consume CPAPI over plain REST |
| `bezant-cli` | `bezant get-accounts`, `bezant positions DU123`, etc. |
| `bezant-mcp` | Model Context Protocol server — Claude / Cursor can call IBKR as tools |
| TypeScript client | `npm install bezant-client` for Node / browser |

All five are driven off the same vendored [IBKR OpenAPI spec][spec]. When
IBKR revises their spec, you re-run `./scripts/codegen.sh` and every surface
is refreshed together.

## Where to go next

- **Just want to try it out?** → [Quickstart](quickstart.md)
- **Want to understand the design?** → [Architecture overview](architecture.md)
- **Want to contribute?** → [Contributing](contributing.md)
- **Curious how the spec normalisation works?** → [Codegen pipeline](internals/codegen.md)

## Status

**Alpha.** Everything works end-to-end against IBKR paper accounts, but the
API surface will evolve until we hit v1.0. See the
[ROADMAP](reference/roadmap.md) for what's next.

[spec]: https://api.ibkr.com/gw/api/v3/api-docs
