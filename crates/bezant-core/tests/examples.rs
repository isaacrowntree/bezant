//! Snapshot tests driven by real IBKR response examples.
//!
//! The fixtures under `tests/fixtures/examples.json` are extracted directly
//! from the vendored OpenAPI spec by `scripts/extract-examples.py`. Each
//! test verifies that the generated Rust response type can still deserialise
//! the upstream example payload — so when IBKR revises their spec and we
//! regenerate `bezant-api`, this file catches silent type-shape drift.
//!
//! The fixtures are grouped by `operationId`. For each interesting
//! operation we want covered, the test below looks up its examples and
//! runs `serde_json::from_value::<ConcreteType>(payload)`. Add more
//! operations by extending `cases!` below and re-running the extractor.

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Fixture {
    #[serde(rename = "operationId")]
    operation_id: String,
    value: Value,
    #[serde(rename = "exampleName")]
    example_name: String,
}

fn load_fixtures() -> HashMap<String, Vec<Fixture>> {
    let raw = include_str!("fixtures/examples.json");
    let items: Vec<Fixture> = serde_json::from_str(raw).expect("fixtures load");
    let mut by_op: HashMap<String, Vec<Fixture>> = HashMap::new();
    for item in items {
        by_op
            .entry(item.operation_id.clone())
            .or_default()
            .push(item);
    }
    by_op
}

/// Round-trip every fixture under `op_id` through `T::deserialize`.
fn assert_deserialises<T: DeserializeOwned>(op_id: &str, fixtures: &HashMap<String, Vec<Fixture>>) {
    let cases = fixtures
        .get(op_id)
        .unwrap_or_else(|| panic!("no fixtures for operationId {op_id}"));
    assert!(
        !cases.is_empty(),
        "fixtures/examples.json has no entries for {op_id}"
    );
    for case in cases {
        let result: Result<T, _> = serde_json::from_value(case.value.clone());
        if let Err(e) = &result {
            panic!(
                "deserialise failed for {op_id}/{}: {e}\n  payload: {}",
                case.example_name, case.value
            );
        }
    }
}

#[test]
fn portfolio_summary_example_deserialises() {
    let fixtures = load_fixtures();
    assert_deserialises::<bezant_api::PortfolioSummary>("getPortfolioSummary", &fixtures);
}

#[test]
fn account_summary_example_deserialises() {
    let fixtures = load_fixtures();
    assert_deserialises::<bezant_api::AccountSummaryResponse>("getAccountSummary", &fixtures);
}

#[test]
fn paginated_positions_example_deserialises() {
    let fixtures = load_fixtures();
    // `getPaginatedPositions` returns `Vec<IndividualPosition>` in the spec
    // (the schemaRef is empty because it's declared inline as an array type).
    assert_deserialises::<Vec<bezant_api::IndividualPosition>>("getPaginatedPositions", &fixtures);
}

/// Makes sure the extractor populated the fixtures file with what we expect.
/// Guards against someone regenerating with the wrong `--only` list.
#[test]
fn fixtures_cover_expected_operations() {
    let fixtures = load_fixtures();
    for op in [
        "getPortfolioSummary",
        "getAccountSummary",
        "getPaginatedPositions",
    ] {
        assert!(
            fixtures.contains_key(op),
            "fixtures/examples.json missing operation {op}"
        );
    }
}
