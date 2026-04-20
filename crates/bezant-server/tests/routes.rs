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
