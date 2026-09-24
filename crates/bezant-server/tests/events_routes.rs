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
        .oneshot(
            Request::builder()
                .uri("/events/orders")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/_status")
                .body(Body::empty())
                .unwrap(),
        )
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
    // What CPAPI honoured, as distinct from what we asked for — the field a
    // fill confirmer reads to know whether waiting on `orders` is pointless.
    assert!(body.get("subscriptions").is_some());
    assert_eq!(body["subscribe_refusals"], json!(0));
    assert_eq!(body["session_rollovers"], json!(0));
    assert!(body.get("uptime_seconds").is_some());
}

#[tokio::test]
async fn events_orders_empty_returns_200_with_empty_array() {
    let (app, _sink) = make_app_with_events().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let (status, body) = response_body(resp).await;
    // Topic doesn't exist yet — handler returns 200 with empty array,
    // this process's cursor and its live epoch.
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"], json!([]));
    assert_eq!(body["next_cursor"], json!(0));
    assert_eq!(body["reset_epoch"], json!(1));
}

#[tokio::test]
async fn events_orders_returns_pushed_events() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"orderId": 12345, "status": "Filled"}))
        .await;
    sink.push("orders", json!({"orderId": 12346, "status": "Working"}))
        .await;

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=0")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=1")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=1")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/_status")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/pnl?since=0")
                .body(Body::empty())
                .unwrap(),
        )
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
        .oneshot(
            Request::builder()
                .uri("/events/orders?since=0")
                .body(Body::empty())
                .unwrap(),
        )
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
    sink.push("orders", json!({"id": 1, "status": "Filled"}))
        .await;
    sink.push("orders", json!({"id": 2, "status": "Filled"}))
        .await;
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

async fn app_on(handle: EventsHandle) -> axum::Router {
    let gateway = MockServer::start().await;
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    Box::leak(Box::new(gateway));
    router(AppState::new(client).with_events(handle))
}

async fn get(app: &axum::Router, uri: &str) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

/// The restart bug: cursors used to restart at 1 in every process, so a
/// client holding cursor 3 from the last run was told "caught up" (204) —
/// and saw nothing — until the new process had pushed 3 events on that
/// topic. For `orders` that is days of fills. Now a restart seeds cursors
/// above the previous run's, so the old cursor reads the new events.
#[tokio::test]
async fn a_restart_does_not_rewind_cursors_below_a_previous_clients() {
    let boot = 1_790_000_000_000;
    let (epoch1, base1) = bezant_server::events::connector::boot_seeds(boot, None);
    let (h1, sink1) = EventsHandle::for_test_seeded(None, epoch1, base1);
    let app1 = app_on(h1).await;
    for i in 0..3 {
        sink1.push("orders", json!({ "orderId": i })).await;
    }
    let (_, body) = response_body(get(&app1, "/events/orders?since=0").await).await;
    let client_cursor = body["next_cursor"].as_u64().unwrap();

    // The process restarts a minute later and sees one fill.
    let (epoch2, base2) = bezant_server::events::connector::boot_seeds(boot + 60_000, None);
    let (h2, sink2) = EventsHandle::for_test_seeded(None, epoch2, base2);
    let app2 = app_on(h2).await;
    sink2
        .push("orders", json!({"orderId": 99, "status": "Filled"}))
        .await;

    let resp = get(&app2, &format!("/events/orders?since={client_cursor}")).await;
    let (status, body) = response_body(resp).await;
    assert_eq!(status, StatusCode::OK, "not 204: the fill must be seen");
    assert_eq!(body["events"][0]["payload"]["orderId"], json!(99));
    assert!(body["next_cursor"].as_u64().unwrap() > client_cursor);
    assert_ne!(
        body["reset_epoch"],
        json!(epoch1),
        "the client sees the reset"
    );
    // Still exact in a JavaScript number.
    assert!(body["next_cursor"].as_u64().unwrap() < (1u64 << 53));
}

