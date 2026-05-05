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
        .route("/debug/jar", get(debug_jar))
        .route("/debug/probe", get(debug_probe))
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
        .route("/events/orders", get(events_orders))
        .route("/events/pnl", get(events_pnl))
        .route("/events/marketdata", get(events_marketdata))
        .route("/events/gap", get(events_gap))
        .route("/events/_status", get(events_status))
        .route("/events/{topic}/history", get(events_history))
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

/// Headers we MUST NOT forward through a proxy hop. Subset of RFC 7230 §6.1
/// (`host`, `content-length`, `connection`, `keep-alive`, `proxy-*`, `te`,
/// `trailer`, `transfer-encoding`, `upgrade`) extended with:
///
/// * **`cookie`** — reqwest's shared jar is the single source of truth.
/// * **`authorization`** — CPGateway doesn't consume bearer/basic auth;
///   forwarding lets a caller probe whatever auth scheme the upstream
///   might (incorrectly) honour. Pure attack surface, drop it.
/// * **`x-forwarded-*` / `forwarded` / `x-real-ip`** — caller-controlled
///   client-IP claims. Forwarding lets a caller spoof their apparent
///   source IP to anything that logs/rate-limits/audits on those headers
///   downstream. Strip at the boundary; the proxy itself doesn't need
///   them.
const HOP_BY_HOP: &[&str] = &[
    "host",
    "content-length",
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailer",
    "transfer-encoding",
    "upgrade",
    "cookie",
    "authorization",
    "x-forwarded-for",
    "x-forwarded-host",
    "x-forwarded-proto",
    "x-real-ip",
    "forwarded",
];

fn is_hop_by_hop(name: &str) -> bool {
    HOP_BY_HOP.iter().any(|h| name.eq_ignore_ascii_case(h))
}

