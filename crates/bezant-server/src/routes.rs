//! HTTP route handlers.
//!
//! The server mostly acts as an untyped pass-through: it takes the CPAPI
//! path, hits the Gateway via the `bezant::Client`'s inner reqwest client,
//! and forwards the JSON body back to the caller. This lets downstream
//! apps in any language consume CPAPI over plain HTTP without touching
//! the typed Rust layer.
//!
//! A handful of handlers (like `/health`) use the typed facade because
//! they project the raw response into a narrower shape.

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::AppError;
use crate::state::AppState;

/// Build the full axum router wired to `state`. Exposed for integration
/// tests that want to drive the router without opening a TCP listener.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", get(accounts))
        .route("/accounts/{account_id}/summary", get(account_summary))
        .route("/accounts/{account_id}/positions", get(account_positions))
        .route("/accounts/{account_id}/ledger", get(account_ledger))
        .route(
            "/accounts/{account_id}/orders",
            get(account_orders).post(submit_order),
        )
        .route(
            "/accounts/{account_id}/orders/{order_id}",
            delete(cancel_order),
        )
        .route("/contracts/search", get(contract_search))
        .route("/market/snapshot", get(market_snapshot))
        // Anything we haven't explicitly wrapped falls through to the
        // Gateway verbatim. The big reason this matters: the CPGateway's
        // interactive login flow (`/sso/Login`, static JS/CSS/img assets,
        // `/v1/api/iserver/auth/ssodh/init`, …) has to run through
        // *this* HTTP client so its cookie jar captures the session. Any
        // future endpoints we add to bezant-server just take precedence
        // over this catch-all via specific routes.
        .fallback(passthrough_any)
        .with_state(state)
}

async fn passthrough_any(
    State(state): State<AppState>,
    req: axum::extract::Request,
) -> Result<Response<Body>, AppError> {
    use axum::http::Method;
    let method = req.method().clone();
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/")
        .to_string();

    // Compose the target URL: Gateway base + the incoming URI.
    let gateway_base = state.client().base_url();
    let base_str = gateway_base
        .as_str()
        .trim_end_matches('/')
        .trim_end_matches("/v1/api")
        .trim_end_matches('/');
    let target = format!("{base_str}{path_and_query}");

    let headers = req.headers().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
        .await
        .map_err(|e| bezant::Error::other(format!("read body: {e}")))?;

    let method_reqwest = reqwest::Method::from_bytes(method.as_str().as_bytes())
        .map_err(|e| bezant::Error::other(format!("method: {e}")))?;
    let mut builder = state.client().http().request(method_reqwest, &target);
    for (name, value) in headers.iter() {
        // Drop hop-by-hop + host headers — reqwest rebuilds them.
        let n = name.as_str().to_ascii_lowercase();
        if matches!(
            n.as_str(),
            "host" | "content-length" | "connection" | "transfer-encoding"
        ) {
            continue;
        }
        if let Ok(v) = reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
            if let Ok(name) = reqwest::header::HeaderName::from_bytes(name.as_str().as_bytes()) {
                builder = builder.header(name, v);
            }
        }
    }
    if method != Method::GET && method != Method::HEAD {
        builder = builder.body(body_bytes.to_vec());
    }

    let resp = builder.send().await.map_err(bezant::Error::Http)?;
    forward(resp).await
}

#[derive(Serialize)]
struct HealthBody {
    authenticated: bool,
    connected: bool,
    competing: bool,
    message: Option<String>,
}

async fn health(State(state): State<AppState>) -> Result<Json<HealthBody>, AppError> {
    let status = state.client().auth_status().await?;
    Ok(Json(HealthBody {
        authenticated: status.authenticated,
        connected: status.connected,
        competing: status.competing,
        message: status.message,
    }))
}

async fn accounts(State(state): State<AppState>) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", "accounts"], &[]).await
}

async fn account_summary(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", account_id.as_str(), "summary"], &[]).await
}

#[derive(Deserialize)]
struct PositionsQuery {
    #[serde(default)]
    page: u32,
}

async fn account_positions(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    Query(q): Query<PositionsQuery>,
) -> Result<Response<Body>, AppError> {
    let page = q.page.to_string();
    passthrough_get(
        &state,
        &["portfolio", account_id.as_str(), "positions", page.as_str()],
        &[],
    )
    .await
}

async fn account_ledger(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", account_id.as_str(), "ledger"], &[]).await
}

