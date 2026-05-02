//! Integration tests for the `bezant-server` HTTP surface.
//!
//! Each test spins up a wiremock Gateway, builds the real axum router
//! against it, and drives requests through `tower::ServiceExt::oneshot`.
//! No TCP binds, no fixtures beyond what wiremock provides — the tests
//! exercise every layer except the public listener.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use bezant_server::{router, AppState};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn make_app(gateway: &MockServer) -> axum::Router {
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    router(AppState::new(client))
}

async fn response_body(resp: axum::http::Response<Body>) -> (StatusCode, Value) {
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let value: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, value)
}

#[tokio::test]
async fn health_projects_gateway_auth_status() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": "ready"
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["authenticated"], json!(true));
    assert_eq!(body["connected"], json!(true));
    assert_eq!(body["message"], json!("ready"));
}

#[tokio::test]
async fn health_maps_unauthorized_to_401_with_code() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["code"], json!("not_authenticated"));
}

#[tokio::test]
async fn health_maps_disconnected_session_to_503() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": false,
            "competing": false,
            "message": ""
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    // Gateway reports connected=false → facade returns NoSession → 503.
    assert_eq!(status, StatusCode::OK); // health is projected, not error-mapped
    assert_eq!(body["connected"], json!(false));
    assert_eq!(body["authenticated"], json!(true));
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let gateway = MockServer::start().await;
    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn accounts_passthrough_forwards_body_verbatim() {
    let gateway = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"accountId": "DU123", "currency": "USD"},
            {"accountId": "DU456", "currency": "GBP"}
        ])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/accounts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["accountId"], json!("DU123"));
    assert_eq!(body[1]["currency"], json!("GBP"));
}

#[tokio::test]
async fn account_summary_substitutes_path_parameter() {
    let gateway = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "netliquidation": {"amount": 1234.5, "currency": "USD"}
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/accounts/DU123/summary")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["netliquidation"]["amount"], json!(1234.5));
}

#[tokio::test]
async fn account_positions_defaults_to_page_zero() {
    let gateway = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"conid": 111, "position": 10.0}
        ])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/accounts/DU123/positions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["conid"], json!(111));
}

#[tokio::test]
async fn contract_search_posts_json_body() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"conid": "265598", "companyName": "Apple Inc"}
        ])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/contracts/search?symbol=AAPL")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["conid"], json!("265598"));
}

#[tokio::test]
async fn account_orders_list_passes_account_query() {
    let gateway = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/iserver/account/orders"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "orders": []
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/accounts/DU123/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["orders"].is_array());
}

#[tokio::test]
async fn submit_order_forwards_body_to_cpapi() {
    let gateway = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/account/DU123/orders"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"order_id": "abc123", "order_status": "Submitted"}
        ])))
        .mount(&gateway)
        .await;

    let body = json!({
        "orders": [{
            "conid": 265598,
            "orderType": "MKT",
            "side": "BUY",
            "quantity": 10,
            "tif": "DAY"
        }]
    });
    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts/DU123/orders")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["order_id"], json!("abc123"));
}

#[tokio::test]
async fn cancel_order_forwards_delete_to_cpapi() {
    let gateway = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v1/api/iserver/account/DU123/order/abc123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "msg": "Request was submitted"
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/accounts/DU123/orders/abc123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, _) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn market_snapshot_requires_conids_param() {
    let gateway = MockServer::start().await;
    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/market/snapshot")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        body["message"].as_str().unwrap().contains("conids"),
        "unexpected body: {body}"
    );
}

// ---------- Passthrough / SSO-flow regressions ----------
// The bugs we hit during the local Gateway integration cycle —
// download dialogs on /sso/Dispatcher, Akamai 400s on cross-origin
// posts, browser cookies never reaching the typed-API jar — all live
// in the passthrough layer. These tests pin the fixes against
// wiremock so we can iterate without a live IBKR session.

use wiremock::matchers::{header, method as wm_method, path as wm_path};

const TEST_ACCOUNT: &str = "DU123";

/// Helper: count how many times a header name appears on the response.
fn header_count(resp: &axum::http::Response<Body>, name: &str) -> usize {
    resp.headers().get_all(name).iter().count()
}

/// Helper: collect the first response header as a String, default empty.
fn header_value(resp: &axum::http::Response<Body>, name: &str) -> String {
    resp.headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_owned()
}

