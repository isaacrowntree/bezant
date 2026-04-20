//! Integration tests for the `bezant` CLI.
//!
//! Tests spawn the real compiled binary via `assert_cmd`, pointed at a
//! wiremock server, so we exercise clap parsing, stdout/stderr routing,
//! exit codes, and JSON output shape end-to-end.

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn gateway_base(server: &MockServer) -> String {
    format!("{}/v1/api", server.uri())
}

#[tokio::test]
async fn help_exits_zero_and_mentions_key_subcommands() {
    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    cmd.arg("--help").assert().success().stdout(
        predicate::str::contains("health")
            .and(predicate::str::contains("accounts"))
            .and(predicate::str::contains("positions"))
            .and(predicate::str::contains("conid")),
    );
}

#[tokio::test]
async fn health_outputs_auth_status_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true,
            "connected": true,
            "competing": false,
            "message": ""
        })))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args(["--gateway-url", &gateway_base(&server), "health"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    assert_eq!(body["authenticated"], json!(true));
    assert_eq!(body["connected"], json!(true));
}

#[tokio::test]
async fn accounts_prints_array_from_gateway() {
    let server = MockServer::start().await;
    // accounts subcommand first checks via typed client, then passes through.
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"accountId": "DU1", "currency": "USD"}
        ])))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args(["--gateway-url", &gateway_base(&server), "accounts"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    assert_eq!(body[0]["accountId"], json!("DU1"));
}

#[tokio::test]
async fn pretty_flag_adds_newlines() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "authenticated": true, "connected": true
        })))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let out = cmd
        .args([
            "--gateway-url",
            &gateway_base(&server),
            "--pretty",
            "health",
        ])
        .output()
        .expect("output");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('\n') && stdout.contains("  "),
        "pretty output should have newlines + indentation: {stdout}"
    );
}

#[tokio::test]
async fn summary_prints_gateway_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/api/portfolio/DU123/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "netliquidation": {"amount": 1234.5, "currency": "USD"}
        })))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args(["--gateway-url", &gateway_base(&server), "summary", "DU123"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    assert_eq!(body["netliquidation"]["amount"], json!(1234.5));
}

#[tokio::test]
async fn positions_paginates_and_flattens() {
    let server = MockServer::start().await;
    let page0: Vec<Value> = (0..30).map(|i| json!({"conid": 1000 + i})).collect();
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

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args([
            "--gateway-url",
            &gateway_base(&server),
            "positions",
            "DU123",
        ])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 31);
    assert_eq!(arr[30]["conid"], json!(9999));
}

#[tokio::test]
async fn conid_resolves_symbol() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/secdef/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"conid": "265598", "companyName": "Apple Inc"}
        ])))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args(["--gateway-url", &gateway_base(&server), "conid", "AAPL"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    assert_eq!(body["symbol"], json!("AAPL"));
    assert_eq!(body["conid"], json!(265_598));
}

#[tokio::test]
async fn tickle_reports_session_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/tickle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "session": "sess-xyz"
        })))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    let assertion = cmd
        .args(["--gateway-url", &gateway_base(&server), "tickle"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    let body: Value = serde_json::from_str(stdout.trim()).expect("json");
    assert_eq!(body["session"], json!("sess-xyz"));
}

#[tokio::test]
async fn health_reports_error_on_401() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/api/iserver/auth/status"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let mut cmd = Command::cargo_bin("bezant").expect("binary");
    cmd.args(["--gateway-url", &gateway_base(&server), "health"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not authenticated"));
}