/// List live + recently-filled orders for one account.
async fn account_orders(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Response<Body>, AppError> {
    // CPAPI exposes this under /iserver/account/orders?accountId=…
    passthrough_get(
        &state,
        &["iserver", "account", "orders"],
        &[("accountId", account_id.as_str())],
    )
    .await
}

/// Submit one or more orders for an account.
///
/// The body is forwarded verbatim to `POST /iserver/account/{id}/orders`.
/// CPAPI accepts either `{ "orders": [...] }` or a single order object; we
/// stay out of the way and let IBKR's own validator surface errors.
async fn submit_order(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    axum::extract::Json(body): axum::extract::Json<serde_json::Value>,
) -> Result<Response<Body>, AppError> {
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::other("base url cannot be a base"))?;
        segs.push("iserver")
            .push("account")
            .push(account_id.as_str())
            .push("orders");
    }
    let resp = state
        .client()
        .http()
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(bezant::Error::Http)?;
    forward(resp).await
}

/// Cancel a live order.
async fn cancel_order(
    State(state): State<AppState>,
    Path((account_id, order_id)): Path<(String, String)>,
) -> Result<Response<Body>, AppError> {
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::other("base url cannot be a base"))?;
        segs.push("iserver")
            .push("account")
            .push(account_id.as_str())
            .push("order")
            .push(order_id.as_str());
    }
    let resp = state
        .client()
        .http()
        .delete(url)
        .send()
        .await
        .map_err(bezant::Error::Http)?;
    forward(resp).await
}

#[derive(Deserialize)]
struct ContractSearchQuery {
    symbol: String,
    #[serde(default)]
    name: bool,
    #[serde(rename = "secType", default = "default_sec_type")]
    sec_type: String,
}

fn default_sec_type() -> String {
    "STK".into()
}

async fn contract_search(
    State(state): State<AppState>,
    Query(q): Query<ContractSearchQuery>,
) -> Result<Response<Body>, AppError> {
    // Symbol lookup is a POST with a JSON body on the CPAPI side.
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::other("base url cannot be a base"))?;
        segs.push("iserver").push("secdef").push("search");
    }
    let body = serde_json::json!({
        "symbol": q.symbol,
        "name": q.name,
        "secType": q.sec_type,
    });
    let resp = state
        .client()
        .http()
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(bezant::Error::Http)?;
    forward(resp).await
}

async fn market_snapshot(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> Result<Response<Body>, AppError> {
    let conids = q
        .get("conids")
        .ok_or_else(|| bezant::Error::other("missing 'conids' query param"))?;
    let fields = q
        .get("fields")
        .cloned()
        .unwrap_or_else(|| "31,84,86,87".into());
    passthrough_get(
        &state,
        &["iserver", "marketdata", "snapshot"],
        &[("conids", conids), ("fields", &fields)],
    )
    .await
}

/// Shared pass-through helper: builds `<base>/a/b/c`, appends query params,
/// forwards the Gateway response verbatim (status + content-type + body).
async fn passthrough_get(
    state: &AppState,
    path_segments: &[&str],
    query: &[(&str, &str)],
) -> Result<Response<Body>, AppError> {
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::other("base url cannot be a base"))?;
        for seg in path_segments {
            segs.push(seg);
        }
    }
    if !query.is_empty() {
        let mut q = url.query_pairs_mut();
        for (k, v) in query {
            q.append_pair(k, v);
        }
    }
    let resp = state
        .client()
        .http()
        .get(url)
        .send()
        .await
        .map_err(bezant::Error::Http)?;
    forward(resp).await
}

/// Forward a `reqwest::Response` as an axum response.
///
/// Copies every response header through so that multi-step Gateway flows
/// (SSO redirects, session cookies, cache headers on static assets)
/// reach the browser unchanged. Hop-by-hop headers that a new transport
/// will rebuild — `content-length`, `transfer-encoding`, `connection` —
/// are dropped. The Gateway issues session cookies with
/// `Secure; SameSite=None`, so deployments need TLS in front of
/// bezant-server (a Railway-style HTTPS edge works out of the box).
async fn forward(resp: reqwest::Response) -> Result<Response<Body>, AppError> {
    let status = resp.status();
    let headers_src = resp.headers().clone();
    let bytes = resp.bytes().await.map_err(bezant::Error::Http)?;

    let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let mut headers = HeaderMap::new();
    for (name, value) in headers_src.iter() {
        let n = name.as_str().to_ascii_lowercase();
        if matches!(
            n.as_str(),
            "content-length" | "transfer-encoding" | "connection"
        ) {
            continue;
        }
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::from_bytes(name.as_str().as_bytes()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            headers.append(name, value);
        }
    }

    let mut response = (status, headers, bytes).into_response();
    *response.status_mut() = status;
    Ok(response)
}
