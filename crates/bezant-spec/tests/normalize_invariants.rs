//! Post-normalisation invariant tests for the vendored 3.0 spec.
//!
//! `scripts/normalize-spec.py` applies 13 transformations to the raw
//! IBKR OpenAPI spec to make it digestible by `oas3-gen` /
//! `progenitor`. Re-implementing those 13 steps in Rust just to
//! verify them would be a 2-3 day port exercise of duplicated logic.
//! Instead, this file pins the **post-conditions** each step is
//! supposed to establish, asserted directly against the vendored
//! `ibkr-openapi.json` shipped on disk.
//!
//! * If someone re-runs the Python normaliser and it produces output
//!   that violates an invariant, this test fails.
//! * If someone hand-edits the vendored spec in a way that re-introduces
//!   a normaliser-target shape, this test fails.
//! * If a future change wants to relax an invariant, the test that
//!   pinned it has to be edited too — surfaces the change in PR review.
//!
//! The 13 steps come from `scripts/normalize-spec.py:1-55`. Each test
//! below carries a comment noting which step it pins.

use serde_json::Value;

const SPEC: &str = bezant_spec::SPEC_JSON;
const SPEC_3_1: &str = bezant_spec::SPEC_JSON_3_1;

const HTTP_METHODS: &[&str] = &[
    "get", "post", "put", "delete", "patch", "options", "head", "trace",
];

fn spec() -> Value {
    serde_json::from_str(SPEC).expect("vendored 3.0 spec must parse")
}

fn spec_3_1() -> Value {
    serde_json::from_str(SPEC_3_1).expect("vendored 3.1 spec must parse")
}

fn for_each_op(spec: &Value, mut f: impl FnMut(&str, &str, &Value)) {
    let Some(paths) = spec["paths"].as_object() else {
        return;
    };
    for (path, item) in paths {
        let Some(item) = item.as_object() else {
            continue;
        };
        for (method, op) in item {
            if HTTP_METHODS.contains(&method.to_ascii_lowercase().as_str()) {
                f(path, method, op);
            }
        }
    }
}

fn walk_objects<'a>(value: &'a Value, mut visit: impl FnMut(&'a Value)) {
    fn walk<'a>(value: &'a Value, visit: &mut dyn FnMut(&'a Value)) {
        visit(value);
        match value {
            Value::Object(o) => {
                for v in o.values() {
                    walk(v, visit);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    walk(v, visit);
                }
            }
            _ => {}
        }
    }
    walk(value, &mut visit);
}

// Step 1: strip null entries from security scope arrays.
#[test]
fn no_null_security_scopes() {
    let s = spec();
    walk_objects(&s, |v| {
        if let Some(security) = v.get("security").and_then(Value::as_array) {
            for req in security {
                if let Some(req) = req.as_object() {
                    for (scheme, scopes) in req {
                        if let Some(scopes) = scopes.as_array() {
                            for (i, scope) in scopes.iter().enumerate() {
                                assert!(
                                    !scope.is_null(),
                                    "null scope at security[{scheme}][{i}] — step 1 missed it"
                                );
                            }
                        }
                    }
                }
            }
        }
    });
}

// Step 2: every operation has an operationId.
#[test]
fn every_operation_has_operation_id() {
    let s = spec();
    let mut missing = Vec::new();
    for_each_op(&s, |path, method, op| {
        let id = op.get("operationId").and_then(Value::as_str).unwrap_or("");
        if id.is_empty() {
            missing.push(format!("{} {}", method.to_uppercase(), path));
        }
    });
    assert!(
        missing.is_empty(),
        "operations without operationId — step 2 missed: {missing:?}"
    );
}

// Step 3: operationIds are unique.
#[test]
fn operation_ids_are_unique() {
    let s = spec();
    let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut dupes = Vec::new();
    for_each_op(&s, |path, method, op| {
        let Some(id) = op.get("operationId").and_then(Value::as_str) else {
            return;
        };
        let here = format!("{} {}", method.to_uppercase(), path);
        if let Some(prev) = seen.insert(id.to_owned(), here.clone()) {
            dupes.push(format!("{id}: {prev} <-> {here}"));
        }
    });
    assert!(dupes.is_empty(), "duplicate operationIds — step 3 missed: {dupes:?}");
}

// Step 4: no enums whose variants collide when stripped to identifiers.
// (Anything with `>=`, `<=`, `>`, `<`, `==` should have been demoted to
// a plain string with the variants moved into `description`.)
#[test]
fn no_ambiguous_enum_variants() {
    let s = spec();
    walk_objects(&s, |v| {
        let (Some(t), Some(variants)) = (
            v.get("type").and_then(Value::as_str),
            v.get("enum").and_then(Value::as_array),
        ) else {
            return;
        };
        if t != "string" {
            return;
        }
        let idents: Vec<String> = variants
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .collect::<String>()
            })
            .collect();
        let unique: std::collections::HashSet<&String> = idents.iter().collect();
        assert!(
            idents.len() == unique.len() && !idents.iter().all(String::is_empty),
            "ambiguous enum still in spec — step 4 missed: variants={variants:?}"
        );
    });
}

