# bezant-mcp

Model Context Protocol server for the IBKR Client Portal Web API.

Expose your IBKR account to Claude / GPT / Cursor / Continue as structured
tools, so the model can read account state, pull positions, resolve tickers
to contract ids — the building blocks an LLM-driven trading workflow needs.

Built on top of [`bezant`](../bezant-core): inherits the session keepalive,
TLS handling, and typed errors. Talks MCP over stdio, so it plugs into any
MCP-compatible client with zero network configuration.

## Why MCP for a broker

Trading assistants that actually work read your *actual* positions, not
whatever the model last saw in training. An MCP tool surface gives the LLM
a safe, narrow API: the model asks for "account summary" and the protocol
delivers live JSON from IBKR. No more hallucinated NAV numbers.

## Tool surface (v0.1)

All read-only. Order placement lives behind a feature flag in later releases.

| Tool | Purpose |
|---|---|
| `health` | Is the Gateway authenticated + connected? |
| `list_accounts` | All IBKR account IDs on the Gateway session |
| `account_summary` | NAV, cash, buying power, margin detail |
| `positions` | Every open position for an account (pagination handled) |
| `conid_for` | Resolve ticker → IBKR contract id (memoised) |
| `tickle` | Manually extend the session (usually not needed) |

## Install

```sh
cargo install --git https://github.com/isaacrowntree/bezant bezant-mcp
```

Prerequisite: IBKR Client Portal Gateway running locally (the repo's
`docker compose up` covers this).

## Configure

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

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

### Cursor / Continue / any MCP-aware client

The binary accepts MCP JSON-RPC messages on stdin, responds on stdout. Any
client that can spawn a stdio MCP server can use it. Pass environment
variables through the client's config.

## Environment

| Variable                        | Default                           | Meaning                                  |
|---------------------------------|-----------------------------------|------------------------------------------|
| `IBKR_GATEWAY_URL`              | `https://localhost:5000/v1/api`   | Gateway base URL                         |
| `BEZANT_REJECT_INVALID_CERTS`   | unset (accepts self-signed)       | Set to `1` to enforce cert validation    |
| `BEZANT_MCP_KEEPALIVE_SECS`     | `60`                              | Seconds between `/tickle` pings          |
| `RUST_LOG`                      | `bezant_mcp=info,bezant=info`     | Standard tracing env-filter              |

Logs go to **stderr** so they don't collide with the MCP protocol on stdout.

## Roadmap

- **v0.2**: market data snapshot + historical bars tools
- **v0.3**: order placement (gated by env var, with scoped tool permissions)
- **v0.4**: streaming / subscription tools via MCP resources

## License

Dual-licensed under MIT or Apache-2.0 at your option.
