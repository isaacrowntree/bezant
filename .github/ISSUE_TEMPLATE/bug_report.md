---
name: Bug report
about: Report a bug so we can reproduce and fix it
title: "bug: "
labels: ["bug", "triage"]
assignees: []
---

## Summary

<!-- 1-2 sentences describing what went wrong. -->

## Reproduction

<!-- Minimal code or shell session that triggers the bug. -->

```rust
// or:
$ bezant <command>
```

## Expected vs actual

**Expected:**

<!-- What should have happened? -->

**Actual:**

<!-- What did happen? Paste error messages, stack traces, or CLI output. -->

## Environment

- `bezant` version / commit: 
- `rustc --version`: 
- OS: 
- IBKR Gateway build (if relevant): 
- Is the bug against a **paper** or **live** account? 

## Suspected cause

- [ ] bezant-side bug (facade / server / CLI / MCP)
- [ ] codegen bug (spec normaliser / oas3-gen output)
- [ ] upstream IBKR spec bug (run with `bezant_spec::UPSTREAM_VERSION` to tag)
- [ ] not sure

## Additional context

<!-- Logs (`RUST_LOG=debug`), screenshots, anything else useful. -->