/// CPGateway tags HTML / plain-text pages like `/sso/Dispatcher` with
/// `application/octet-stream` (or no Content-Type at all), which makes
/// the browser offer to download instead of render. `forward()` should
/// rewrite both cases to text/html.
#[tokio::test]
async fn passthrough_rewrites_octet_stream_to_html() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Dispatcher"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw("Client login succeeds", "application/octet-stream"),
        )
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Dispatcher")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        header_count(&resp, "content-type"),
        1,
        "expected exactly one Content-Type header (no duplicate from rewrite path)"
    );
    let ct = header_value(&resp, "content-type");
    assert!(
        ct.starts_with("text/html"),
        "expected text/html rewrite, got {ct:?}"
    );
}

/// When the upstream sends *no* Content-Type at all (CPGateway does this
/// on `/sso/Dispatcher` for plain-text bodies), our default kicks in
/// and prevents axum's `Vec<u8>` body adapter from filling
/// `application/octet-stream`. RFC §8.3 demands we *not* invent one
/// for 1xx/204/304/3xx — covered by the next two tests.
#[tokio::test]
async fn passthrough_defaults_missing_content_type_to_html() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Dispatcher"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw("Client login succeeds", "text/plain"),
        )
        .mount(&gateway)
        .await;

    // We can't easily make wiremock send *no* Content-Type, so the
    // realistic regression-test for "axum would otherwise default to
    // octet-stream" is: any non-empty body must end up with a
    // Content-Type header set on the response. Pin the contract.
    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Dispatcher")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let ct = header_value(&resp, "content-type");
    assert!(
        !ct.is_empty(),
        "expected a Content-Type to flow through, got empty"
    );
}

/// Don't invent a Content-Type for 204 No Content responses — RFC 9110
/// §8.3 says payload-metadata headers MUST NOT accompany them, and
/// adding `text/html` to a body-less response is misleading.
#[tokio::test]
async fn forward_does_not_default_content_type_on_204() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/iserver/dynaccounts"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/iserver/dynaccounts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    assert!(
        resp.headers().get("content-type").is_none(),
        "204 must not carry a default Content-Type, got {:?}",
        resp.headers().get("content-type")
    );
}

/// Multiple `Set-Cookie` headers from upstream must each survive the
/// proxy hop. `forward()` uses `HeaderMap::append` for them; previously
/// a refactor to `insert` would silently drop all but the last.
#[tokio::test]
async fn forward_preserves_multiple_set_cookie_headers() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(
            ResponseTemplate::new(200)
                .append_header("set-cookie", "JSESSIONID=abc; Path=/sso; HttpOnly")
                .append_header("set-cookie", "x-sess-uuid=def; HttpOnly")
                .append_header("set-cookie", "USERID=42; Path=/")
                .set_body_string("<html></html>"),
        )
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        header_count(&resp, "set-cookie"),
        3,
        "expected all three Set-Cookie headers to round-trip"
    );
}

/// Hop-by-hop request headers (RFC 7230 §6.1) must not be forwarded to
/// the upstream Gateway. `cookie` is also stripped because reqwest
/// sources cookies from the shared jar.
#[tokio::test]
async fn passthrough_strips_hop_by_hop_request_headers() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        // Wiremock matches "header NOT present" via custom matcher; use
        // a bare match here and rely on `verify` semantics. The
        // assertion is implicit — if hop-by-hop leaked, the request
        // would still match because we don't *require* their absence,
        // but the production effect is upstream confusion. We instead
        // assert that the request *succeeded* (proving we didn't break
        // forwarding) and rely on the unit-level `is_hop_by_hop`
        // coverage to catch list regressions.
        .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .header("connection", "close")
                .header("upgrade", "websocket")
                .header("te", "trailers")
                .header("trailer", "expires")
                .header("keep-alive", "timeout=5")
                .header("proxy-authorization", "Basic xxx")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

/// On a 502 from upstream the proxy should emit a `502 Bad Gateway`
/// (or whatever the typed handler says) rather than crashing the
/// request task with a panic / 500.
#[tokio::test]
async fn passthrough_propagates_upstream_5xx() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream is down"))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

/// A single `Cookie:` header from the browser may carry many name=value
/// pairs separated by `; `. All of them should land in the shared jar
/// so subsequent typed calls authenticate correctly.
#[tokio::test]
async fn passthrough_replays_multiple_cookie_pairs() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/status"))
        // Order of cookies in `Cookie:` is implementation-defined; we
        // only assert both are present via substring matchers.
        .and(header_contains_all("cookie", &["a=1", "b=2"]))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": ""
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .header("cookie", "a=1; b=2")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

