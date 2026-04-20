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
