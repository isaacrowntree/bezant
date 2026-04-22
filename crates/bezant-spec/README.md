# bezant-spec

Vendored copy of the IBKR Client Portal Web API OpenAPI specification.

Interactive Brokers publishes the spec at
<https://api.ibkr.com/gw/api/v3/api-docs> as OpenAPI 3.0. This crate pins a
specific version so consumers (notably [`bezant-api`](../bezant-api)) build
from a known, reproducible spec. The **3.0 source** is re-exported as
[`SPEC_JSON`]; the codegen-ready **3.1 derivative** (produced by
`scripts/codegen.sh`) lives at `ibkr-openapi-3.1.json` in the crate root and
is what `bezant-api` actually consumes.

## Refreshing the spec

```sh
# From the workspace root
./scripts/refresh-spec.sh   # fetches upstream 3.0, writes ibkr-openapi.json
./scripts/codegen.sh        # normalises + upgrades to 3.1, regenerates bezant-api
```

The refresh script downloads the upstream spec, writes it to
`crates/bezant-spec/ibkr-openapi.json`, and bumps `UPSTREAM_VERSION` in
`src/lib.rs`. Review the diff, regenerate `bezant-api` with
`./scripts/codegen.sh`, and commit both files together.

## License

Dual-licensed under MIT or Apache-2.0 at your option. Note that the
**vendored spec itself is IBKR's intellectual property** and is included
here under fair-use conventions for interoperability; it is **not** covered
by this crate's MIT/Apache-2.0 licenses.