// `skip_all` because `req` and `state` aren't `Display`; we emit
// only the safe scalars (method + path) as span fields. Path is
// already query-stripped — see the `path_only` derivation below.
#[tracing::instrument(skip_all, fields(method, path))]
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
    // For logs we keep just the path — query strings frequently carry SSO
    // tokens / session ids that we don't want fanned out into log shippers.
    let path_only = req.uri().path().to_string();
    // Record into the parent span so the rest of the handler's
    // tracing events inherit them automatically.
    tracing::Span::current().record("method", tracing::field::display(&method));
    tracing::Span::current().record("path", tracing::field::display(&path_only));

    // Compose the target URL: Gateway root + the incoming URI. We use
    // the client's own derived root (scheme + host + trailing '/')
    // instead of hand-trimming the base URL so a non-standard prefix
    // doesn't silently break passthrough.
    let gateway_root = state.client().gateway_root_url().as_str();
    let target = format!("{}{}", gateway_root.trim_end_matches('/'), path_and_query);
    let target_url: reqwest::Url = target
        .parse()
        .map_err(|e| bezant::Error::BadRequest(format!("target url: {e}")))?;

    let headers = req.headers().clone();

    // Replay any cookies the browser sent into the shared jar so typed
    // API calls (`/health`, `/accounts`, …) see the same session that
    // the interactive login established.
    //
    // The jar (`bezant::NameKeyedJar`) keys cookies purely by name, so
    // inserting `JSESSIONID=NEW` always replaces `JSESSIONID=OLD` —
    // duplicates can't accumulate even if the Gateway sets the same
    // cookie at different paths in different responses. CPGateway
    // rejects requests that arrive with two values for the same cookie
    // name, so this single-source-of-truth model is required.
    //
    // **Trust model:** bezant-server is single-tenant. The shared jar
    // is intentionally visible to *all* server-side typed callers.
    // Don't deploy this proxy multi-tenant.
    let jar = state.client().cookie_jar();
    let mut pairs: Vec<&str> = Vec::new();
    for cookie_header in headers.get_all(axum::http::header::COOKIE) {
        if let Ok(raw) = cookie_header.to_str() {
            for pair in raw.split(';') {
                let trimmed = pair.trim();
                if trimmed.is_empty() {
                    continue;
                }
                // Drop edge-proxy auth cookies (Cloudflare Access /
                // Cloudflare-style infrastructure cookies). They're set
                // by the proxy layer for *its* session — not anything
                // CPGateway / api.ibkr.com expects, and Akamai 401s the
                // upstream call when an unrecognised `CF_Authorization=…`
                // cookie shows up alongside the IBKR session cookies.
                let name = trimmed.split('=').next().unwrap_or("").trim();
                if is_edge_auth_cookie(name) {
                    continue;
                }
                pairs.push(trimmed);
            }
        }
    }
    let injected = pairs.len();
    if injected > 0 {
        jar.set_pairs(&pairs);
    }
    // The Railway-deploy diagnostic phase is over (the residential-Pi
    // pattern is settled). Cookie replay is a per-request event and
    // belongs at debug level — no need to fan it out to log shippers.
    tracing::debug!(
        path = %path_only,
        cookies = injected,
        "passthrough cookie replay"
    );

    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
        .await
        .map_err(|e| bezant::Error::BadRequest(format!("read body: {e}")))?;

    let method_reqwest = reqwest::Method::from_bytes(method.as_str().as_bytes())
        .map_err(|e| bezant::Error::BadRequest(format!("method: {e}")))?;
    // Origin/Referer policy is path-conditional:
    //   * `/sso/*` (the interactive login flow) keeps the browser's
    //     `Origin` verbatim — IBKR's 2FA polling validates it as part
    //     of its session check, and rewriting it silently breaks the
    //     `/sso/Authenticator` poll.
    //   * `/v1/api/*` (CPAPI calls — the post-login surface) rewrites
    //     `Origin` to the Gateway's own host so its CPAPI CSRF guard
    //     accepts the call. Without this, post-login `/v1/api/*`
    //     returns 401 when the proxy is on a different public host
    //     than the Gateway thinks it's running on (Railway, fly.io,
    //     ngrok, …).
    let rewrite_origin = path_only.starts_with("/v1/api/") || path_only == "/v1/api";
    let gateway_origin = if rewrite_origin {
        let scheme = target_url.scheme();
        target_url.host_str().map(|h| match target_url.port() {
            Some(p) => format!("{scheme}://{h}:{p}"),
            None => format!("{scheme}://{h}"),
        })
    } else {
        None
    };
    let mut builder = state.client().http().request(method_reqwest, &target);
    for (name, value) in headers.iter() {
        // Drop hop-by-hop headers per RFC 7230 §6.1 plus `host`/`cookie`
        // (reqwest rebuilds the former, the shared jar replaces the
        // latter).
        if is_hop_by_hop(name.as_str()) {
            continue;
        }
        let lower = name.as_str().to_ascii_lowercase();
        if let Some(ref origin) = gateway_origin {
            if lower == "origin" {
                if let Ok(v) = reqwest::header::HeaderValue::from_str(origin) {
                    builder = builder.header(reqwest::header::ORIGIN, v);
                }
                continue;
            }
            if lower == "referer" {
                // Replace the origin prefix of the Referer URL but keep
                // the path/query — the upstream uses the path to drive
                // post-login redirects, so we don't want to lose it.
                if let Ok(orig) = value.to_str() {
                    let rewritten = rewrite_referer_origin(orig, origin);
                    if let Ok(v) = reqwest::header::HeaderValue::from_str(&rewritten) {
                        builder = builder.header(reqwest::header::REFERER, v);
                    }
                }
                continue;
            }
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

/// Diagnostic-only endpoint: lists shared cookie jar entries by name
/// (and value length — never value itself). Useful when chasing
/// "browser is logged in but typed /health says not_authenticated"
/// issues without leaking the live IBKR session cookie to anyone who
/// can hit the bind address.
///
/// Gated on `BEZANT_DEBUG_TOKEN`: returns 404 when no token is
/// configured, 401 when the caller's token doesn't match. Callers
/// authenticate via `?token=…` or the `X-Bezant-Debug-Token` header.
async fn debug_jar(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> Response<Body> {
    if let Err(resp) = debug_auth(&state, &headers, &q) {
        return resp;
    }
    let jar = state.client().cookie_jar();
    let entries: Vec<serde_json::Value> = jar
        .snapshot()
        .into_iter()
        .map(|(name, value)| {
            serde_json::json!({
                "name": name,
                "value_length": value.len(),
            })
        })
        .collect();
    let body = serde_json::json!({
        "gateway_root": state.client().gateway_root_url().as_str(),
        "size": entries.len(),
        "entries": entries,
    });
    Json(body).into_response()
}

/// Token check shared by every `/debug/*` handler.
///
/// Returns a 404 response when debug is disabled (no token
/// configured) so the existence of the endpoints isn't disclosed to
/// a probing attacker — they look identical to any other unmapped
/// route.
///
/// Returns a 401 response when a token IS configured but the caller
/// didn't present a matching one (different status because the
/// endpoint clearly exists; the caller is just unauthorised).
///
/// Token comparison is constant-time to avoid leaking length or
/// prefix-match info via response timing.
#[allow(clippy::result_large_err)]
// `Response<Body>` is the return type axum hands back; boxing it would
// just add an alloc on the (rare) auth-failure path. Suppress the lint.
fn debug_auth(
    state: &AppState,
    headers: &HeaderMap,
    query: &HashMap<String, String>,
) -> Result<(), Response<Body>> {
    let Some(expected) = state.debug_token() else {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap_or_default());
    };
    let presented = headers
        .get("x-bezant-debug-token")
        .and_then(|v| v.to_str().ok())
        .or_else(|| query.get("token").map(String::as_str))
        .unwrap_or("");
    if constant_time_eq(presented.as_bytes(), expected.as_bytes()) {
        return Ok(());
    }
    Err(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            r#"{"code":"debug_unauthorized","message":"missing or invalid debug token"}"#,
        ))
        .unwrap_or_default())
}

