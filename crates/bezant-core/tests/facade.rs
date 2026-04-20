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
