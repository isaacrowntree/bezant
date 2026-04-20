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

## Sanity-check via curl

```sh
curl http://localhost:8080/health
# {"authenticated":true,"connected":true,"competing":false,"message":null}

curl http://localhost:8080/accounts
# [ ... your accounts ]
```

Everything from here is optional sugar on top.

## Rust

```toml
# Cargo.toml
[dependencies]
bezant = { git = "https://github.com/isaacrowntree/bezant", package = "bezant-core" }
tokio = { version = "1", features = ["full"] }
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
cargo install --git https://github.com/isaacrowntree/bezant bezant-cli
bezant health --pretty
bezant positions DU123456 --pretty
```

## MCP (Claude Desktop / Cursor / Continue)

```sh
cargo install --git https://github.com/isaacrowntree/bezant bezant-mcp
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
