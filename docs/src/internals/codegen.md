# Codegen pipeline

Every surface except the facade is auto-generated. Here's the pipeline in
one picture:

```
api.ibkr.com/gw/api/v3/api-docs
          │
          ▼ scripts/refresh-spec.sh (curl + jq)
┌─────────────────────────┐
│ ibkr-openapi.json       │  ← vendored 3.0 spec (IBKR upstream format)
└───────────┬─────────────┘
            │
            ▼ scripts/normalize-spec.py  (13 normalisation steps)
┌─────────────────────────┐
│ ibkr-openapi.json       │  ← still 3.0, but repaired
└───────────┬─────────────┘
            │
            ▼ scripts/upgrade-to-3.1.py
┌─────────────────────────┐
│ ibkr-openapi-3.1.json   │  ← 3.1; fed to every generator
└─────┬───────────────────┘
      │                   │
      │ oas3-gen          │ openapi-generator-cli
      ▼                   ▼
┌──────────────────┐  ┌─────────────────────┐
│ bezant-api       │  │ clients/typescript  │
│ (Rust generated) │  │ (TS generated)      │
└──────────────────┘  └─────────────────────┘
```

## Running it

```sh
./scripts/refresh-spec.sh    # pull upstream (optional; run when IBKR revises)
./scripts/codegen.sh          # normalise → 3.1 → oas3-gen → bezant-api
./scripts/codegen-ts.sh       # openapi-generator-cli → clients/typescript
```

## Why this many steps

Most OpenAPI toolchains assume the spec is well-formed. Real-world broker
specs rarely are. IBKR's spec ships 13 distinct categories of quirk that
break codegen if you feed the raw spec to any generator. Documenting and
normalising each one means the generators don't need to be tuned per-quirk —
and we can upstream each normalisation as a bug report against IBKR, with
the eventual goal of deleting our normaliser entirely.

See [Spec normalisation](normalisation.md) for the full list.

## Extending to another language

Adding (say) a Go client is ~1 hour of work:

1. Pick a generator — `oapi-codegen` is idiomatic for Go.
2. Write a `scripts/codegen-go.sh` that invokes the generator against the
   **3.1 normalised** spec (`crates/bezant-spec/ibkr-openapi-3.1.json`).
3. If that generator hits new quirks, add steps to
   `scripts/normalize-spec.py` — they benefit every language, not just Go.

The normalisation tax you pay once, benefits every generator forever.