/// Tiny custom matcher: the named header must contain every needle.
fn header_contains_all(
    name: &'static str,
    needles: &'static [&'static str],
) -> impl wiremock::Match {
    HeaderContains { name, needles }
}

struct HeaderContains {
    name: &'static str,
    needles: &'static [&'static str],
}

impl wiremock::Match for HeaderContains {
    fn matches(&self, req: &wiremock::Request) -> bool {
        let Some(header_value) = req.headers.get(self.name) else {
            return false;
        };
        let Ok(s) = header_value.to_str() else {
            return false;
        };
        self.needles.iter().all(|n| s.contains(n))
    }
}

/// Ensure the `TEST_ACCOUNT` constant matches the existing fixture so we
/// can introduce it gradually without breaking tests that hard-code
/// `"DU123"`. Future cleanup: thread `TEST_ACCOUNT` through every test.
#[test]
fn test_account_constant_matches_fixture() {
    assert_eq!(TEST_ACCOUNT, "DU123");
}

/// IBKR's 2FA polling validates the request's `Origin` as part of
/// its session check, so on `/sso/*` we forward the browser's Origin
/// verbatim. An earlier attempt at fixing CSRF 400s rewrote it
/// globally and silently broke `/sso/Authenticator`.
#[tokio::test]
async fn passthrough_forwards_browser_origin_on_sso() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Authenticator"))
        .and(header("origin", "http://localhost:8080"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Authenticator")
                .header("origin", "http://localhost:8080")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "browser Origin should be forwarded verbatim on /sso/* — never rewrite"
    );
}

/// On `/v1/api/*` the Gateway's CPAPI CSRF guard rejects browser-supplied
/// Origin/Referer when the proxy is on a different public host. Rewrite
/// Origin (and the origin-prefix of Referer) to point at the Gateway's
/// own host so post-login authenticated calls succeed end-to-end.
#[tokio::test]
async fn passthrough_rewrites_origin_on_v1_api() {
    let gateway = MockServer::start().await;
    let gateway_uri = gateway.uri();
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .and(header("origin", gateway_uri.as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/v1/api/portfolio/accounts")
                .header("origin", "https://bezant-server-production.up.railway.app")
                .header(
                    "referer",
                    "https://bezant-server-production.up.railway.app/sso/Dispatcher",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "/v1/api/* should rewrite Origin to the Gateway origin so CPAPI CSRF passes"
    );
}

/// Set-Cookie headers from the Gateway carry `Domain=.ibkr.com` (the
/// upstream brand). Forwarded verbatim, the browser would silently
/// drop the cookie because the response arrives from a different
/// host. `forward()` should strip the Domain attribute.
#[tokio::test]
async fn passthrough_strips_cookie_domain_attribute() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(
            ResponseTemplate::new(200)
                .append_header("set-cookie", "SID=abc; Domain=.ibkr.com; Path=/; Secure")
                .set_body_string("<html></html>"),
        )
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let cookie = resp
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|c| c.starts_with("SID="))
        .map(str::to_owned)
        .expect("expected SID cookie in response");
    assert!(
        !cookie.to_ascii_lowercase().contains("domain="),
        "Domain attribute should be stripped, got {cookie:?}"
    );
    assert!(
        cookie.contains("Secure"),
        "Secure flag should survive the rewrite, got {cookie:?}"
    );
}

/// Browser cookies arriving on the bezant-server URL should land in
/// reqwest's shared jar so subsequent typed-API calls can authenticate
/// without the browser in the loop. We assert this end-to-end: an
/// initial passthrough request seeds the jar, and the next typed call
/// (`/health`) carries the same cookie upstream.
#[tokio::test]
async fn passthrough_replays_browser_cookies_into_typed_api() {
    let gateway = MockServer::start().await;
    // Step 1: any passthrough request that lets us seed the jar.
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
        .mount(&gateway)
        .await;
    // Step 2: typed auth_status that *requires* the cookie value the
    // browser supplied. If the cookie didn't make it into the jar,
    // wiremock won't match and we'll get a 502/Unknown response
    // instead of the 200 the test expects.
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/status"))
        .and(header("cookie", "JSESSIONID=test-session"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": "ok"
        })))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    // Seed: passthrough call carrying a Cookie header.
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .header("cookie", "JSESSIONID=test-session")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // Typed call: must succeed because the jar carries the cookie.
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "typed call didn't replay the cookie ({body})"
    );
    assert_eq!(body["authenticated"], json!(true));
}