// Step 5: no exotic content types — every body content key is one of
// the supported set.
#[test]
fn only_supported_content_types() {
    const SUPPORTED: &[&str] = &[
        "application/json",
        "application/json; charset=utf-8",
        "application/problem+json",
        "application/x-www-form-urlencoded",
        "application/pdf",
        "application/octet-stream",
        "text/plain",
        "*/*",
    ];
    let s = spec();
    walk_objects(&s, |v| {
        if let Some(content) = v.get("content").and_then(Value::as_object) {
            for ct in content.keys() {
                assert!(
                    SUPPORTED.contains(&ct.as_str()),
                    "exotic content type still in spec — step 5 missed: {ct}"
                );
            }
        }
    });
}

// Step 6: enum variants are compatible with the schema's declared type.
#[test]
fn enum_variants_match_declared_type() {
    let s = spec();
    walk_objects(&s, |v| {
        let (Some(t), Some(variants)) = (
            v.get("type").and_then(Value::as_str),
            v.get("enum").and_then(Value::as_array),
        ) else {
            return;
        };
        for variant in variants {
            match t {
                "integer" => assert!(
                    variant.is_i64() || variant.is_u64(),
                    "non-integer variant in integer enum — step 6 missed: {variant:?}"
                ),
                "number" => assert!(
                    variant.is_number(),
                    "non-number variant in number enum — step 6 missed: {variant:?}"
                ),
                "string" => assert!(
                    variant.is_string(),
                    "non-string variant in string enum — step 6 missed: {variant:?}"
                ),
                _ => {}
            }
        }
    });
}

