# Contributing to bezant

Thanks for your interest in bezant! This is an early-stage OSS project, so
contributions of every size are welcome — from typos in the docs to
auto-generating entire new client SDKs from the spec.

## Quick start

```sh
git clone https://github.com/isaacrowntree/bezant
cd bezant
cargo test --workspace
./scripts/codegen.sh            # regenerate Rust client from the spec
./scripts/codegen-ts.sh         # regenerate TypeScript client
```

You'll need:

- **Rust** 1.82+ (install via [rustup](https://rustup.rs/))
- **Python 3.9+** (for the spec normaliser)
- **Java 17+** (for the TypeScript codegen via `openapi-generator-cli`)
- **`oas3-gen`** — `cargo install oas3-gen`
- **jq** — for the spec-refresh script (optional; only needed if you re-download the spec)

## Repository layout

```
crates/
  bezant-spec/     — vendored IBKR OpenAPI spec + refresh tooling
  bezant-api/      — auto-generated Rust client (don't edit by hand)
  bezant-core/     — ergonomic facade (hand-written)
  bezant-server/   — axum HTTP sidecar (hand-written)
  bezant-cli/      — CLI wrapping the facade
  bezant-mcp/      — Model Context Protocol server
clients/
  typescript/      — auto-generated TS client
scripts/
  refresh-spec.sh  — pull latest spec from api.ibkr.com
  normalize-spec.py — work around IBKR spec quirks
  upgrade-to-3.1.py — OAS 3.0 → 3.1 upgrade
  codegen.sh       — normalise → upgrade → oas3-gen
  codegen-ts.sh    — openapi-generator-cli → TS client
  extract-examples.py — collect spec examples for snapshot tests
xtask/             — dev-only tools (spec probing, bisection)
docs/              — mdbook source for the docs site
```

## Pull-request checklist

Before opening a PR:

- [ ] `cargo fmt --all` — no formatting delta
- [ ] `cargo clippy -p bezant-core -p bezant-spec --all-targets -- -D warnings`
      (the generated crates have warnings we intentionally silence)
- [ ] `cargo test --workspace`
- [ ] Docs updated if you changed public API surface
- [ ] Added a test (unit, integration, or snapshot) for any behaviour change

## Spec changes

When IBKR updates their OpenAPI:

1. `./scripts/refresh-spec.sh` — pulls latest from `api.ibkr.com`.
2. `./scripts/codegen.sh` — re-normalises and regenerates `bezant-api`.
3. `cargo test --workspace` — snapshot tests catch breaking response shapes.
4. Commit `crates/bezant-spec/ibkr-openapi*.json` **and** the regenerated
   `crates/bezant-api/src/generated/` tree.

If the codegen starts failing after a spec refresh, the first place to look
is `scripts/normalize-spec.py` — we handle 13+ upstream quirks there and
new spec versions sometimes add more.

## Reporting bugs

Open an issue with:

- Your environment (`rustc --version`, OS, Gateway version)
- Minimal repro
- Whether this is a bezant bug or a suspected IBKR spec bug (they're sometimes
  hard to tell apart — share the spec version from `bezant_spec::UPSTREAM_VERSION`)

## Security

See [SECURITY.md](SECURITY.md) for the disclosure policy. Short version:
**do not open a public issue for vulnerabilities**. Email the maintainer.

## Code of Conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). tl;dr: be kind, assume good
faith, and don't be a jerk.