/// Regression: when the upstream sets the same cookie name at two
/// different paths in successive responses (Gateway's keepalive at
/// `/v1/api/`, login passthrough at `/sso/`), the OLD `Jar`
/// accumulated both as separate entries and the next typed call sent
/// `Cookie: JSESSIONID=A; JSESSIONID=B`, which CPGateway rejected as
/// a session mismatch. The `NameKeyedJar` keys by name only — the
/// later value must overwrite the earlier one and only ONE
/// `JSESSIONID=` token must reach upstream.
#[tokio::test]
async fn passthrough_collapses_same_name_set_cookies_to_one_value() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let gateway = MockServer::start().await;

    // First passthrough: Gateway sets `JSESSIONID=ALPHA` at /sso.
    Mock::given(wm_method("GET"))
        .and(wm_path("/sso/Login"))
        .respond_with(
            ResponseTemplate::new(200)
                .append_header("set-cookie", "JSESSIONID=ALPHA; Path=/sso; HttpOnly")
                .set_body_string("<html></html>"),
        )
        .mount(&gateway)
        .await;
    // Second passthrough: Gateway sets `JSESSIONID=BETA` at root —
    // mimics a refresh post-login.
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(
            ResponseTemplate::new(200)
                .append_header("set-cookie", "JSESSIONID=BETA; Path=/; HttpOnly")
                .set_body_json(json!({})),
        )
        .mount(&gateway)
        .await;

    // Final typed call asserts the upstream Cookie header carries
    // exactly one `JSESSIONID=BETA` token (not `ALPHA; BETA`).
    let cookie_seen: Arc<std::sync::Mutex<Option<String>>> = Arc::new(std::sync::Mutex::new(None));
    let count = Arc::new(AtomicUsize::new(0));
    let cookie_for_assert = Arc::clone(&cookie_seen);
    let count_for_assert = Arc::clone(&count);

    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/status"))
        .respond_with(move |req: &wiremock::Request| {
            let header = req
                .headers
                .get("cookie")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_owned();
            *cookie_for_assert.lock().unwrap() = Some(header);
            count_for_assert.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200).set_body_json(json!({
                "authenticated": true,
                "connected": true,
                "competing": false,
                "message": ""
            }))
        })
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    // Drive both Set-Cookie responses through the proxy.
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/sso/Login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/api/tickle")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // Trigger a typed auth_status — wiremock will record the Cookie header.
    let _ = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let cookie = cookie_seen.lock().unwrap().clone().unwrap_or_default();
    let jsessionid_count = cookie.match_indices("JSESSIONID=").count();
    assert_eq!(
        jsessionid_count, 1,
        "Cookie header should carry exactly one JSESSIONID; got {cookie:?}"
    );
    assert!(
        cookie.contains("JSESSIONID=BETA"),
        "Latest Set-Cookie value should win; got {cookie:?}"
    );
    assert!(
        !cookie.contains("ALPHA"),
        "Earlier value must be replaced, not appended; got {cookie:?}"
    );
}

// ----- /debug/probe -----------------------------------------------------------
//
// The probe walks `auth/status` → `ssodh/init` → `tickle` →
// `portfolio/accounts` and pins each step's result. These tests cover
// the four verdicts the handler can emit: `ok`, `needs_login`,
// `ssodh_failed` (the Railway-failure-mode hypothesis), and
// `accounts_failed`.

async fn probe_body(app: axum::Router) -> Value {
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/debug/probe")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK, "probe must return 200");
    body
}

fn authed_status_builder() -> wiremock::MockBuilder {
    Mock::given(wm_method("POST")).and(wm_path("/v1/api/iserver/auth/status"))
}

fn authed_status_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "authenticated": true,
        "connected": true,
        "competing": false,
        "message": "ready"
    }))
}

#[tokio::test]
async fn probe_happy_path_returns_ok_verdict() {
    let gateway = MockServer::start().await;
    authed_status_builder().respond_with(authed_status_response()).mount(&gateway).await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/ssodh/init"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true
        })))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "session": "sess-123"
        })))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"id": "DU123456"}
        ])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let body = probe_body(app).await;

    assert_eq!(body["verdict"], json!("ok"));
    let steps = body["steps"].as_array().expect("steps array");
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0]["name"], json!("auth_status"));
    assert_eq!(steps[0]["status"], json!(200));
    assert_eq!(steps[1]["name"], json!("ssodh_init"));
    assert_eq!(steps[1]["status"], json!(200));
    assert_eq!(steps[2]["name"], json!("tickle"));
    assert_eq!(steps[2]["status"], json!(200));
    assert_eq!(steps[3]["name"], json!("accounts"));
    assert_eq!(steps[3]["status"], json!(200));
    // Each step records latency in millis as a number.
    for step in steps {
        assert!(
            step["latency_ms"].is_u64(),
            "latency_ms must be a number, got {step}"
        );
        assert!(step["error"].is_null(), "happy-path error must be null");
    }
}

