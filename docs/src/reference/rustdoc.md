# Rust API reference (rustdoc)

Every public item in every crate is documented. Three ways to read it:

- **Live, browsable:** the CI deploys `cargo doc` output alongside this
  book under `/rustdoc/`. You're probably reading the book at
  `https://isaacrowntree.github.io/bezant/` — jump to
  [`/rustdoc/bezant/`](../rustdoc/bezant/index.html) for the facade crate.
- **Local:** `cargo doc --workspace --no-deps --open`
- **docs.rs (once published):** will link here once the crates hit
  crates.io.

## Per-crate entry points

- [`bezant`](../rustdoc/bezant/index.html) — ergonomic facade
- [`bezant_api`](../rustdoc/bezant_api/index.html) — auto-generated client
- [`bezant_spec`](../rustdoc/bezant_spec/index.html) — vendored spec
- [`bezant_server`](../rustdoc/bezant_server/index.html) — HTTP sidecar lib
- [`bezant_mcp`](../rustdoc/bezant_mcp/index.html) — MCP tool surface

## Conventions

- Every public function has a one-line summary in the first sentence, a
  longer explanation where useful, and `# Errors` / `# Panics` sections
  per the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Examples compile as doctests (try `cargo test --workspace --doc`).
- Generated crates (`bezant-api`) have auto-derived docs from the spec's
  `description` fields — so the same docstrings ship to docs.rs that IBKR
  writes themselves.
