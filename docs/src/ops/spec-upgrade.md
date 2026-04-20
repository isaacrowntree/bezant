# Upgrading the vendored spec

IBKR revises their OpenAPI spec every few weeks. The routine is short.

```sh
./scripts/refresh-spec.sh       # pulls latest from api.ibkr.com
./scripts/codegen.sh             # normalise → upgrade → regenerate bezant-api
./scripts/codegen-ts.sh          # regenerate TypeScript client
cargo test --workspace           # snapshot tests catch breaking changes
```

## Review the diff

After refresh:

```sh
git diff crates/bezant-spec/ibkr-openapi.json | less
```

Look for:

- **New operations** — may want to add convenience wrappers to
  `bezant-core` or CLI subcommands.
- **Removed operations** — downstream consumers need migration notes; drop
  mentions in the MCP tool surface and CLI.
- **Response shape changes** — the snapshot tests catch obvious breakage.
  Subtle ones (renamed fields, loosened types) may need spec-normaliser
  tweaks.

## When the build breaks

Common cause: the new spec has a quirk `scripts/normalize-spec.py` doesn't
handle yet. Workflow:

1. Try `cargo run -p xtask -- probe` to see the first error.
2. If it's an oas3-gen panic, grep rmcp/typify/progenitor-impl source for
   the assertion message to understand what's being rejected.
3. Add a targeted normalisation step with a clear comment about *why*.
4. Include the new step in `CHANGELOG.md` under "Added".
5. Ideally: open an IBKR support ticket for the upstream bug so we can
   eventually delete the normalisation.

## When tests break

The snapshot tests in `crates/bezant-core/tests/examples.rs` are the
canary. They load real IBKR example payloads and round-trip them through
the generated types. If a test fails:

- If the payload shape is **genuinely wrong** (e.g. an int field now ships
  a float), add a normaliser step that widens the type.
- If the payload is **fine** but the generator produced the wrong type,
  file a bug against `oas3-gen`.
- If the example itself is **corrupt** in the spec, file against IBKR and
  tag the example name in the spec.
