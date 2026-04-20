# Spec normalisation

`scripts/normalize-spec.py` takes the IBKR upstream spec and applies a
series of surgical transforms so every downstream generator can consume it
cleanly.

Each transform is a distinct, upstreamable fix — the end goal is that IBKR
fixes these in their spec and we can delete the corresponding normalisation
step.

## The 13 steps (current)

1. **Strip null security scopes.** IBKR emits `security[].scheme: [null]`
   where OpenAPI 3.0 requires `[]` or `[scope-string]`.
2. **Synthesise missing `operationId`s.** Progenitor and oas3-gen both
   require every operation to have one; IBKR omits them on ~50 operations.
3. **Disambiguate duplicate `operationId`s.** IBKR ships at least one
   duplicate (`getTradingSchedule` × 2 on different paths). We append a
   path-derived suffix to later occurrences.
4. **Desugar ambiguous enum variants.** Enums whose values collapse into
   non-unique Rust identifiers after sanitisation (`>=`, `<=`, `>`, `<`,
   `==`) are downgraded to plain `type: string` with the variants captured
   in the `description`.
5. **Rewrite exotic content types.** IBKR uses `application/jwt` in a few
   places; we rewrite to `text/plain` with a string-typed schema.
6. **Reconcile enum values with declared `type`.** Example: a field
   declared `type: number` with enum `["0", "1", "2"]` gets the enum values
   coerced to numbers.
7. **Demote misplaced path parameters.** Several operations declare `in:
   path` parameters whose placeholder isn't in the URL template. We demote
   them to `in: query`.
8. **Drop unknown string formats.** `format: "jwt"` isn't a standard string
   format; we strip it so generators don't emit broken wrappers.
9. **Demote cookie parameters to headers.** Progenitor doesn't support
   `in: cookie`; we rewrite to `in: header`.
10. **Collapse multi-content-type success responses.** When IBKR offers a
    200 response in both `application/json` and `application/pdf`, we pick
    JSON and drop the rest so progenitor's assertion holds.
11. **Drop WebSocket upgrade operations.** Operations with only `1xx`
    responses (e.g. `101 Switching Protocols`) can't be modelled as REST.
12. **Stringify numeric-array query parameters.** oas3-gen's
    `StringWithCommaSeparator` only handles strings; array-of-integer query
    params get their items coerced to strings.
13. **Widen `integer` fields with float example values.** IBKR declares
    `SMA`, `balance`, `accruedInterest` etc. as `integer` but ships their
    example payloads as `368538.0`. The snapshot tests catch this and the
    normaliser widens the field to `number` automatically.

## The spec-example-widening story

Step 13 was discovered by the snapshot tests in `bezant-core/tests/examples.rs`.
Those tests round-trip real IBKR example payloads through the generated Rust
types. The first run failed on `SMA: 368538.0` because the type was `i32`.

Rather than papering over it with a manual cast, we made the normaliser
smarter: walk every example, find every integer-typed field with a float
value, widen the schema to `number`. 37 fields get widened per codegen run
now.

This is the canonical pattern: **a failing test should prompt a
normalisation step, not a hand-patch**. It catches future IBKR drift
without human attention.