/// Constant-time byte comparison so token mismatch can't be timed.
/// Naive `==` short-circuits on first differing byte, leaking length
/// + prefix-match info via response timing.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Diagnostic-only endpoint: walks the post-login CPAPI sequence
/// (`auth/status` → `ssodh/init` → `tickle` → `portfolio/accounts`)
/// against the Gateway and reports each step's status, latency, body
/// preview, and Set-Cookie names — *plus* a top-level `verdict` that
/// pins down which step diverges from the happy path.
///
/// Built to discriminate between proxy-layer regressions and upstream
/// failures (e.g. CPGateway's internal SSODH bridge to `api.ibkr.com`
/// being rejected from a datacenter egress IP). The probe never aborts
/// on a step failure — it runs all four so the full picture is in
/// the response body.
///
/// Response shape:
/// ```json
/// {
///   "gateway_root": "https://localhost:5000/",
///   "elapsed_ms": 412,
///   "jar_size_before": 7,
///   "jar_size_after": 7,
///   "verdict": "ok",
///   "steps": [{ "name": "auth_status", "status": 200, ... }, ...]
/// }
/// ```
///
/// Returns 200 with the diagnostic body when authenticated for debug
/// (diagnostic-step failures surface in the body, not the HTTP status).
/// Returns 404-style error when debug isn't enabled, 401 when the
/// caller's debug token is missing/wrong.
async fn debug_probe(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> Response<Body> {
    if let Err(resp) = debug_auth(&state, &headers, &q) {
        return resp;
    }
    let client = state.client();
    let started = std::time::Instant::now();
    let jar_before = client.cookie_jar().snapshot().len();

    let auth_status = probe_step(
        client,
        "auth_status",
        reqwest::Method::POST,
        &["iserver", "auth", "status"],
        None,
    )
    .await;
    // `ssodh/init` is the bridge-establish call. Issuing it against a
    // session that's *already* bridged tears the session down — every
    // subsequent call then 401s. We discovered this the hard way: probe
    // would show `auth_status:200, ssodh_init:401, accounts:401` and
    // make a working session look broken. So skip the bridge step when
    // auth_status already reports `authenticated:true`; the diagnostic
    // is meant to *observe*, not perturb.
    let already_bridged = is_authenticated(&auth_status);
    let ssodh_init = if already_bridged {
        skipped_step(
            "ssodh_init",
            "POST",
            "/v1/api/iserver/auth/ssodh/init",
            "session already bridged (auth_status authenticated)",
        )
    } else {
        probe_step(
            client,
            "ssodh_init",
            reqwest::Method::POST,
            &["iserver", "auth", "ssodh", "init"],
            Some(serde_json::json!({ "publish": true, "compete": true })),
        )
        .await
    };
    let tickle = probe_step(client, "tickle", reqwest::Method::POST, &["tickle"], None).await;
    let accounts = probe_step(
        client,
        "accounts",
        reqwest::Method::GET,
        &["portfolio", "accounts"],
        None,
    )
    .await;

    let verdict = compute_verdict(&auth_status, &ssodh_init, &tickle, &accounts);
    let jar_after = client.cookie_jar().snapshot().len();

    let body = serde_json::json!({
        "gateway_root": client.gateway_root_url().as_str(),
        "elapsed_ms": started.elapsed().as_millis() as u64,
        "jar_size_before": jar_before,
        "jar_size_after": jar_after,
        "verdict": verdict,
        "steps": [auth_status, ssodh_init, tickle, accounts],
    });
    Json(body).into_response()
}

/// One step in the diagnostic probe.
///
/// Builds the request the same way the real proxy / typed client does:
/// pins `Content-Length` (Akamai 411 workaround), rewrites `Origin` and
/// `Referer` to the Gateway's own origin (CPAPI CSRF guard), and
/// captures the response without touching the shared cookie jar's
/// happy-path semantics — every Set-Cookie still flows back into the
/// jar via reqwest's cookie provider.
async fn probe_step(
    client: &bezant::Client,
    name: &'static str,
    method: reqwest::Method,
    path_segments: &[&str],
    body: Option<serde_json::Value>,
) -> serde_json::Value {
    let mut url = client.base_url().clone();
    if let Ok(mut segs) = url.path_segments_mut() {
        for seg in path_segments {
            segs.push(seg);
        }
    }
    let path_for_log = url.path().to_owned();

    let gateway_origin = client
        .gateway_root_url()
        .as_str()
        .trim_end_matches('/')
        .to_owned();

    let mut builder = client
        .http()
        .request(method.clone(), url.clone())
        .header(reqwest::header::ORIGIN, &gateway_origin)
        .header(reqwest::header::REFERER, format!("{gateway_origin}/"));

    let body_bytes: Vec<u8> = match (&method, body) {
        (m, _) if m == reqwest::Method::GET || m == reqwest::Method::HEAD => Vec::new(),
        (_, Some(json)) => serde_json::to_vec(&json).unwrap_or_default(),
        (_, None) => Vec::new(),
    };
    if method != reqwest::Method::GET && method != reqwest::Method::HEAD {
        builder = builder
            .header(
                reqwest::header::CONTENT_LENGTH,
                body_bytes.len().to_string(),
            )
            .body(body_bytes.clone());
        if !body_bytes.is_empty() {
            builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
        }
    }

    let started = std::time::Instant::now();
    // Per-step deadline: a hung Gateway shouldn't take the whole
    // probe with it. 5s is generous for the small JSON payloads we
    // hit; tickle/auth_status/accounts all return in <500 ms in
    // healthy operation.
    let result = match tokio::time::timeout(std::time::Duration::from_secs(5), builder.send()).await
    {
        Ok(send_result) => send_result.map_err(|e| e.to_string()),
        Err(_) => Err("step timed out after 5s".to_string()),
    };
    let latency_ms = started.elapsed().as_millis() as u64;

    match result {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let set_cookie_names: Vec<String> = resp
                .headers()
                .get_all(reqwest::header::SET_COOKIE)
                .iter()
                .filter_map(|v| v.to_str().ok())
                .map(|raw| {
                    raw.split(';')
                        .next()
                        .and_then(|s| s.split('=').next())
                        .map(|s| s.trim().to_owned())
                        .unwrap_or_default()
                })
                .filter(|s| !s.is_empty())
                .collect();
            // Cap the upstream read at 1 MiB so a misbehaving Gateway
            // can't OOM the probe with an unbounded body. Probe targets
            // are JSON status payloads — any response over 1 MiB is
            // already broken, the cap just bounds the damage.
            let bytes = read_capped(resp, 1024 * 1024).await.unwrap_or_default();
            // Parse the FULL body (not the truncated preview) for
            // discriminator fields the verdict logic needs. Doing it
            // here means `compute_verdict` can't be misled by a
            // response whose `authenticated` field happens to land
            // past the 512-byte preview window.
            let parsed_authenticated = serde_json::from_slice::<serde_json::Value>(&bytes)
                .ok()
                .and_then(|v| v["authenticated"].as_bool());
            // Redact known token-bearing keys from the preview before
            // exposing it to the operator — `session`, `ssoConclusion`,
            // and any field with `token` in the name commonly carry
            // resumable session material that shouldn't ride out via
            // a debug endpoint or log shipper.
            let preview_len = bytes.len().min(512);
            let raw_preview = String::from_utf8_lossy(&bytes[..preview_len]).into_owned();
            let body_preview = redact_tokens(&raw_preview);
            serde_json::json!({
                "name": name,
                "method": method.as_str(),
                "path": path_for_log,
                "status": status,
                "latency_ms": latency_ms,
                "body_bytes": bytes.len(),
                "body_preview": body_preview,
                "set_cookie_names": set_cookie_names,
                "error": serde_json::Value::Null,
                "_authenticated": parsed_authenticated,
            })
        }
        Err(e) => serde_json::json!({
            "name": name,
            "method": method.as_str(),
            "path": path_for_log,
            "status": serde_json::Value::Null,
            "latency_ms": latency_ms,
            "body_bytes": 0,
            "body_preview": "",
            "set_cookie_names": [],
            "error": e,
        }),
    }
}

