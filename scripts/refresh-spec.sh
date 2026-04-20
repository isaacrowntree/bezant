#!/usr/bin/env bash
# Refresh the vendored IBKR OpenAPI spec from upstream.
#
# Run from the workspace root:
#   ./scripts/refresh-spec.sh
#
# After running:
#   - review the git diff on crates/bezant-spec/ibkr-openapi.json
#   - review the UPSTREAM_VERSION bump in crates/bezant-spec/src/lib.rs
#   - rebuild the workspace and address any codegen changes
#   - commit

set -euo pipefail

UPSTREAM_URL="${UPSTREAM_URL:-https://api.ibkr.com/gw/api/v3/api-docs}"
SPEC_PATH="crates/bezant-spec/ibkr-openapi.json"
LIB_PATH="crates/bezant-spec/src/lib.rs"

if [[ ! -f Cargo.toml ]]; then
  echo "error: run from the workspace root (where Cargo.toml lives)" >&2
  exit 1
fi

echo "→ downloading spec from ${UPSTREAM_URL}"
tmp=$(mktemp)
curl -fsSL "${UPSTREAM_URL}" -o "${tmp}"

echo "→ validating downloaded JSON"
jq -e '.openapi and .paths and .info.version' "${tmp}" >/dev/null

new_version=$(jq -r '.info.version' "${tmp}")
echo "→ upstream spec version: ${new_version}"

# Upstream quirks we normalise before vendoring. See scripts/normalize-spec.py
# for the full list of transformations.
echo "→ normalising upstream quirks"
python3 scripts/normalize-spec.py "${tmp}" "${SPEC_PATH}"

if grep -q 'UPSTREAM_VERSION' "${LIB_PATH}"; then
  sed -i.bak -E "s|pub const UPSTREAM_VERSION: &str = \"[^\"]*\";|pub const UPSTREAM_VERSION: \&str = \"${new_version}\";|" "${LIB_PATH}"
  rm -f "${LIB_PATH}.bak"
  echo "→ bumped UPSTREAM_VERSION to ${new_version}"
else
  echo "warning: UPSTREAM_VERSION constant not found in ${LIB_PATH}" >&2
fi

rm -f "${tmp}"
echo "→ done. review 'git diff ${SPEC_PATH}' and rebuild."
