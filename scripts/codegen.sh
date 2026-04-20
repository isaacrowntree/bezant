#!/usr/bin/env bash
# Regenerate crates/bezant-api/src/generated from the vendored spec.
#
# Pipeline:
#   1. normalize-spec.py  — work around upstream spec quirks (missing
#                           operationIds, security[null], etc.)
#   2. upgrade-to-3.1.py  — convert the 3.0 spec to 3.1 for oas3-gen
#   3. oas3-gen           — emit crates/bezant-api/src/generated/
#
# Prerequisites:
#   - python3 on PATH
#   - oas3-gen on PATH (cargo install oas3-gen)
#
# Run from the workspace root.

set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
  echo "error: run from the workspace root (where Cargo.toml lives)" >&2
  exit 1
fi

SPEC_VENDORED="crates/bezant-spec/ibkr-openapi.json"
SPEC_3_1="crates/bezant-spec/ibkr-openapi-3.1.json"
GEN_DIR="crates/bezant-api/src/generated"

command -v python3 >/dev/null || { echo "error: python3 required" >&2; exit 1; }
command -v oas3-gen >/dev/null || {
  echo "error: oas3-gen required. Install: cargo install oas3-gen" >&2
  exit 1
}

echo "→ normalising vendored spec"
python3 scripts/normalize-spec.py "${SPEC_VENDORED}" "${SPEC_VENDORED}"

echo "→ upgrading 3.0 → 3.1"
python3 scripts/upgrade-to-3.1.py "${SPEC_VENDORED}" "${SPEC_3_1}"

echo "→ clearing previous generated output"
rm -rf "${GEN_DIR}"

echo "→ running oas3-gen client-mod"
oas3-gen generate \
  --input "${SPEC_3_1}" \
  --output "${GEN_DIR}" \
  client-mod

echo "→ done. Run 'cargo build -p bezant-api' to verify."