/// Best-effort redaction of token-bearing JSON keys in a body preview.
///
/// Walks the JSON once, replaces values for keys named `session`,
/// `ssoConclusion`, or anything containing `token` (case-insensitive)
/// with `"<redacted>"`. Falls back to returning the input verbatim if
/// the preview isn't valid JSON — this is a best-effort guard, not a
/// security boundary.
///
/// Used by `/debug/probe` because preview output ships across HTTP
/// (and into the operator's terminal / log shipper) — leaking a live
/// session token there would let anyone with read access to the
/// debug response resume the IBKR session.
fn redact_tokens(preview: &str) -> String {
    let Ok(mut value) = serde_json::from_str::<serde_json::Value>(preview) else {
        // Not JSON — return verbatim. Most non-JSON CPAPI responses
        // are status pages or HTML error bodies that don't carry
        // tokens.
        return preview.to_owned();
    };
    redact_in_place(&mut value);
    value.to_string()
}

fn redact_in_place(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                let lower = k.to_ascii_lowercase();
                if lower == "session"
                    || lower == "ssoconclusion"
                    || lower.contains("token")
                    || lower.contains("secret")
                {
                    *v = serde_json::Value::String("<redacted>".to_owned());
                } else {
                    redact_in_place(v);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                redact_in_place(v);
            }
        }
        _ => {}
    }
}

/// Pin the failure to the first diverging step.
///
/// `auth_status` carries an extra parse: a 200 with `authenticated:false`
/// is a "needs_login" situation, not a transport success. Subsequent
/// steps are pure HTTP-status checks — any non-2xx pins the verdict to
/// that step's name. A `skipped` step is treated as success (the probe
/// chose not to run it because it would have been destructive).
fn compute_verdict(
    auth_status: &serde_json::Value,
    ssodh_init: &serde_json::Value,
    tickle: &serde_json::Value,
    accounts: &serde_json::Value,
) -> &'static str {
    if !is_2xx(auth_status) {
        return "auth_status_failed";
    }
    // ssodh_init is the canonical Railway-vs-Pi discriminator: when
    // auth_status reports unauthenticated AND a manual bridge attempt
    // also fails, that pins the failure to the SSODH leg upstream of
    // bezant-server. Check this BEFORE the needs_login short-circuit
    // so a real ssodh failure surfaces with its own verdict instead of
    // collapsing into the generic needs_login bucket.
    let ssodh_ran_and_failed = !is_skipped(ssodh_init) && !is_2xx(ssodh_init);
    if ssodh_ran_and_failed {
        return "ssodh_failed";
    }
    if !is_authenticated(auth_status) {
        return "needs_login";
    }
    if !is_2xx_or_skipped(tickle) {
        return "tickle_failed";
    }
    if !is_2xx_or_skipped(accounts) {
        return "accounts_failed";
    }
    "ok"
}

fn is_skipped(step: &serde_json::Value) -> bool {
    step["skipped"].as_bool().unwrap_or(false)
}

fn is_2xx(step: &serde_json::Value) -> bool {
    matches!(step["status"].as_u64(), Some(200..=299))
}

