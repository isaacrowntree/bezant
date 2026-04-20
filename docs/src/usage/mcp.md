# MCP server (`bezant-mcp`)

A Model Context Protocol server that exposes IBKR read-only endpoints as
structured tools an LLM can call. Runs over stdio, so it plugs into any
MCP-compatible client (Claude Desktop, Cursor, Continue, Claude Code).

## Why MCP

LLM-driven trading assistants only work if they read your **live** account
state. MCP gives the model a narrow, typed API: the model asks for
"account_summary", the protocol delivers fresh JSON from IBKR. No more
hallucinated NAV numbers.

## Tool surface (v0.1)

All read-only. Order placement lives behind a feature flag in later
releases — MCP tools are powerful and we don't want a chat window
accidentally firing orders.

| Tool | Purpose |
|---|---|
| `health` | Is the Gateway authenticated + connected? |
| `list_accounts` | All IBKR account IDs on the Gateway session |
| `account_summary` | NAV, cash, buying power, margin detail |
| `positions` | Every open position for an account (pagination handled) |
| `conid_for` | Resolve ticker → IBKR contract id (memoised) |
| `tickle` | Manually extend the session |

## Install

```sh
cargo install --git https://github.com/isaacrowntree/bezant bezant-mcp
```

## Configure Claude Desktop

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

Restart Claude Desktop. Ask: *"What accounts do I have?"* and the LLM
should call `list_accounts` automatically.

## Configure any other MCP client

Spawn `bezant-mcp` as a stdio subprocess. Environment variables from the
shell are inherited. Logs go to stderr — **don't redirect stdout**; that's
the MCP protocol channel.

## Safety

- Every tool is read-only in v0.1.
- The server inherits `bezant`'s session keepalive, so it won't spam IBKR
  with reconnects.
- Tool descriptions explicitly warn the LLM when it needs to call `health`
  before anything else.
- Future order-placement tools will require `BEZANT_MCP_ALLOW_ORDERS=1`
  plus fresh confirmation for every call — no silent trading.
