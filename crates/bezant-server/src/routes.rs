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
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::AppError;
use crate::state::AppState;

/// Build the full axum router wired to `state`.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", get(accounts))
        .route("/accounts/{account_id}/summary", get(account_summary))
        .route("/accounts/{account_id}/positions", get(account_positions))
        .route("/accounts/{account_id}/ledger", get(account_ledger))
        .route("/contracts/search", get(contract_search))
        .route("/market/snapshot", get(market_snapshot))
        .with_state(state)
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
    let url = state
        .client()
        .base_url()
        .join("iserver/secdef/search")
        .map_err(|e| bezant::Error::other(format!("url build: {e}")))?;
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

/// Forward a `reqwest::Response` as an axum response with the same status,
/// content-type, and body bytes.
async fn forward(resp: reqwest::Response) -> Result<Response<Body>, AppError> {
    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json")
        .to_owned();
    let bytes = resp.bytes().await.map_err(bezant::Error::Http)?;

    // Re-map reqwest's `StatusCode` into axum's via numeric value.
    let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let mut headers = HeaderMap::new();
    if let Ok(v) = HeaderValue::from_str(&content_type) {
        headers.insert(header::CONTENT_TYPE, v);
    }

    let mut response = (status, headers, bytes).into_response();
    // Ensure we don't clobber the Body variant
    *response.status_mut() = status;
    Ok(response)
}