fn is_2xx_or_skipped(step: &serde_json::Value) -> bool {
    is_2xx(step) || step["skipped"].as_bool().unwrap_or(false)
}

/// Decide whether the session is already bridged. Reads the
/// `_authenticated` discriminator that `probe_step` extracted from the
/// FULL response body (not the 512-byte preview), so an `authenticated`
/// field that lands past the preview window doesn't silently flip the
/// verdict to "needs_login" and trigger the destructive ssodh path.
fn is_authenticated(step: &serde_json::Value) -> bool {
    if !is_2xx(step) {
        return false;
    }
    step["_authenticated"].as_bool().unwrap_or(false)
}

/// Build a placeholder step entry for a step the probe deliberately
/// did not execute. Surfaces in the JSON body so a reader can tell
/// "didn't fail, didn't run" apart from "ran and succeeded".
fn skipped_step(
    name: &'static str,
    method: &'static str,
    path: &'static str,
    reason: &'static str,
) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "method": method,
        "path": path,
        "status": serde_json::Value::Null,
        "latency_ms": 0,
        "body_bytes": 0,
        "body_preview": "",
        "set_cookie_names": [],
        "error": serde_json::Value::Null,
        "skipped": true,
        "skipped_reason": reason,
    })
}

#[tracing::instrument(skip_all)]
async fn health(State(state): State<AppState>) -> Result<Json<HealthBody>, AppError> {
    let status = state.client().auth_status().await?;
    Ok(Json(HealthBody {
        authenticated: status.authenticated,
        connected: status.connected,
        competing: status.competing,
        message: status.message,
    }))
}

#[tracing::instrument(skip_all)]
async fn accounts(State(state): State<AppState>) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", "accounts"], &[]).await
}

#[tracing::instrument(skip(state), fields(account_id = %account_id))]
async fn account_summary(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", account_id.as_str(), "summary"], &[]).await
}

#[derive(Deserialize, Debug)]
struct PositionsQuery {
    #[serde(default)]
    page: u32,
}

#[tracing::instrument(skip(state), fields(account_id = %account_id, page = q.page))]
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

