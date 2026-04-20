# Testing strategy

34 tests across the workspace, all green in CI. Here's where they live
and what they cover.

```
┌──────────────────┬──────────────────────────────────────────────────────┐
│ Suite            │ What it proves                                       │
├──────────────────┼──────────────────────────────────────────────────────┤
│ bezant-spec  (2) │ Vendored JSON parses; UPSTREAM_VERSION matches       │
│                  │ the embedded `info.version`                          │
├──────────────────┼──────────────────────────────────────────────────────┤
│ bezant (12)      │ 6 ws::tests (URL rewriting, message classification)  │
│                  │ 6 facade tests against wiremock (auth, tickle,       │
│                  │ health, error mapping)                               │
│                  │ 4 snapshot tests (deserialise real IBKR examples)    │
├──────────────────┼──────────────────────────────────────────────────────┤
│ bezant-server    │ 7 axum integration tests against wiremock (every     │
│            (7)   │ endpoint, including error paths)                     │
├──────────────────┼──────────────────────────────────────────────────────┤
│ bezant-cli (5)   │ Spawn real compiled binary, exercise subcommands     │
│                  │ against wiremock, verify JSON output + exit codes    │
├──────────────────┼──────────────────────────────────────────────────────┤
│ bezant-mcp (4)   │ In-process MCP server over `tokio::io::duplex`,      │
│                  │ client lists tools + calls them, verify JSON round-  │
│                  │ trip and pagination                                  │
└──────────────────┴──────────────────────────────────────────────────────┘
```

## Snapshot tests from spec examples

The coolest part. `scripts/extract-examples.py` walks the vendored spec
and pulls every `examples.*.value` entry into a JSON fixture file.
`crates/bezant-core/tests/examples.rs` then round-trips each payload
through the corresponding Rust type.

This means:

- If IBKR changes a response shape, our tests break **before** our users do.
- If our spec normaliser accidentally collapses a type, the snapshot tests
  catch it.
- New coverage is ~30 seconds of work: add operation IDs to the
  `--only` list in `scripts/codegen.sh`, re-run, done.

## Mock gateway pattern

Every integration test shares this pattern:

```rust
let gateway = MockServer::start().await;
Mock::given(method("POST"))
    .and(path("/v1/api/iserver/auth/status"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))
    .mount(&gateway)
    .await;

let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
    .accept_invalid_certs(true)
    .build()?;
```

`wiremock` runs an actual HTTP server on a random port; `bezant::Client`
talks to it exactly like it would talk to the real Gateway. No mocks of
`reqwest`, no fake `Response` objects — real HTTP end-to-end.

## Running locally

```sh
cargo test --workspace                    # all 34 tests
cargo test -p bezant-core                 # just the facade
cargo test -p bezant-server --test routes # just the axum integration
```

## Adding new tests

See the existing patterns in:

- `crates/bezant-core/tests/facade.rs` — wiremock integration
- `crates/bezant-core/tests/examples.rs` — spec-example round-trips
- `crates/bezant-server/tests/routes.rs` — axum + wiremock
- `crates/bezant-cli/tests/cli.rs` — `assert_cmd` + wiremock
- `crates/bezant-mcp/tests/tools.rs` — in-process MCP round-trip
