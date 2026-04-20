//! Vendored copy of the IBKR Client Portal Web API OpenAPI 3.0 specification.
//!
//! The spec is published by Interactive Brokers at
//! <https://api.ibkr.com/gw/api/v3/api-docs> and updated with each CPAPI
//! release. Bezant vendors a pinned copy so our downstream crates
//! (`bezant-api` in particular) always build from a known spec.
//!
//! Refresh the vendored copy with `scripts/refresh-spec.sh` — it re-downloads
//! the upstream spec and diffs against the version checked in.

/// The pinned IBKR OpenAPI 3.0 spec as a UTF-8 string.
///
/// Use [`SPEC_JSON`] directly if you want to drive your own code generator
/// or inspect the spec programmatically.
pub const SPEC_JSON: &str = include_str!("../ibkr-openapi.json");

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
