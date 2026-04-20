//! Integration tests that exercise the `BezantMcp` tool surface through
//! the real MCP protocol over an in-memory transport pair.
//!
//! We spin up a wiremock Gateway, build a `BezantMcp` handler against it,
//! run the MCP server on one end of a `tokio::io::duplex` pipe, and connect
//! a vanilla MCP client to the other end. Then we exercise each tool as a
//! real client would.

use std::sync::Arc;

use rmcp::model::CallToolRequestParams;
use rmcp::{service::RunningService, ClientHandler, ServiceExt};
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Clone, Default)]
struct NoopClient;

// ClientHandler's blanket `get_info` returns a sensible default, so we
// don't need to override it for these tests.
impl ClientHandler for NoopClient {}

async fn start_pair(gateway: &MockServer) -> RunningService<rmcp::RoleClient, NoopClient> {
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("bezant client");
    let cache = Arc::new(bezant::SymbolCache::new(client.clone()));
    let handler = bezant_mcp::BezantMcp::new(client, cache);

    let (server_io, client_io) = tokio::io::duplex(1024 * 64);

    tokio::spawn(async move {
        if let Ok(service) = handler.serve(server_io).await {
            let _ = service.waiting().await;
        }
    });

    NoopClient
        .serve(client_io)
        .await
        .expect("client initialisation")
}

fn first_text_content(result: &rmcp::model::CallToolResult) -> &str {
    for item in &result.content {
        if let Some(text) = item.as_text() {
            return text.text.as_str();
        }
    }
    panic!("no text content in tool result");
}

#[tokio::test]
async fn tools_list_includes_expected_tools() {
    let gateway = MockServer::start().await;
    let client = start_pair(&gateway).await;
    let tools = client.list_all_tools().await.expect("list_tools");
    let names: Vec<_> = tools.iter().map(|t| t.name.to_string()).collect();
    for expected in [
        "health",
        "list_accounts",
        "account_summary",
        "positions",
        "conid_for",
        "tickle",
    ] {
        assert!(
            names.iter().any(|n| n == expected),
            "tool {expected} missing from {names:?}"
        );
    }
    client.cancel().await.ok();
}

#[tokio::test]
async fn health_tool_projects_gateway_response() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": "ok"
        })))
        .mount(&gateway)
        .await;

    let client = start_pair(&gateway).await;
    let result = client
        .call_tool(CallToolRequestParams::new("health"))
        .await
        .expect("health");

    let body: Value = serde_json::from_str(first_text_content(&result)).expect("json");
    assert_eq!(body["authenticated"], json!(true));
    assert_eq!(body["connected"], json!(true));
    client.cancel().await.ok();
}

#[tokio::test]
async fn account_summary_tool_passes_through_path_argument() {
    let gateway = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "netliquidation": {"amount": 999.0, "currency": "USD"}
        })))
        .mount(&gateway)
        .await;

    let client = start_pair(&gateway).await;
    let args = serde_json::Map::from_iter([("account_id".into(), Value::String("DU123".into()))]);
    let mut params = CallToolRequestParams::new("account_summary");
    params.arguments = Some(args);
    let result = client.call_tool(params).await.expect("account_summary");

    let body: Value = serde_json::from_str(first_text_content(&result)).expect("json");
    assert_eq!(body["netliquidation"]["amount"], json!(999.0));
    client.cancel().await.ok();
}

#[tokio::test]
async fn positions_tool_paginates_until_short_page() {
    let gateway = MockServer::start().await;
    // Full page on 0, short page on 1 → should stop after page 1.
    let page0: Vec<Value> = (0..30)
        .map(|i| json!({"conid": 1000 + i, "position": 1.0}))
        .collect();
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Value::Array(page0)))
        .mount(&gateway)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{"conid": 9999}])))
        .mount(&gateway)
        .await;

    let client = start_pair(&gateway).await;
    let args = serde_json::Map::from_iter([("account_id".into(), Value::String("DU123".into()))]);
    let mut params = CallToolRequestParams::new("positions");
    params.arguments = Some(args);
    let result = client.call_tool(params).await.expect("positions");

    let body: Value = serde_json::from_str(first_text_content(&result)).expect("json");
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 31);
    assert_eq!(arr[30]["conid"], json!(9999));
    client.cancel().await.ok();
}