#[tracing::instrument(skip(state), fields(account_id = %account_id))]
async fn account_ledger(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<Response<Body>, AppError> {
    passthrough_get(&state, &["portfolio", account_id.as_str(), "ledger"], &[]).await
}

/// List live + recently-filled orders for one account.
#[tracing::instrument(skip(state), fields(account_id = %account_id))]
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
#[tracing::instrument(skip(state, body), fields(account_id = %account_id))]
async fn submit_order(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    axum::extract::Json(body): axum::extract::Json<serde_json::Value>,
) -> Result<Response<Body>, AppError> {
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::UrlNotABase {
                url: state.client().base_url().to_string(),
            })?;
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
#[tracing::instrument(skip(state), fields(account_id = %account_id, order_id = %order_id))]
async fn cancel_order(
    State(state): State<AppState>,
    Path((account_id, order_id)): Path<(String, String)>,
) -> Result<Response<Body>, AppError> {
    let mut url = state.client().base_url().clone();
    {
        let mut segs = url
            .path_segments_mut()
            .map_err(|()| bezant::Error::UrlNotABase {
                url: state.client().base_url().to_string(),
            })?;
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
            .map_err(|()| bezant::Error::UrlNotABase {
                url: state.client().base_url().to_string(),
            })?;
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
        .ok_or(bezant::Error::MissingQuery { name: "conids" })?;
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
            .map_err(|()| bezant::Error::UrlNotABase {
                url: state.client().base_url().to_string(),
            })?;
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
/// What we do, header by header:
/// * **Hop-by-hop** (`content-length`, `transfer-encoding`, `connection`,
///   `keep-alive`, `te`, `trailer`, `upgrade`, `proxy-authenticate`,
///   `proxy-authorization`) are dropped — RFC 7230 §6.1.
/// * **`Set-Cookie`** has any `Domain=` attribute stripped. The Gateway's
///   upstream (IBKR) sets `Domain=.ibkr.com`, which the browser silently
///   discards when the response arrives from a different host. Falling
///   back to a host-only cookie keeps the SSO flow working behind any
///   hostname; the `Secure`, `HttpOnly`, `SameSite`, and `Path` flags
///   are preserved.
/// * **`Content-Type`** is rewritten / defaulted in two cases (skipped
///   when the body is empty *or* the status is 1xx/204/304):
///     - upstream sent `application/octet-stream` for what's actually a
///       text/HTML response (CPGateway does this on `/sso/Dispatcher`),
///     - upstream sent no Content-Type at all and our `Vec<u8>` body
///       would default to `application/octet-stream` (which makes
///       browsers offer to download instead of rendering).
/// * **Body decode failures** are tolerated *only* on 1xx/204/304/3xx,
///   where any body is required-or-conventionally empty. On 2xx/4xx/5xx
///   the decode failure surfaces as a normal upstream-transport error so
///   real data loss can't slip through silently.
///
/// `Origin` and `Referer` on the *request* side are deliberately
/// forwarded verbatim — see `passthrough_any`.
/// Maximum upstream response body the proxy will buffer. CPAPI
/// payloads are JSON status / position / order objects — anything
/// over a few hundred KB is already malformed. Cap at 25 MiB so a
/// hostile or buggy upstream sending unbounded chunks can't OOM the
/// proxy. (Inbound side has the matching 10 MiB limit via
/// `RequestBodyLimitLayer`.)
const MAX_UPSTREAM_BODY_BYTES: usize = 25 * 1024 * 1024;

#[tracing::instrument(skip_all, fields(upstream_status = %resp.status()))]
async fn forward(resp: reqwest::Response) -> Result<Response<Body>, AppError> {
    let status = resp.status();
    let headers_src = resp.headers().clone();
    let body_must_be_empty =
        matches!(status.as_u16(), 100..=199 | 204 | 304) || status.is_redirection();

    let bytes: Vec<u8> = match read_capped(resp, MAX_UPSTREAM_BODY_BYTES).await {
        Ok(b) => b,
        Err(e) if body_must_be_empty => {
            // Reqwest sometimes errors finalising chunked-encoded
            // empty bodies on 3xx/204/304; the headers we care about
            // (Location, Set-Cookie) are already in `headers_src`, so
            // recover instead of 502-ing the redirect.
            tracing::debug!(
                %status,
                error = %e,
                "forward: empty-body fallback on no-body status"
            );
            Vec::new()
        }
        Err(e) => {
            return Err(bezant::Error::UpstreamStatus {
                endpoint: "passthrough",
                status: status.as_u16(),
                body_preview: Some(e),
            }
            .into())
        }
    };

    let body_is_empty = bytes.is_empty();
    let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let mut headers = HeaderMap::new();
    let mut had_content_type = false;
    for (name, value) in headers_src.iter() {
        let n = name.as_str().to_ascii_lowercase();
        if is_hop_by_hop_response(&n) {
            continue;
        }
        // Set-Cookie may legitimately appear multiple times.
        if n == "set-cookie" {
            let value_bytes: Vec<u8> = match value.to_str() {
                Ok(raw) => strip_cookie_domain(raw).into_bytes(),
                Err(_) => value.as_bytes().to_vec(),
            };
            if let (Ok(name), Ok(value)) = (
                header::HeaderName::from_bytes(name.as_str().as_bytes()),
                HeaderValue::from_bytes(&value_bytes),
            ) {
                headers.append(name, value);
            }
            continue;
        }
        // Content-Type: rewrite octet-stream → text/html only when there
        // *is* a body and the status normally has one. Use `insert` so
        // we never accidentally emit two Content-Type headers.
        if n == "content-type" {
            let raw = value.to_str().unwrap_or("");
            let rewrite = !body_is_empty
                && !body_must_be_empty
                && raw.eq_ignore_ascii_case("application/octet-stream");
            let bytes_to_emit: &[u8] = if rewrite {
                b"text/html; charset=UTF-8"
            } else {
                value.as_bytes()
            };
            if let (Ok(name), Ok(value)) = (
                header::HeaderName::from_bytes(name.as_str().as_bytes()),
                HeaderValue::from_bytes(bytes_to_emit),
            ) {
                headers.insert(name, value);
                had_content_type = true;
            }
            continue;
        }
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::from_bytes(name.as_str().as_bytes()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            headers.append(name, value);
        }
    }

    // Default Content-Type to text/html only when we actually have a
    // body to render. On 1xx/204/304/3xx we leave it absent to comply
    // with RFC 9110 §8.3.
    if !had_content_type && !body_is_empty && !body_must_be_empty {
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=UTF-8"),
        );
    }

    // Build the response body directly instead of via
    // `(status, headers, bytes).into_response()`, because the latter
    // unconditionally inserts `Content-Type: application/octet-stream`
    // when none is set — which would defeat the explicit "no
    // Content-Type on 204" branch above.
    let mut response = Response::builder()
        .status(status)
        .body(Body::from(bytes))
        .map_err(|e| bezant::Error::ResponseBuild(e.to_string()))?;
    *response.headers_mut() = headers;
    Ok(response)
}

/// Response-side hop-by-hop denylist.
fn is_hop_by_hop_response(name: &str) -> bool {
    matches!(
        name,
        "content-length"
            | "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}

/// Replace the scheme + host[:port] prefix of `original` with
/// `new_origin`, preserving path + query. Used to rewrite the Referer
/// header on `/v1/api/*` requests when the proxy lives on a different
/// host than the Gateway.
fn rewrite_referer_origin(original: &str, new_origin: &str) -> String {
    match url::Url::parse(original) {
        Ok(u) => {
            let mut path_and_query = u.path().to_owned();
            if let Some(q) = u.query() {
                path_and_query.push('?');
                path_and_query.push_str(q);
            }
            format!("{}{}", new_origin.trim_end_matches('/'), path_and_query)
        }
        Err(_) => new_origin.to_owned(),
    }
}

/// Recognise cookies set by an edge-proxy / Zero-Trust front so we
/// don't replay them into bezant-server's shared jar — they're for
/// the proxy's own session and confuse upstream CPAPI/Akamai.
///
/// Built-in matches cover the common managed Zero-Trust providers:
/// * **Cloudflare Access** — `CF_Authorization` (JWT), `CF_AppSession`
/// * **AWS ALB OIDC** — `AWSELBAuthSessionCookie-*` (split across
///   multiple cookies on long sessions)
/// * **OAuth2 Proxy** — `_oauth2_proxy*`
/// * **Vercel Authentication** — `_vercel_jwt`, `_vercel_sso_nonce`
/// * **Pomerium** — `_pomerium*`
///
/// For deployments behind a custom edge, set `BEZANT_EDGE_COOKIE_PREFIXES`
/// to a comma-separated list of additional prefixes — e.g.
/// `BEZANT_EDGE_COOKIE_PREFIXES=MyEdge_,_internal_` — and any cookie
/// whose name starts with one of those is dropped at the boundary.
fn is_edge_auth_cookie(name: &str) -> bool {
    const BUILTIN_PREFIXES: &[&str] = &[
        "CF_Authorization",
        "CF_AppSession",
        "AWSELBAuthSessionCookie",
        "_oauth2_proxy",
        "_vercel_jwt",
        "_vercel_sso_nonce",
        "_pomerium",
    ];
    if BUILTIN_PREFIXES
        .iter()
        .any(|p| name.eq_ignore_ascii_case(p) || name.starts_with(p))
    {
        return true;
    }
    // Lazy env lookup: keeps the helper cheap for the no-config case.
    if let Ok(extra) = std::env::var("BEZANT_EDGE_COOKIE_PREFIXES") {
        for prefix in extra.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            if name.starts_with(prefix) {
                return true;
            }
        }
    }
    false
}

/// Stream an upstream response body into a `Vec<u8>` while enforcing a
/// hard byte cap. Stops as soon as the cap is exceeded so a hostile
/// upstream sending unbounded chunks can't OOM the proxy. Returns the
/// fully buffered bytes on success, or an error if the cap is hit or
/// the upstream connection breaks. Used by both `forward()` (for
/// passthrough responses, large cap) and `probe_step` (smaller cap,
/// for diagnostic JSON).
async fn read_capped(resp: reqwest::Response, max: usize) -> std::result::Result<Vec<u8>, String> {
    use futures_util::StreamExt;
    let mut bytes = Vec::new();
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("read chunk: {e}"))?;
        if bytes.len() + chunk.len() > max {
            return Err(format!(
                "upstream body exceeded {max} byte cap (>{}B)",
                bytes.len() + chunk.len()
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

/// Strip any `Domain=...` attribute from a Set-Cookie value. The browser
/// will fall back to a host-only cookie scoped to whatever host it saw
/// the response on, which is what we want when proxying a cookie that
/// the Gateway pinned to `Domain=.ibkr.com`.
fn strip_cookie_domain(value: &str) -> String {
    value
        .split(';')
        .filter(|part| !part.trim().to_ascii_lowercase().starts_with("domain="))
        .collect::<Vec<_>>()
        .join(";")
}

// ===========================================================================
// /events/* handlers — cursor-based reads against the in-memory ring buffers
// maintained by the connector task. See `events::connector::EventsHandle`.
// ===========================================================================

/// Query string for `/events/{topic}` reads.
#[derive(Debug, Deserialize)]
struct EventsQuery {
    /// Resume from this cursor (exclusive). `0` (or omitted) means "from
    /// the head of the buffer".
    #[serde(default)]
    since: u64,
    /// Maximum number of events to return. Defaults to `100`, capped at
    /// `1000` server-side to bound response size.
    #[serde(default = "EventsQuery::default_limit")]
    limit: usize,
}

impl EventsQuery {
    fn default_limit() -> usize {
        100
    }
}

/// 412 body when the caller's cursor falls past the ring buffer's head.
#[derive(Debug, Serialize)]
struct CursorExpiredBody {
    code: &'static str,
    head_cursor: u64,
    reset_epoch: u64,
    message: &'static str,
}

async fn events_orders(
    State(state): State<AppState>,
    Query(q): Query<EventsQuery>,
) -> Response<Body> {
    read_events_topic(&state, "orders", q).await
}

async fn events_pnl(
    State(state): State<AppState>,
    Query(q): Query<EventsQuery>,
) -> Response<Body> {
    read_events_topic(&state, "pnl", q).await
}

async fn events_gap(
    State(state): State<AppState>,
    Query(q): Query<EventsQuery>,
) -> Response<Body> {
    read_events_topic(&state, "gap", q).await
}

#[derive(Debug, Deserialize)]
struct MarketDataEventsQuery {
    conid: i64,
    #[serde(default)]
    since: u64,
    #[serde(default = "EventsQuery::default_limit")]
    limit: usize,
}

async fn events_marketdata(
    State(state): State<AppState>,
    Query(q): Query<MarketDataEventsQuery>,
) -> Response<Body> {
    let Some(handle) = state.events() else {
        return events_disabled_response();
    };
    // Lazily ensure the upstream WS is subscribed for this conid.
    if let Err(e) = handle.ensure_market_data(q.conid).await {
        return (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "code": "events_subscribe_failed",
                "message": e,
            })),
        )
            .into_response();
    }

    let topic = format!("marketdata:{}", q.conid);
    let limit = q.limit.min(1_000);
    read_events_topic_resolved(handle, &topic, q.since, limit).await
}

async fn events_status(State(state): State<AppState>) -> Response<Body> {
    let Some(handle) = state.events() else {
        return events_disabled_response();
    };
    Json(handle.status().await).into_response()
}

#[derive(Debug, Deserialize)]
struct HistoryQuery {
    /// RFC 3339 timestamp lower bound (exclusive). Required.
    since_ts: String,
    /// Cap on returned rows. Default 500, server cap 5000.
    #[serde(default = "HistoryQuery::default_limit")]
    limit: usize,
}

impl HistoryQuery {
    fn default_limit() -> usize {
        500
    }
}

async fn events_history(
    State(state): State<AppState>,
    Path(topic): Path<String>,
    Query(q): Query<HistoryQuery>,
) -> Response<Body> {
    let Some(handle) = state.events() else {
        return events_disabled_response();
    };
    let Some(log) = handle.event_log() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "code": "events_history_disabled",
                "message": "events sqlite persistence is not enabled \
                            (set BEZANT_EVENTS_DB_PATH to turn it on)"
            })),
        )
            .into_response();
    };
    let limit = q.limit.min(5_000).max(1);
    let log_clone = log.clone();
    let topic_clone = topic.clone();
    let since_ts = q.since_ts.clone();
    let result = tokio::task::spawn_blocking(move || {
        log_clone.query_since(&topic_clone, &since_ts, limit)
    })
    .await;
    match result {
        Ok(Ok(events)) => {
            let body = serde_json::json!({
                "topic": topic,
                "events": events,
                "count": events.len(),
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "code": "events_history_query_failed",
                "message": e.to_string(),
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "code": "events_history_join_failed",
                "message": e.to_string(),
            })),
        )
            .into_response(),
    }
}