/// Where the seed cannot help — the clock came up behind the last run —
/// the old cursor is from the future, and the answer is 412, which the
/// fund already handles by resyncing to `head_cursor - 1`.
#[tokio::test]
async fn a_restart_behind_the_clock_answers_the_old_cursor_with_412() {
    let (h1, sink1) = EventsHandle::for_test_seeded(None, 5_000, 5_000_000);
    let app1 = app_on(h1).await;
    sink1.push("orders", json!({"orderId": 1})).await;
    let (_, body) = response_body(get(&app1, "/events/orders?since=0").await).await;
    let client_cursor = body["next_cursor"].as_u64().unwrap();

    let (h2, sink2) = EventsHandle::for_test_seeded(None, 4_000, 4_000_000);
    let app2 = app_on(h2).await;
    sink2.push("orders", json!({"orderId": 2})).await;

    let (status, body) =
        response_body(get(&app2, &format!("/events/orders?since={client_cursor}")).await).await;
    assert_eq!(status, StatusCode::PRECONDITION_FAILED);
    assert_eq!(body["code"], json!("cursor_expired"));
    assert_eq!(body["head_cursor"], json!(4_000_000));
    assert_eq!(body["reset_epoch"], json!(4_000));

    // Resyncing as the fund does reads the new event.
    let resync = body["head_cursor"].as_u64().unwrap() - 1;
    let (status, body) =
        response_body(get(&app2, &format!("/events/orders?since={resync}")).await).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"][0]["payload"]["orderId"], json!(2));
}

/// A topic with no ring yet (market data before its first tick, `gap`
/// before the first reconnect) used to echo the caller's cursor back, which
/// kept a stale cursor alive until the first event landed below it.
#[tokio::test]
async fn a_lazy_ring_does_not_echo_the_callers_cursor() {
    let (h, sink) = EventsHandle::for_test_seeded(None, 7, 1_000);
    let app = app_on(h).await;

    let (status, body) = response_body(get(&app, "/events/gap?since=17").await).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"], json!([]));
    assert_eq!(
        body["next_cursor"],
        json!(999),
        "this process's cursor space"
    );
    assert_eq!(body["reset_epoch"], json!(7));

    // Polling on from what it was told, the client sees the first event.
    sink.push("gap", json!({"reason": "reconnected_after_disconnect"}))
        .await;
    let (status, body) = response_body(get(&app, "/events/gap?since=999").await).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["events"][0]["cursor"], json!(1_000));

    // A cursor this process never issued, on a lazy ring, is a 412.
    let (status, _) = response_body(get(&app, "/events/pnl?since=5000").await).await;
    assert_eq!(status, StatusCode::PRECONDITION_FAILED);
}

#[tokio::test]
async fn caught_up_204_carries_the_live_epoch() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"id": 1})).await;
    sink.set_reset_epoch(9).await;

    let resp = get(&app, "/events/orders?since=1").await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    assert_eq!(resp.headers()["x-bezant-cursor"], "1");
    assert_eq!(resp.headers()["x-bezant-reset-epoch"], "9");
}

#[tokio::test]
async fn a_batch_straddling_a_reconnect_reports_the_new_epoch() {
    let (app, sink) = make_app_with_events().await;
    sink.push("orders", json!({"id": 1})).await;
    sink.set_reset_epoch(2).await;
    sink.push("orders", json!({"id": 2})).await;

    let (status, body) = response_body(get(&app, "/events/orders?since=0").await).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["reset_epoch"], json!(2));
    assert_eq!(body["events"][0]["reset_epoch"], json!(1));
}

async fn app_with_token(handle: EventsHandle, token: &str) -> axum::Router {
    let gateway = MockServer::start().await;
    let client = bezant::Client::builder(format!("{}/v1/api", gateway.uri()))
        .accept_invalid_certs(true)
        .build()
        .expect("client");
    Box::leak(Box::new(gateway));
    router(AppState::with_debug_token(client, token).with_events(handle))
}

async fn post(app: &axum::Router, uri: &str, token: Option<&str>) -> axum::http::Response<Body> {
    let mut req = Request::builder().method("POST").uri(uri);
    if let Some(t) = token {
        req = req.header("x-bezant-debug-token", t);
    }
    app.clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn reconnect_goes_through_the_connector_command_channel() {
    let (handle, sink) = EventsHandle::for_test();
    sink.serve_commands();
    let app = app_with_token(handle, "s3cret").await;

    let (status, body) =
        response_body(post(&app, "/events/_reconnect", Some("s3cret")).await).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["code"], json!("reconnect_requested"));
    assert_eq!(body["was_connected"], json!(true));
    assert_eq!(sink.reconnect_requests(), 1);
}

#[tokio::test]
async fn reconnect_is_gated_by_the_debug_token() {
    let (handle, sink) = EventsHandle::for_test();
    sink.serve_commands();
    let app = app_with_token(handle, "s3cret").await;

    let resp = post(&app, "/events/_reconnect", Some("wrong")).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let resp = post(&app, "/events/_reconnect", None).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        sink.reconnect_requests(),
        0,
        "nothing reached the connector"
    );

    // No token configured: the endpoint does not exist.
    let (handle, sink) = EventsHandle::for_test();
    sink.serve_commands();
    let app = app_on(handle).await;
    let resp = post(&app, "/events/_reconnect", Some("s3cret")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(sink.reconnect_requests(), 0);
}
