# Quickstart

The fastest path from zero to a live IBKR call.

## Prerequisites

- An IBKR account (paper is fine for everything below).
- The IBKR **Client Portal Gateway** running locally. The repo ships a
  Docker compose file that packages it alongside `bezant-server`, so:

```sh
git clone https://github.com/isaacrowntree/bezant
cd bezant
docker compose up
```

Open <https://localhost:5000>, log in with your IBKR credentials + 2FA.
That's the Gateway. From here, Bezant keeps the session alive automatically.

> **macOS gotcha — port 5000.**
> macOS Sonoma and later run an **AirPlay Receiver** on `:5000` by default.
> If your Docker compose comes up but `https://localhost:5000` returns a
> mysterious `403` with `Server: AirTunes`, that's why. Either:
> 1. Disable it in *System Settings → General → AirDrop & Handoff →
>    AirPlay Receiver*, **or**
> 2. Edit `docker-compose.yml` to remap the host port:
>    `"5001:5000"` instead of `"5000:5000"`, then open
>    <https://localhost:5001> instead.

## Sanity-check via curl

```sh
curl http://localhost:8080/health
# {"authenticated":true,"connected":true,"competing":false,"message":null}

curl http://localhost:8080/accounts
# [ ... your accounts ]
```

Everything from here is optional sugar on top.

## Rust

```sh
cargo add bezant-core tokio --features tokio/full
```

Or in `Cargo.toml`:

```toml
[dependencies]
bezant-core = "0.3"
tokio = { version = "1", features = ["full"] }
```

The crate publishes its lib as `bezant`, so you `use bezant::*` regardless
of the manifest entry. There's also a `bezant::prelude` for the common
imports:

```rust,no_run
use bezant::prelude::*;
```

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    let client = bezant::Client::new("https://localhost:5000/v1/api")?;
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
    client.health().await?;

    let accounts = client
        .api()
        .get_all_accounts(bezant::api::GetAllAccountsRequest::default())
        .await?;
    println!("{accounts:#?}");
    Ok(())
}
```

## TypeScript / Node

```sh
npm install github:isaacrowntree/bezant#main:clients/typescript
```

```ts
import { Configuration, TradingPortfolioApi } from "bezant-client";

const config = new Configuration({
  basePath: "https://localhost:5000/v1/api",
});
const accounts = await new TradingPortfolioApi(config).getAllAccounts();
console.log(accounts);
```

## CLI

```sh
cargo install bezant-cli
bezant health
bezant accounts --output table
bezant positions DU123456 --output table
bezant quote AAPL
bezant orders DU123456 --output table
```

`--output {json,table}` controls the format; default is `json` for
piping into `jq`. Tabular endpoints (`accounts`, `positions`, `orders`)
get a comfy-table renderer when you pass `--output table`.

## MCP (Claude Desktop / Cursor / Continue)

```sh
cargo install bezant-mcp
```

Add to your client config:

```json
{
  "mcpServers": {
    "bezant": {
      "command": "bezant-mcp",
      "env": {
        "IBKR_GATEWAY_URL": "https://localhost:5000/v1/api"
      }
    }
  }
}
```

The LLM now has six IBKR tools: `health`, `list_accounts`,
`account_summary`, `positions`, `conid_for`, `tickle`.