async fn read_events_topic(state: &AppState, topic: &str, q: EventsQuery) -> Response<Body> {
    let Some(handle) = state.events() else {
        return events_disabled_response();
    };
    read_events_topic_resolved(handle, topic, q.since, q.limit.min(1_000)).await
}

async fn read_events_topic_resolved(
    handle: &crate::events::EventsHandle,
    topic: &str,
    since: u64,
    limit: usize,
) -> Response<Body> {
    let Some(result) = handle.read_topic(topic, since, limit.max(1)).await else {
        // Topic doesn't exist yet — no events have ever arrived for it.
        // Return a 200 with empty array + the caller's cursor so polls
        // remain idempotent until the first event lands.
        let status = handle.status().await;
        let body = serde_json::json!({
            "events": [],
            "next_cursor": since,
            "reset_epoch": status.reset_epoch,
        });
        return (StatusCode::OK, Json(body)).into_response();
    };

    use crate::events::ReadResult;
    match result {
        ReadResult::Ok { events, next_cursor } => {
            if events.is_empty() {
                // No new events — 204 to make "nothing happened" cheap to
                // detect on the client side without parsing a body.
                let mut response = Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Body::empty())
                    .unwrap();
                let cursor_str = next_cursor.to_string();
                if let Ok(v) = HeaderValue::from_str(&cursor_str) {
                    response.headers_mut().insert("x-bezant-cursor", v);
                }
                return response;
            }
            let reset_epoch = events
                .first()
                .map(|e| e.reset_epoch)
                .unwrap_or_else(|| 0);
            let body = serde_json::json!({
                "events": events,
                "next_cursor": next_cursor,
                "reset_epoch": reset_epoch,
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        ReadResult::CursorExpired {
            head_cursor,
            reset_epoch,
        } => {
            let body = CursorExpiredBody {
                code: "cursor_expired",
                head_cursor,
                reset_epoch,
                message:
                    "the requested cursor is older than the oldest buffered event; \
                     reset to head_cursor and emit a synthetic gap on the consumer side",
            };
            (StatusCode::PRECONDITION_FAILED, Json(body)).into_response()
        }
    }
}

fn events_disabled_response() -> Response<Body> {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "code": "events_disabled",
            "message": "events capture is not enabled on this bezant-server instance \
                        (set BEZANT_EVENTS_ENABLED=1 to turn it on)"
        })),
    )
        .into_response()
}

