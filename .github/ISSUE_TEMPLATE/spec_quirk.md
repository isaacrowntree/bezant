---
name: IBKR spec quirk
about: Report a bug in IBKR's upstream OpenAPI spec that bezant's normaliser should handle
title: "spec: "
labels: ["spec-normaliser", "triage"]
assignees: []
---

## Upstream spec version

<!-- `bezant_spec::UPSTREAM_VERSION` or the `info.version` field of the vendored JSON. -->

## What went wrong

<!-- Describe the codegen / compile / runtime failure, or the response shape that doesn't deserialise. -->

## Minimal repro

<!-- Preferably: the path, operationId, schema, or example value in the spec that's broken. -->

```json
// paste the offending snippet
```

## Proposed normalisation

<!-- If you have an idea for a step to add to scripts/normalize-spec.py, describe it here. -->

## Should we also file against IBKR?

- [ ] Yes, this looks like an upstream bug they should fix
- [ ] No, this is a mismatch with Rust / OpenAPI conventions but valid OpenAPI
- [ ] Unclear
