//! Vendored copy of the IBKR Client Portal Web API OpenAPI specification.
//!
//! Interactive Brokers publishes the spec at
//! <https://api.ibkr.com/gw/api/v3/api-docs> as OpenAPI **3.0**. Bezant
//! vendors a pinned copy so downstream crates always build from a known
//! version. For Rust codegen via the [`bezant-api`] crate we upgrade
//! the spec to **3.1** (see [`SPEC_JSON_3_1`]) — the 3.0 source is
//! preserved via [`SPEC_JSON`] so consumers running their own generator
//! can pick whichever major they need.
//!
//! Refresh the vendored copy with `scripts/refresh-spec.sh`, then run
//! `scripts/codegen.sh` to regenerate the 3.1 derivative and the Rust client.
//!
//! [`bezant-api`]: https://docs.rs/bezant-api

/// The pinned IBKR OpenAPI 3.0 spec as a UTF-8 string — verbatim from
/// upstream.
///
/// Use [`SPEC_JSON`] if you need the original 3.0 wording; use
/// [`SPEC_JSON_3_1`] if you want the normalised-and-upgraded variant
/// `bezant-api` itself is generated from.
pub const SPEC_JSON: &str = include_str!("../ibkr-openapi.json");

/// The normalised + 3.1-upgraded spec as a UTF-8 string. Produced by
/// `scripts/codegen.sh` and checked in alongside the 3.0 source so
/// consumers don't need Python to reproduce our generator input.
pub const SPEC_JSON_3_1: &str = include_str!("../ibkr-openapi-3.1.json");

/// IBKR-reported spec version embedded in the vendored spec's `info.version`
/// field. Updated by `scripts/refresh-spec.sh`.
pub const UPSTREAM_VERSION: &str = "2.29.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_is_valid_json() {
        let v: serde_json::Value =
            serde_json::from_str(SPEC_JSON).expect("spec must be valid JSON");
        assert_eq!(v["openapi"].as_str().unwrap_or(""), "3.0.0");
        assert!(!v["paths"].as_object().unwrap().is_empty());
    }

    #[test]
    fn upstream_version_matches_embedded() {
        let v: serde_json::Value = serde_json::from_str(SPEC_JSON).unwrap();
        let embedded = v["info"]["version"].as_str().unwrap_or("");
        assert_eq!(
            embedded, UPSTREAM_VERSION,
            "UPSTREAM_VERSION constant out of sync with vendored spec"
        );
    }
}
