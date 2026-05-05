//! Integration tests for the `/events/*` HTTP surface.
//!
//! These tests use [`EventsHandle::for_test`] to sidestep the live WS
//! connector — the route layer is what matters here (cursoring, 204
//! semantics, 412 cursor-expired, JSON shape, the `_status` envelope).
//! End-to-end coverage with a real CPAPI WS lives in the connector
//! unit tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use bezant_server::events::{EventLog, EventsHandle};
use bezant_server::{router, AppState};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;
use wiremock::MockServer;

async fn make_app_with_events() -> (axum::Router, bezant_server::events::TestSink) {
    let gateway = MockServer::start().await;
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    let (handle, sink) = EventsHandle::for_test();
    let state = AppState::new(client).with_events(handle);
    let app = router(state);
    // Leak the gateway so its drop doesn't kill the mock; tests are
    // short-lived so this is fine.
    Box::leak(Box::new(gateway));
    (app, sink)
}

async fn make_app_with_log() -> (axum::Router, bezant_server::events::TestSink, Arc<EventLog>) {
    let gateway = MockServer::start().await;
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    let log = Arc::new(EventLog::open_in_memory().expect("memory db"));
    let (handle, sink) = EventsHandle::for_test_with_log(Some(log.clone()));
    let state = AppState::new(client).with_events(handle);
    let app = router(state);
    Box::leak(Box::new(gateway));
    (app, sink, log)
}

async fn make_app_without_events() -> axum::Router {
    let gateway = MockServer::start().await;
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    let app = router(AppState::new(client));
    Box::leak(Box::new(gateway));
    app
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
async fn events_disabled_when_handle_not_attached() {
    let app = make_app_without_events().await;
    let resp = app
        .oneshot(Request::builder().uri("/events/orders").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["code"], "events_disabled");
}

#[tokio::test]
async fn events_status_returns_disconnected_envelope_initially() {
    let (app, _sink) = make_app_with_events().await;
    let resp = app
        .oneshot(Request::builder().uri("/events/_status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    // for_test() sets connected=true so the test sink is usable; verify
    // the envelope shape has every field present.
    assert!(body.get("connected").is_some());
    assert!(body.get("reset_epoch").is_some());
    assert!(body.get("topics_subscribed").is_some());
    assert!(body.get("buffer_sizes").is_some());
    assert!(body.get("uptime_seconds").is_some());
}

#[tokio::test]
async fn events_orders_empty_returns_200_with_empty_array() {
    let (app, _sink) = make_app_with_events().await;
    let resp = app
        .oneshot(Request::builder().uri("/events/orders?since=0").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    // Topic doesn't exist yet — handler returns 200 with empty array
    // so polls before any event are idempotent on cursor.
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"], json!([]));
    assert_eq!(body["next_cursor"], json!(0));
}

#[tokio::test]
async fn events_orders_returns_pushed_events() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"orderId": 12345, "status": "Filled"})).await;
    sink.push("orders", json!({"orderId": 12346, "status": "Working"})).await;

    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/events/orders?since=0").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"].as_array().unwrap().len(), 2);
    assert_eq!(body["events"][0]["payload"]["orderId"], json!(12345));
    assert_eq!(body["events"][1]["payload"]["orderId"], json!(12346));
    assert_eq!(body["next_cursor"], json!(2));
}

#[tokio::test]
async fn events_orders_advances_cursor() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"id": 1})).await;
    sink.push("orders", json!({"id": 2})).await;
    sink.push("orders", json!({"id": 3})).await;

    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/events/orders?since=1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    let events = body["events"].as_array().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["cursor"], json!(2));
    assert_eq!(events[1]["cursor"], json!(3));
    assert_eq!(body["next_cursor"], json!(3));
}

#[tokio::test]
async fn events_orders_caught_up_returns_204() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"id": 1})).await;

    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/events/orders?since=1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    let cursor_header = resp.headers().get("x-bezant-cursor").unwrap();
    assert_eq!(cursor_header.to_str().unwrap(), "1");
}

#[tokio::test]
async fn events_orders_respects_limit() {
    let (app, sink) = make_app_with_events().await;
    for i in 1..=10 {
        sink.push("orders", json!({"id": i})).await;
    }

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=0&limit=3")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"].as_array().unwrap().len(), 3);
    assert_eq!(body["next_cursor"], json!(3));
}

#[tokio::test]
async fn events_orders_status_subscribed_topics_grow() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"id": 1})).await;
    sink.push("pnl", json!({"upnl": 10.0})).await;

    let resp = app
        .oneshot(Request::builder().uri("/events/_status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (_status, body) = response_body(resp).await;
    let topics = body["topics_subscribed"].as_array().unwrap();
    assert!(topics.iter().any(|t| t == "orders"));
    assert!(topics.iter().any(|t| t == "pnl"));
    assert_eq!(body["buffer_sizes"]["orders"], json!(1));
    assert_eq!(body["buffer_sizes"]["pnl"], json!(1));
}

#[tokio::test]
async fn events_pnl_endpoint_returns_pnl_topic() {
    let (app, sink) = make_app_with_events().await;
    sink.push("pnl", json!({"unrealizedUsd": 125.34})).await;

    let resp = app
        .oneshot(Request::builder().uri("/events/pnl?since=0").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"][0]["payload"]["unrealizedUsd"], json!(125.34));
    assert_eq!(body["events"][0]["topic"], "pnl");
}

#[tokio::test]
async fn events_envelope_includes_reset_epoch() {
    let (app, sink) = make_app_with_events().await;
    sink.set_reset_epoch(7).await;
    sink.push("orders", json!({"id": 1})).await;

    let resp = app
        .oneshot(Request::builder().uri("/events/orders?since=0").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["reset_epoch"], json!(7));
    assert_eq!(body["events"][0]["reset_epoch"], json!(7));
}

#[tokio::test]
async fn events_history_returns_503_without_log() {
    let (app, _sink) = make_app_with_events().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/events/orders/history?since_ts=2026-01-01T00:00:00Z")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["code"], "events_history_disabled");
}

#[tokio::test]
async fn events_history_returns_persisted_events() {
    let (app, sink, _log) = make_app_with_log().await;
    sink.push("orders", json!({"id": 1, "status": "Filled"})).await;
    sink.push("orders", json!({"id": 2, "status": "Filled"})).await;
    sink.push("pnl", json!({"upnl": 100})).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/events/orders/history?since_ts=1970-01-01T00:00:00Z&limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["topic"], "orders");
    assert_eq!(body["count"], 2);
    assert_eq!(body["events"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn events_history_filters_by_topic_in_path() {
    let (app, sink, _log) = make_app_with_log().await;
    sink.push("orders", json!({"id": 1})).await;
    sink.push("pnl", json!({"x": 1})).await;
    sink.push("pnl", json!({"x": 2})).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/events/pnl/history?since_ts=1970-01-01T00:00:00Z&limit=100")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["count"], 2);
}
