# bezant-spec

Vendored copy of the IBKR Client Portal Web API OpenAPI 3.0 specification.

Interactive Brokers publishes the spec at
<https://api.ibkr.com/gw/api/v3/api-docs>. This crate pins a specific version
so consumers (notably [`bezant-api`](../bezant-api)) build from a known,
reproducible spec.

## Refreshing the spec

```sh
# From the workspace root
./scripts/refresh-spec.sh
```

The script downloads the upstream spec, writes it to
`crates/bezant-spec/ibkr-openapi.json`, and bumps `UPSTREAM_VERSION` in
`src/lib.rs`. Review the diff, regenerate `bezant-api` with `cargo build`,
and commit.

## License

Dual-licensed under MIT or Apache-2.0 at your option. Note that the
**vendored spec itself is IBKR's intellectual property** and is included
here under fair-use conventions for interoperability; it is **not** covered
by this crate's MIT/Apache-2.0 licenses.
