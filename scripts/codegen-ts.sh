#!/usr/bin/env bash
# Regenerate the TypeScript client from the vendored spec.
#
# Depends on:
#   - Java 17+ (for openapi-generator-cli)
#   - Node + npx (to run @openapitools/openapi-generator-cli)
#
# Run from the workspace root.

set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
  echo "error: run from the workspace root (where Cargo.toml lives)" >&2
  exit 1
fi

SPEC_3_1="crates/bezant-spec/ibkr-openapi-3.1.json"
OUT_DIR="clients/typescript"

if [[ ! -f "${SPEC_3_1}" ]]; then
  echo "error: ${SPEC_3_1} is missing. Run ./scripts/codegen.sh first." >&2
  exit 1
fi

# Preserve files we hand-curate (README, package.json) across regenerations.
tmp_preserve=$(mktemp -d)
cp "${OUT_DIR}/README.md" "${tmp_preserve}/README.md" 2>/dev/null || true
cp "${OUT_DIR}/package.json" "${tmp_preserve}/package.json" 2>/dev/null || true

echo "→ running openapi-generator-cli (typescript-fetch)"
npx --yes @openapitools/openapi-generator-cli generate \
  --input-spec "${SPEC_3_1}" \
  --generator-name typescript-fetch \
  --output "${OUT_DIR}" \
  --skip-validate-spec \
  --additional-properties=npmName=bezant-client,supportsES6=true,withInterfaces=true,modelPropertyNaming=original \
  >/dev/null

# Restore curated files
[[ -f "${tmp_preserve}/README.md" ]] && cp "${tmp_preserve}/README.md" "${OUT_DIR}/README.md"
[[ -f "${tmp_preserve}/package.json" ]] && cp "${tmp_preserve}/package.json" "${OUT_DIR}/package.json"
rm -rf "${tmp_preserve}"

echo "→ patching known generator bugs"
# openapi-generator-cli sometimes emits `export type X = ;` for oneOf
# schemas whose alternatives collapsed during normalisation. Rewrite those
# to `unknown` so TypeScript 6 accepts them.
find "${OUT_DIR}" -name "*.ts" -print0 \
  | xargs -0 perl -i -pe 's/^export type (\w+) = ;/export type $1 = unknown;/'

# `ClientPublicKeySetKeysInner` in particular has a JWK-shaped oneOf whose
# upstream stub indexes `value['kty']` on the collapsed type. Replace the
# whole file with a permissive passthrough stub — callers that need the
# real JWK structure can cast downstream.
cat > "${OUT_DIR}/src/models/ClientPublicKeySetKeysInner.ts" <<'STUB'
/* tslint:disable */
/* eslint-disable */
// NOTE: Manual stub. See scripts/codegen-ts.sh for why.
export type ClientPublicKeySetKeysInner = any;
export function ClientPublicKeySetKeysInnerFromJSON(json: any): any { return json; }
export function ClientPublicKeySetKeysInnerFromJSONTyped(json: any, _ignore: boolean): any { return json; }
export function ClientPublicKeySetKeysInnerToJSON(value: any): any { return value; }
export function ClientPublicKeySetKeysInnerToJSONTyped(value?: any | null, _ignore: boolean = false): any { return value ?? undefined; }
STUB

echo "→ done. Run 'cd ${OUT_DIR} && npm install && npm run build' to verify."
