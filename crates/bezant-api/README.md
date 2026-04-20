# bezant-api

Auto-generated Rust client for the IBKR Client Portal Web API, built from the
OpenAPI 3.0 spec vendored in [`bezant-spec`](../bezant-spec).

This crate gives you **raw, 1:1 access** to every CPAPI endpoint and type.
If you want convenience wrappers — keepalive loops, pagination, symbol
caching — use the higher-level [`bezant`](../bezant-core) crate instead.

## Usage

```rust,no_run
use bezant_api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http = reqwest::Client::builder()
        .cookie_store(true)
        .danger_accept_invalid_certs(true) // Gateway default self-signed cert
        .build()?;
    let client = Client::new_with_client("https://localhost:5000/v1/api", http);

    // All 155 CPAPI endpoints are available as typed async methods.
    let accounts = client.portfolio_accounts_get().send().await?;
    println!("{:#?}", accounts);

    Ok(())
}
```

## Regeneration

The generated client is built by `build.rs` from the pinned spec in
[`bezant-spec`](../bezant-spec). To refresh the spec from IBKR upstream:

```sh
./scripts/refresh-spec.sh
cargo build -p bezant-api
```

## License

Dual-licensed under MIT or Apache-2.0 at your option.
