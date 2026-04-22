# bezant-api

Auto-generated Rust client for the IBKR Client Portal Web API, emitted by
[`oas3-gen`](https://crates.io/crates/oas3-gen) from the OpenAPI 3.1 spec
derived in [`bezant-spec`](../bezant-spec).

This crate gives you **raw, 1:1 access** to every CPAPI endpoint and type.
If you want convenience wrappers — keepalive loops, pagination, symbol
caching — use the higher-level [`bezant`](../bezant-core) crate instead.

## Usage

```rust,no_run
use bezant_api::{IbRestApiClient, GetAllAccountsRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http = reqwest::Client::builder()
        .cookie_store(true)
        .danger_accept_invalid_certs(true) // Gateway default self-signed cert
        .build()?;
    let client = IbRestApiClient::with_client("https://localhost:5000/v1/api", http)?;

    // All 155 CPAPI endpoints are available as typed async methods.
    let accounts = client
        .get_all_accounts(GetAllAccountsRequest::default())
        .await?;
    println!("{:#?}", accounts);

    Ok(())
}
```

## Regeneration

The generated client is built by `scripts/codegen.sh` from the pinned spec
in [`bezant-spec`](../bezant-spec). To refresh the spec and regenerate:

```sh
./scripts/refresh-spec.sh   # pulls the latest upstream spec, diffs it
./scripts/codegen.sh        # normalises, upgrades 3.0 → 3.1, regenerates
cargo build -p bezant-api
```

## License

Dual-licensed under MIT or Apache-2.0 at your option.