// Step 7: every {placeholder} in a path has a matching `in: path` parameter
// (or is referenced via $ref through components.parameters).
#[test]
fn path_placeholders_have_parameters() {
    let s = spec();
    let shared = s["components"]["parameters"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    let placeholder_re = regex_lite_re_findall;
    let Some(paths) = s["paths"].as_object() else {
        return;
    };
    for (path, item) in paths {
        let placeholders: std::collections::HashSet<String> = placeholder_re(path);
        let Some(item) = item.as_object() else {
            continue;
        };
        for (method, op) in item {
            if !HTTP_METHODS.contains(&method.to_ascii_lowercase().as_str()) {
                continue;
            }
            let mut declared: std::collections::HashSet<String> = Default::default();
            if let Some(params) = op.get("parameters").and_then(Value::as_array) {
                for p in params {
                    if let Some(name) = resolve_path_param_name(p, &shared) {
                        declared.insert(name);
                    }
                }
            }
            let missing: Vec<&String> = placeholders.difference(&declared).collect();
            assert!(
                missing.is_empty(),
                "{} {}: placeholders without params — step 7 missed: {missing:?}",
                method.to_uppercase(),
                path
            );
        }
    }
}

fn resolve_path_param_name(p: &Value, shared: &serde_json::Map<String, Value>) -> Option<String> {
    if let Some(reference) = p.get("$ref").and_then(Value::as_str) {
        let name = reference.rsplit('/').next()?;
        let target = shared.get(name)?;
        if target.get("in").and_then(Value::as_str) == Some("path") {
            return target.get("name").and_then(Value::as_str).map(str::to_owned);
        }
        return None;
    }
    if p.get("in").and_then(Value::as_str) == Some("path") {
        return p.get("name").and_then(Value::as_str).map(str::to_owned);
    }
    None
}

fn regex_lite_re_findall(path: &str) -> std::collections::HashSet<String> {
    // Tiny zero-dep stand-in for `re.findall(r"\{([^}]+)\}", path)`.
    let bytes = path.as_bytes();
    let mut out = std::collections::HashSet::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'}' {
                j += 1;
            }
            if j < bytes.len() {
                if let Ok(s) = std::str::from_utf8(&bytes[i + 1..j]) {
                    out.insert(s.to_owned());
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

// Step 8: no string formats outside the supported set.
#[test]
fn no_unknown_string_formats() {
    const KNOWN: &[&str] = &[
        "byte",
        "binary",
        "date",
        "date-time",
        "email",
        "hostname",
        "ipv4",
        "ipv6",
        "password",
        "uri",
        "uri-reference",
        "uri-template",
        "url",
        "uuid",
    ];
    let s = spec();
    walk_objects(&s, |v| {
        let (Some(t), Some(fmt)) = (
            v.get("type").and_then(Value::as_str),
            v.get("format").and_then(Value::as_str),
        ) else {
            return;
        };
        if t == "string" {
            assert!(
                KNOWN.contains(&fmt),
                "unknown string format still in spec — step 8 missed: {fmt}"
            );
        }
    });
}

// Step 9: no `in: cookie` parameters anywhere.
#[test]
fn no_cookie_parameters() {
    let s = spec();
    walk_objects(&s, |v| {
        let in_value = v.get("in").and_then(Value::as_str);
        assert_ne!(
            in_value,
            Some("cookie"),
            "cookie parameter still in spec — step 9 missed"
        );
    });
}

// Step 10: no operation has more than one content-type per response code.
#[test]
fn at_most_one_content_type_per_response() {
    let s = spec();
    for_each_op(&s, |path, method, op| {
        let Some(responses) = op.get("responses").and_then(Value::as_object) else {
            return;
        };
        for (code, response) in responses {
            if let Some(content) = response.get("content").and_then(Value::as_object) {
                assert!(
                    content.len() <= 1,
                    "{} {} response {}: {} content types — step 10 missed",
                    method.to_uppercase(),
                    path,
                    code,
                    content.len()
                );
            }
        }
    });
}

// Step 11: no operation has only 1xx responses (websocket-upgrade ops dropped).
#[test]
fn no_websocket_upgrade_only_operations() {
    let s = spec();
    for_each_op(&s, |path, method, op| {
        let Some(responses) = op.get("responses").and_then(Value::as_object) else {
            return;
        };
        let codes: Vec<&str> = responses
            .keys()
            .filter(|c| *c != "default")
            .map(String::as_str)
            .collect();
        if codes.is_empty() {
            return;
        }
        let all_1xx = codes.iter().all(|c| c.starts_with('1'));
        assert!(
            !all_1xx,
            "{} {}: 1xx-only operation still present — step 11 missed",
            method.to_uppercase(),
            path
        );
    });
}

// Step 12: query parameters declared as `array` of integer/number have items
// of `string` type (oas3-gen's CSV helper only handles strings).
#[test]
fn array_query_items_are_string_typed() {
    let s = spec();
    for_each_op(&s, |path, method, op| {
        let Some(params) = op.get("parameters").and_then(Value::as_array) else {
            return;
        };
        for p in params {
            if p.get("in").and_then(Value::as_str) != Some("query") {
                continue;
            }
            let Some(schema) = p.get("schema") else { continue };
            if schema.get("type").and_then(Value::as_str) != Some("array") {
                continue;
            }
            if let Some(items) = schema.get("items") {
                let item_type = items.get("type").and_then(Value::as_str);
                assert!(
                    matches!(item_type, Some("string") | None),
                    "{} {} array query param items not string-typed — step 12 missed: {item_type:?}",
                    method.to_uppercase(),
                    path
                );
            }
        }
    });
}

// Step 13: every key listed under `examples.*.value` for an integer-typed
// schema property must itself be an integer (the widening pass should have
// upgraded properties whose examples were floats).
//
// We can't easily resolve the example→property mapping here without
// re-implementing the BFS, so we assert the inverse invariant: no schema
// field is `type: integer` while its examples contain floats. This is a
// structural smoke test, not a full proof.
#[test]
fn integer_fields_not_paired_with_float_examples() {
    let s = spec();
    let schemas = s["components"]["schemas"].as_object().cloned().unwrap_or_default();
    walk_objects(&s, |v| {
        let Some(examples) = v.get("examples").and_then(Value::as_object) else {
            return;
        };
        for ex in examples.values() {
            let Some(value) = ex.get("value") else {
                continue;
            };
            check_example_against_schemas(value, &schemas);
        }
    });
}

fn check_example_against_schemas(value: &Value, schemas: &serde_json::Map<String, Value>) {
    let Some(obj) = value.as_object() else {
        return;
    };
    for (key, val) in obj {
        if val.is_f64() && val.as_f64().is_some_and(|f| f.fract() != 0.0) {
            for schema in schemas.values() {
                let Some(props) = schema.get("properties").and_then(Value::as_object) else {
                    continue;
                };
                let Some(prop) = props.get(key) else { continue };
                let prop_type = prop.get("type").and_then(Value::as_str);
                assert_ne!(
                    prop_type,
                    Some("integer"),
                    "field '{key}' typed `integer` paired with float example {val:?} — step 13 missed"
                );
            }
        }
    }
}

// Sanity: 3.1 spec is also valid + upgraded.
#[test]
fn spec_3_1_is_valid_and_upgraded() {
    let s = spec_3_1();
    let openapi = s["openapi"].as_str().unwrap_or("");
    assert!(
        openapi.starts_with("3.1"),
        "3.1 spec doesn't claim 3.1: {openapi}"
    );
    assert!(s["paths"].as_object().is_some_and(|p| !p.is_empty()));
}
