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
    use reqwest::cookie::CookieStore;
    let method = req.method().clone();
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/")
        .to_string();

    // Compose the target URL: Gateway root + the incoming URI. We use
    // the client's own derived root (scheme + host + trailing '/')
    // instead of hand-trimming the base URL so a non-standard prefix
    // doesn't silently break passthrough.
    let gateway_root = state.client().gateway_root_url().as_str();
    let target = format!(
        "{}{}",
        gateway_root.trim_end_matches('/'),
        path_and_query
    );
    let target_url: reqwest::Url = target
        .parse()
        .map_err(|e| bezant::Error::other(format!("target url: {e}")))?;

    let headers = req.headers().clone();

    // Replay any cookies the browser sent into the shared jar so typed
    // API calls (`/health`, `/accounts`, …) see the same session that
    // the interactive login established. Each injection overwrites the
    // existing entry by name, which avoids both (a) monotonic growth of
    // the jar as the browser rotates cookies and (b) silent scope
    // broadening when the browser's cookie was originally scoped to
    // something narrower than `Path=/`.
    let jar = state.client().cookie_jar();
    let mut injected = 0usize;
    for cookie_header in headers.get_all(axum::http::header::COOKIE) {
        if let Ok(raw) = cookie_header.to_str() {
            for pair in raw.split(';') {
                let pair = pair.trim();
                if pair.is_empty() {
                    continue;
                }
                // Reqwest's `Jar::set_cookies` keys entries by
                // (domain, path, name), so re-setting a cookie with
                // the same name + path (`/`) replaces the previous
                // value rather than accumulating. `Path=/` matches how
                // the browser treats its own jar for these paths.
                let set_cookie = format!("{pair}; Path=/");
                if let Ok(hv) = reqwest::header::HeaderValue::from_str(&set_cookie) {
                    jar.set_cookies(&mut std::iter::once(&hv), &target_url);
                    injected += 1;
                }
            }
        }
    }
    if injected > 0 {
        tracing::info!(
            path = %path_and_query,
            cookies = injected,
            "passthrough injected browser cookies into shared jar"
        );
    } else {
        tracing::debug!(path = %path_and_query, "passthrough: no browser cookies");
    }

    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
        .await
        .map_err(|e| bezant::Error::other(format!("read body: {e}")))?;

    let method_reqwest = reqwest::Method::from_bytes(method.as_str().as_bytes())
        .map_err(|e| bezant::Error::other(format!("method: {e}")))?;
    let mut builder = state.client().http().request(method_reqwest, &target);
    for (name, value) in headers.iter() {
        // Drop hop-by-hop + host + cookie headers — reqwest rebuilds
        // the host header and sources cookies from the shared jar.
        let n = name.as_str().to_ascii_lowercase();
        if matches!(
            n.as_str(),
            "host" | "content-length" | "connection" | "transfer-encoding" | "cookie"
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
        // Pin Content-Length even for empty bodies — Akamai (fronting
        // the CPAPI) returns 411 if the POST arrives with neither
        // Content-Length nor Transfer-Encoding, which is the wire
        // shape reqwest/hyper can produce for an empty Vec.
        let len = body_bytes.len();
        builder = builder
            .header(reqwest::header::CONTENT_LENGTH, len.to_string())
            .body(body_bytes.to_vec());
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
///
/// `Set-Cookie` headers have any `Domain=` attribute stripped before
/// being emitted to the browser: the Gateway's upstream (IBKR) sets
/// `Domain=.ibkr.com`, which a browser would immediately discard when
/// the response arrives from a completely different host. Falling back
/// to a host-only cookie keeps the flow working behind any hostname.
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
        let value_bytes: Vec<u8> = if n == "set-cookie" {
            match value.to_str() {
                Ok(raw) => strip_cookie_domain(raw).into_bytes(),
                Err(_) => value.as_bytes().to_vec(),
            }
        } else {
            value.as_bytes().to_vec()
        };
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::from_bytes(name.as_str().as_bytes()),
            HeaderValue::from_bytes(&value_bytes),
        ) {
            headers.append(name, value);
        }
    }

    Ok((status, headers, bytes).into_response())
}

/// Strip any `Domain=...` attribute from a Set-Cookie value. The browser
/// will fall back to a host-only cookie scoped to whatever host it saw
/// the response on, which is what we want when proxying a cookie that
/// the Gateway pinned to `Domain=.ibkr.com`.
fn strip_cookie_domain(value: &str) -> String {
    value
        .split(';')
        .filter(|part| {
            !part
                .trim()
                .to_ascii_lowercase()
                .starts_with("domain=")
        })
        .collect::<Vec<_>>()
        .join(";")
}

#[cfg(test)]
mod forward_tests {
    use super::strip_cookie_domain;

    #[test]
    fn drops_ibkr_domain_attribute() {
        let input = "SID=abc; Domain=.ibkr.com; Path=/; Secure";
        // Whitespace retention is deliberate — we only drop the Domain
        // segment and keep the original spacing elsewhere.
        assert_eq!(strip_cookie_domain(input), "SID=abc; Path=/; Secure");
    }

    #[test]
    fn leaves_cookie_without_domain_untouched() {
        let input = "SID=abc; Path=/; Secure";
        assert_eq!(strip_cookie_domain(input), input);
    }

    #[test]
    fn case_insensitive() {
        let input = "SID=abc; DOMAIN=ibkr.com; Path=/";
        assert_eq!(strip_cookie_domain(input), "SID=abc; Path=/");
    }
}
