<!--
Thanks for contributing to bezant!

Keep the summary crisp. If you're touching the spec normaliser or the
generated client, include before/after stats (added/removed warnings,
snapshot-test diffs, etc.).
-->

## Summary

<!-- 1-3 bullets describing the change. -->

## Motivation

<!-- Why is this change needed? Link to an issue if applicable. -->

## Change surface

- [ ] Public API change (Rust crates)
- [ ] Spec normalisation change
- [ ] Codegen change (regenerated `bezant-api` / TypeScript)
- [ ] Docs only

## Test plan

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy -p bezant-core -p bezant-spec --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Added / updated tests covering the change
- [ ] Docs updated if public API changed

## Screenshots / output

<!-- Optional — especially useful for CLI / MCP changes. -->