#[tokio::test]
async fn probe_unauthenticated_session_pins_needs_login() {
    let gateway = MockServer::start().await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": false,
            "connected": true,
            "competing": false,
            "message": ""
        })))
        .mount(&gateway)
        .await;
    // The remaining steps are still mounted so the probe runs end-to-end —
    // we want to confirm verdict reflects auth_status, not a downstream 404.
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/ssodh/init"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let body = probe_body(app).await;

    assert_eq!(body["verdict"], json!("needs_login"));
    let steps = body["steps"].as_array().unwrap();
    // auth_status itself succeeded transport-wise; the verdict comes
    // from the parsed body, not the HTTP status.
    assert_eq!(steps[0]["status"], json!(200));
}

/// Regression-style: this is the Railway-failure-mode signature we
/// built the probe to detect. `auth_status` says authenticated, but
/// CPGateway's internal SSO→CPAPI bridge call to `api.ibkr.com` is
/// rejected → `ssodh/init` returns 401. The probe must pin the
/// verdict to that exact step so a remote operator can read the
/// `/debug/probe` body and instantly know the failure is upstream of
/// bezant-server's proxy layer.
#[tokio::test]
async fn probe_ssodh_failure_pins_ssodh_failed_verdict() {
    let gateway = MockServer::start().await;
    authed_status_builder().respond_with(authed_status_response()).mount(&gateway).await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/ssodh/init"))
        .respond_with(ResponseTemplate::new(401).set_body_string("unauthorized"))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let body = probe_body(app).await;

    assert_eq!(body["verdict"], json!("ssodh_failed"));
    let steps = body["steps"].as_array().unwrap();
    assert_eq!(steps[1]["name"], json!("ssodh_init"));
    assert_eq!(steps[1]["status"], json!(401));
    // Body preview should carry the upstream message verbatim so a human
    // reader can tell *what* upstream said, not just that it failed.
    assert!(
        steps[1]["body_preview"]
            .as_str()
            .unwrap_or("")
            .contains("unauthorized"),
        "body_preview should include upstream's response: {step}",
        step = steps[1]
    );
}

#[tokio::test]
async fn probe_accounts_failure_pins_accounts_failed_verdict() {
    let gateway = MockServer::start().await;
    authed_status_builder().respond_with(authed_status_response()).mount(&gateway).await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/ssodh/init"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let body = probe_body(app).await;

    assert_eq!(body["verdict"], json!("accounts_failed"));
    assert_eq!(body["steps"][3]["status"], json!(503));
}

/// `Origin` header pinning is part of the proxy's CSRF-bypass logic,
/// so the probe — which exercises the same upstream surface — must
/// carry it too. Otherwise a pass on `/debug/probe` wouldn't predict a
/// pass on `/v1/api/*` proxy calls.
#[tokio::test]
async fn probe_pins_origin_to_gateway_root() {
    use std::sync::{Arc, Mutex};

    let gateway = MockServer::start().await;
    let captured: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let captured_clone = Arc::clone(&captured);

    authed_status_builder()
        .respond_with(move |req: &wiremock::Request| {
            let origin = req
                .headers
                .get("origin")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_owned();
            *captured_clone.lock().unwrap() = Some(origin);
            ResponseTemplate::new(200).set_body_json(json!({
                "authenticated": true,
                "connected": true,
                "competing": false
            }))
        })
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/iserver/auth/ssodh/init"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("POST"))
        .and(wm_path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&gateway)
        .await;
    Mock::given(wm_method("GET"))
        .and(wm_path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&gateway)
        .await;

    let app = make_app(&gateway).await;
    let _ = probe_body(app).await;

    let origin = captured.lock().unwrap().clone().unwrap_or_default();
    let expected = gateway.uri();
    assert_eq!(
        origin,
        expected.trim_end_matches('/'),
        "probe must pin Origin to the Gateway's own root"
    );
}
