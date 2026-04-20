//! Library half of `bezant-mcp` — the `BezantMcp` struct, its tool router,
//! and the `ServerHandler` impl. Lives here so integration tests can
//! exercise the tools via an in-memory MCP transport without shelling out
//! to the built binary.

#![allow(missing_docs)]

use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::{schemars, tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler};
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone)]
pub struct BezantMcp {
    pub client: bezant::Client,
    pub cache: Arc<bezant::SymbolCache>,
    pub tool_router: ToolRouter<BezantMcp>,
}

impl BezantMcp {
    /// Construct a handler around a prepared [`bezant::Client`] and cache.
    pub fn new(client: bezant::Client, cache: Arc<bezant::SymbolCache>) -> Self {
        Self {
            client,
            cache,
            tool_router: BezantMcp::tool_router(),
        }
    }
}

// ---------- Tool argument types ----------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AccountIdArgs {
    /// IBKR account identifier (e.g. `DU123456` for paper, `U123456` for live).
    pub account_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SymbolArgs {
    /// Ticker symbol, e.g. `AAPL`, `BRK B`, `GLD`.
    pub symbol: String,
}

// ---------- Tool implementations ----------

#[tool_router]
impl BezantMcp {
    /// Report Gateway session status.
    #[tool(
        description = "Check whether the IBKR Client Portal Gateway is authenticated and connected. Returns the authenticated/connected/competing flags and any message. Always call this first if other tools start returning 'not authenticated' errors."
    )]
    pub async fn health(&self) -> Result<CallToolResult, McpError> {
        let status = self
            .client
            .auth_status()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json_result(serde_json::json!({
            "authenticated": status.authenticated,
            "connected": status.connected,
            "competing": status.competing,
            "message": status.message,
        })))
    }

    /// List every account.
    #[tool(
        description = "List every IBKR account the Gateway is aware of. Call this to discover account IDs before asking about positions or summary."
    )]
    pub async fn list_accounts(&self) -> Result<CallToolResult, McpError> {
        let raw = self
            .fetch_json(&["portfolio", "accounts"])
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json_result(raw))
    }

    /// Portfolio summary (NAV, cash, buying power).
    #[tool(
        description = "Get the portfolio summary for one account: net liquidation value, total cash, buying power, margin details, etc."
    )]
    pub async fn account_summary(
        &self,
        Parameters(args): Parameters<AccountIdArgs>,
    ) -> Result<CallToolResult, McpError> {
        let raw = self
            .fetch_json(&["portfolio", args.account_id.as_str(), "summary"])
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json_result(raw))
    }

    /// All positions (paginated).
    #[tool(
        description = "Return every open position for one account. Pagination is handled automatically."
    )]
    pub async fn positions(
        &self,
        Parameters(args): Parameters<AccountIdArgs>,
    ) -> Result<CallToolResult, McpError> {
        let mut all = Vec::new();
        for page in 0..100u32 {
            let page_str = page.to_string();
            let raw = self
                .fetch_json(&[
                    "portfolio",
                    args.account_id.as_str(),
                    "positions",
                    &page_str,
                ])
                .await
                .map_err(|e| McpError::internal_error(e.to_string(), None))?;
            let arr = raw.as_array().cloned().unwrap_or_default();
            let n = arr.len();
            all.extend(arr);
            if n < 30 {
                break;
            }
        }
        Ok(json_result(Value::Array(all)))
    }

    /// Ticker → conid resolution, cached.
    #[tool(
        description = "Resolve a ticker symbol to IBKR's numeric contract id (conid). Needed before most market-data or order operations. Results are memoised across calls in this MCP session."
    )]
    pub async fn conid_for(
        &self,
        Parameters(args): Parameters<SymbolArgs>,
    ) -> Result<CallToolResult, McpError> {
        let conid = self
            .cache
            .conid_for(&args.symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json_result(serde_json::json!({
            "symbol": args.symbol,
            "conid": conid,
        })))
    }

    /// Tickle / session keepalive.
    #[tool(
        description = "Send a tickle to keep the IBKR session alive. The server already keeps the session alive automatically; this tool is mainly useful if you want to extend it explicitly."
    )]
    pub async fn tickle(&self) -> Result<CallToolResult, McpError> {
        let tickle = self
            .client
            .tickle()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(json_result(
            serde_json::json!({ "session": tickle.session }),
        ))
    }
}

impl BezantMcp {
    /// Fetch a JSON body by walking the Gateway's path segments.
    async fn fetch_json(&self, segments: &[&str]) -> anyhow::Result<Value> {
        let mut url = self.client.base_url().clone();
        {
            let mut segs = url
                .path_segments_mut()
                .map_err(|()| anyhow::anyhow!("base url cannot be a base"))?;
            for s in segments {
                segs.push(s);
            }
        }
        let resp = self.client.http().get(url).send().await?;
        let status = resp.status();
        let bytes = resp.bytes().await?;
        if !status.is_success() {
            anyhow::bail!(
                "HTTP {status}: {body}",
                body = String::from_utf8_lossy(&bytes)
            );
        }
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[tool_handler]
impl ServerHandler for BezantMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
    }
}

fn json_result(value: Value) -> CallToolResult {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".into());
    CallToolResult::success(vec![Content::text(text)])
}