#[cfg(test)]
mod redact_tests {
    use super::redact_tokens;

    #[test]
    fn redacts_session_field() {
        let input = r#"{"session":"AAAA-real-token","other":1}"#;
        let out = redact_tokens(input);
        assert!(out.contains("<redacted>"), "got: {out}");
        assert!(!out.contains("AAAA-real-token"), "raw token leaked: {out}");
        // Non-token fields preserved.
        assert!(out.contains("\"other\":1"), "got: {out}");
    }

    #[test]
    fn redacts_token_substring_keys() {
        let input = r#"{"accessToken":"x","refresh_token":"y","tokenExpiry":99}"#;
        let out = redact_tokens(input);
        assert!(!out.contains(r#""x""#), "accessToken leaked: {out}");
        assert!(!out.contains(r#""y""#), "refresh_token leaked: {out}");
        // tokenExpiry has "token" in the name → also redacted (intentional;
        // expiry timestamps don't carry secrets but the conservative bias
        // is correct for a debug surface).
        assert!(!out.contains("99"), "tokenExpiry value leaked: {out}");
    }

    #[test]
    fn passes_non_json_through_verbatim() {
        let input = "Not a JSON body, just text.";
        let out = redact_tokens(input);
        assert_eq!(out, input);
    }

    #[test]
    fn redacts_inside_arrays_and_nested_objects() {
        let input = r#"{"sessions":[{"session":"a"},{"session":"b"}]}"#;
        let out = redact_tokens(input);
        assert!(!out.contains(r#""a""#), "got: {out}");
        assert!(!out.contains(r#""b""#), "got: {out}");
    }
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
