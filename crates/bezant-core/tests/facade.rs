//! End-to-end tests for the ergonomic facade exposed by `bezant::Client`.
//!
//! Every test spins up a fresh wiremock server, mounts the CPAPI endpoints
//! we care about, and exercises the facade against it. This keeps the
//! tests free of any live-network dependency while still verifying that
//! our URL construction, response projection, and error mapping all line
//! up with the shape IBKR's Gateway actually speaks.

use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(base: &str) -> bezant::Client {
    bezant::Client::builder(base)
        // The wiremock server is plain HTTP with a real cert from nothing —
        // just turn off TLS verification, same posture as against the real
        // Gateway.
        .accept_invalid_certs(true)
        .build()
        .expect("client")
}

#[tokio::test]
async fn auth_status_ok_projects_bool_fields() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": "",
        })))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let status = client.auth_status().await.expect("auth_status");
    assert!(status.authenticated);
    assert!(status.connected);
    assert!(!status.competing);
}

#[tokio::test]
async fn auth_status_unauthorized_maps_to_typed_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let err = client.auth_status().await.expect_err("expected 401");
    assert!(matches!(err, bezant::Error::NotAuthenticated));
}

#[tokio::test]
async fn health_rejects_disconnected_sessions() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": false,
            "competing": false,
            "message": "",
        })))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let err = client.health().await.expect_err("expected NoSession");
    assert!(
        matches!(err, bezant::Error::NoSession),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn health_rejects_unauthenticated_sessions() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": false,
            "connected": true,
            "competing": false,
            "message": "",
        })))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let err = client
        .health()
        .await
        .expect_err("expected NotAuthenticated");
    assert!(
        matches!(err, bezant::Error::NotAuthenticated),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn tickle_returns_session_on_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "session": "sess-abc123",
            "ssoExpires": 180000,
            "iserver": {
                "authStatus": {
                    "authenticated": true,
                    "connected": true
                }
            }
        })))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let resp = client.tickle().await.expect("tickle");
    assert_eq!(resp.session.as_deref(), Some("sess-abc123"));
}

#[tokio::test]
async fn symbol_cache_hits_after_first_lookup() {
    let server = MockServer::start().await;
    // Expect exactly ONE contract search call across two cache lookups.
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"conid": "265598", "companyName": "Apple Inc"}
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let cache = bezant::SymbolCache::new(client);
    let a = cache.conid_for("AAPL").await.expect("first lookup");
    let b = cache.conid_for("AAPL").await.expect("second lookup");
    assert_eq!(a, 265_598);
    assert_eq!(a, b);
    server.verify().await;
}

#[tokio::test]
async fn symbol_cache_forget_triggers_refetch() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"conid": "265598"}
        ])))
        .expect(2) // once before forget, once after
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let cache = bezant::SymbolCache::new(client);
    let _ = cache.conid_for("AAPL").await.expect("first");
    cache.forget("AAPL");
    let _ = cache.conid_for("AAPL").await.expect("second");
    server.verify().await;
}

#[tokio::test]
async fn symbol_cache_surfaces_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let cache = bezant::SymbolCache::new(client);
    let err = cache
        .conid_for("AAPL")
        .await
        .expect_err("expected unauthorised");
    assert!(matches!(err, bezant::Error::NotAuthenticated));
}

#[tokio::test]
async fn symbol_cache_errors_on_missing_conid_field() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"companyName": "Apple Inc"}  // no conid field
        ])))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let cache = bezant::SymbolCache::new(client);
    let err = cache.conid_for("AAPL").await.expect_err("expected error");
    assert!(err.to_string().contains("no conid") || err.to_string().contains("has no conid"));
}

#[tokio::test]
async fn all_positions_stops_on_short_page() {
    let server = MockServer::start().await;
    let page0: Vec<_> = (0..30)
        .map(|i| json!({"conid": 1000 + i, "position": 1.0}))
        .collect();
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(page0)))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{"conid": 9999}])))
        .mount(&server)
        .await;
    // Page 2 should never be requested.
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/2"))
        .respond_with(ResponseTemplate::new(500))
        .expect(0)
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let positions = client.all_positions("DU123").await.expect("positions");
    assert_eq!(positions.len(), 31);
    server.verify().await;
}

#[tokio::test]
async fn all_positions_maps_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/positions/0"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let err = client
        .all_positions("DU123")
        .await
        .expect_err("expected unauthorised");
    assert!(matches!(err, bezant::Error::NotAuthenticated));
}

#[tokio::test]
async fn tickle_unauthorized_maps_to_typed_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = client_for(&format!("{}/v1/api", server.uri()));
    let err = client.tickle().await.expect_err("expected 401");
    assert!(matches!(err, bezant::Error::NotAuthenticated));
}
