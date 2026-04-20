#!/usr/bin/env python3
"""Normalize the IBKR OpenAPI spec for code generation.

Transformations applied:
  1. Strip null entries from security scope arrays (IBKR emits `[null]`
     where OpenAPI 3.0 requires `[]` or `[scope-string]`).
  2. Synthesise an `operationId` for every operation that lacks one.
     Codegen tools like progenitor require every operation to have a stable
     operationId to name the generated method.
  3. Disambiguate duplicate `operationId`s by appending a path-derived suffix
     to later occurrences. Progenitor rejects duplicates; OpenAPI forbids
     them in spirit but IBKR ships a few.
  4. Sanitise enum variants that contain symbols typify can't turn into
     unique Rust identifiers (e.g. `>=`, `<=`, `>`, `<`, `==` all collapse
     after identifier normalisation). We downgrade such enums to plain
     strings — the schema still documents the valid values via `description`,
     but at the type level it's `String`.
  5. Rewrite non-standard content types progenitor doesn't understand
     (e.g. `application/jwt`) to `text/plain`. The body is still a string
     on the wire; progenitor just refuses to emit a method otherwise.
  6. Reconcile enum values with their declared `type`. IBKR sometimes lists
     string enum values under `type: "number"` (e.g. `bankAccountTypeCode`
     declares numbers but enumerates `"0"`, `"1"`, `"2"`). Typify rejects
     that. We coerce enum values to match the declared type where feasible;
     otherwise we coerce the `type` to match the majority-variant type.
  7. Reassign path parameters whose placeholder is missing from the path.
     IBKR declares several `in: path` params (e.g. `client-id`) but never
     includes `{client-id}` in the URL template. Progenitor panics. We
     rewrite these to `in: query`, which is the pragmatic intent.
  8. Drop non-standard `format` values (e.g. `"jwt"`) from string schemas.
     Progenitor accepts only a small set of string formats for request
     bodies; unknown ones cause codegen to fail.
  9. Demote `in: cookie` parameters to headers. Progenitor doesn't support
     cookie parameters.
 10. Collapse multi-content-type success responses to a single content type.
     Progenitor asserts `response_types.len() <= 1`. When IBKR offers a 2xx
     response in both `application/json` and `application/pdf`, we prefer
     JSON (the typed response) and drop the binary alternatives.
 11. Drop websocket-upgrade operations (those with only `1xx` responses, e.g.
     `101 Switching Protocols`). Progenitor has no way to model them and
     REST clients shouldn't call them anyway.
 12. Coerce numeric-array query parameters to string-array items.
     oas3-gen's `StringWithCommaSeparator` wrapper only handles strings on the
     wire; an array of `integer` items fails compilation. The serialized form
     is identical either way, so stringifying the item type is harmless.

Usage:
    python3 scripts/normalize-spec.py INPUT.json OUTPUT.json

Idempotent: running twice produces the same output as running once.
"""
from __future__ import annotations

import json
import re
import sys
from typing import Any

HTTP_METHODS = {"get", "post", "put", "delete", "patch", "options", "head", "trace"}


def strip_null_security_scopes(spec: dict[str, Any]) -> int:
    """Remove null values from every `security[].scheme[]` list. Returns count."""
    removed = 0

    def clean(req: dict[str, Any]) -> dict[str, Any]:
        nonlocal removed
        out = {}
        for scheme, scopes in req.items():
            if isinstance(scopes, list):
                filtered = [s for s in scopes if s is not None]
                removed += len(scopes) - len(filtered)
                out[scheme] = filtered
            else:
                out[scheme] = scopes
        return out

    def walk(obj: Any) -> None:
        if isinstance(obj, dict):
            if isinstance(obj.get("security"), list):
                obj["security"] = [
                    clean(req) if isinstance(req, dict) else req for req in obj["security"]
                ]
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return removed


def operation_id_from(method: str, path: str) -> str:
    """Synthesise a stable operationId from HTTP method + path.

    Converts `/gw/api/v1/accounts/{accountId}/status` + `GET` into
    `get_gw_api_v1_accounts_accountid_status`.
    """
    cleaned = re.sub(r"[{}]", "", path).strip("/")
    cleaned = re.sub(r"[^A-Za-z0-9]+", "_", cleaned)
    cleaned = cleaned.strip("_").lower()
    return f"{method.lower()}_{cleaned}" if cleaned else method.lower()


def synthesise_operation_ids(spec: dict[str, Any]) -> int:
    """Give every operation missing an operationId a synthetic one. Returns count added."""
    added = 0
    for path, path_item in spec.get("paths", {}).items():
        if not isinstance(path_item, dict):
            continue
        for method, op in list(path_item.items()):
            if method.lower() not in HTTP_METHODS:
                continue
            if not isinstance(op, dict):
                continue
            if "operationId" not in op or not op["operationId"]:
                op["operationId"] = operation_id_from(method, path)
                added += 1
    return added


def sanitize_ambiguous_enums(spec: dict[str, Any]) -> int:
    """Replace enums whose variants collide when turned into Rust identifiers.

    Typify generates enum variants by stripping non-alphanumeric chars, which
    collapses `>=`, `<=`, `>`, `<`, `==` etc to empty or near-identical names.
    We demote such enums to a plain `type: "string"` with the original variants
    listed in the `description` for reference.
    """

    def ident_for(v: str) -> str:
        return re.sub(r"[^A-Za-z0-9]+", "", v)

    affected = 0

    def walk(obj: Any) -> None:
        nonlocal affected
        if isinstance(obj, dict):
            if obj.get("type") == "string" and isinstance(obj.get("enum"), list):
                variants = [v for v in obj["enum"] if isinstance(v, str)]
                idents = [ident_for(v) for v in variants]
                has_collision = len(idents) != len(set(idents))
                all_empty = all(not i for i in idents)
                if has_collision or all_empty:
                    preserved = ", ".join(variants)
                    old_desc = obj.get("description") or ""
                    sep = " " if old_desc else ""
                    obj["description"] = (
                        f"{old_desc}{sep}(one of: {preserved})".strip()
                    )
                    del obj["enum"]
                    affected += 1
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return affected


# Formats progenitor/typify recognise on string-typed schemas. Anything
# outside this set is dropped from string schemas during normalisation.
_STRING_FORMATS = {
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
}


_CONTENT_TYPE_PREFERENCE = [
    "application/json",
    "application/json; charset=utf-8",
    "application/problem+json",
    "application/x-www-form-urlencoded",
    "text/plain",
    "*/*",
    "application/pdf",
    "application/octet-stream",
]


def collapse_multi_content_responses(spec: dict[str, Any]) -> int:
    """Simplify response types to a single typed success variant.

    Progenitor asserts `response_types.len() <= 1` after collecting the
    response schemas from every declared response (2xx, 4xx, 5xx). To satisfy
    the assertion without losing HTTP error handling entirely, we:
      - pick a single preferred content type per response when multiple are listed
      - strip the `content` from every non-2xx response so their schema is
        transparent to progenitor (errors still surface as
        `Error::ErrorResponse { status, .. }` at the client layer).
    """
    collapsed = 0

    def pick_one_ct(r: Any) -> None:
        nonlocal collapsed
        if not isinstance(r, dict):
            return
        content = r.get("content")
        if not isinstance(content, dict) or len(content) <= 1:
            return
        chosen = next(
            (ct for ct in _CONTENT_TYPE_PREFERENCE if ct in content),
            next(iter(content)),
        )
        r["content"] = {chosen: content[chosen]}
        collapsed += 1

    def strip_error_bodies(responses: dict[str, Any]) -> None:
        nonlocal collapsed
        # Progenitor asserts every response in an operation resolves to the
        # same Rust type. We keep exactly one 2xx response (the primary
        # success case) with its content, and remove every other 2xx entirely
        # so progenitor doesn't see a secondary unit-typed response.
        # Non-2xx responses are stripped of content but kept as metadata.
        success_codes = sorted(c for c in responses if c.startswith("2"))
        primary = success_codes[0] if success_codes else None
        for code, r in list(responses.items()):
            if code == primary or code == "default":
                pick_one_ct(r)
            elif code.startswith("2"):
                # Secondary success code — delete outright so progenitor
                # doesn't fold its unit type into response_types alongside
                # the primary's real type.
                del responses[code]
                collapsed += 1
            else:
                if isinstance(r, dict) and "content" in r:
                    del r["content"]
                    collapsed += 1

    for path_item in spec.get("paths", {}).values():
        if not isinstance(path_item, dict):
            continue
        for method, op in path_item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(op, dict):
                continue
            resps = op.get("responses")
            if isinstance(resps, dict):
                strip_error_bodies(resps)

    # Shared component responses are almost always used as error bodies
    # (BadRequest / Unauthorized / InternalError). We strip their content too
    # so progenitor doesn't see a different response schema per status code.
    for r in (spec.get("components", {}).get("responses", {}) or {}).values():
        if isinstance(r, dict) and "content" in r:
            del r["content"]
            collapsed += 1

    return collapsed


def stringify_numeric_array_query_params(spec: dict[str, Any]) -> int:
    """Coerce `array of integer` query params to `array of string`.

    oas3-gen only has a `StringWithCommaSeparator` helper for form-style
    comma-separated arrays; it chokes on numeric items. The coercion is
    lossless on the wire (CSV either way) and the downstream user just
    passes strings.
    """
    changed = 0
    for path_item in spec.get("paths", {}).values():
        if not isinstance(path_item, dict):
            continue
        for method, op in path_item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(op, dict):
                continue
            for p in op.get("parameters", []) or []:
                if not isinstance(p, dict):
                    continue
                if p.get("in") != "query":
                    continue
                schema = p.get("schema") or {}
                if schema.get("type") == "array":
                    items = schema.get("items") or {}
                    if items.get("type") in ("integer", "number"):
                        items["type"] = "string"
                        items.pop("format", None)
                        changed += 1
    return changed


def drop_websocket_ops(spec: dict[str, Any]) -> int:
    """Remove operations whose responses are 1xx only (WebSocket upgrade etc.)."""
    dropped = 0
    for path, item in list(spec.get("paths", {}).items()):
        if not isinstance(item, dict):
            continue
        for method in list(item.keys()):
            if method.lower() not in HTTP_METHODS:
                continue
            op = item[method]
            if not isinstance(op, dict):
                continue
            resps = op.get("responses") or {}
            codes = [c for c in resps.keys() if c != "default"]
            if codes and all(c.startswith("1") for c in codes):
                del item[method]
                dropped += 1
        # If the path item has no operations left, drop it too
        if not any(k.lower() in HTTP_METHODS for k in item.keys()):
            del spec["paths"][path]
    return dropped


def demote_cookie_params(spec: dict[str, Any]) -> int:
    """Rewrite `in: cookie` parameters to `in: header`. Progenitor doesn't
    support cookie parameters; this is the pragmatic fallback."""
    changed = 0

    def walk(obj: Any) -> None:
        nonlocal changed
        if isinstance(obj, dict):
            if obj.get("in") == "cookie":
                obj["in"] = "header"
                obj.pop("style", None)
                obj.pop("explode", None)
                changed += 1
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return changed


def drop_unknown_string_formats(spec: dict[str, Any]) -> int:
    """Remove non-standard `format` fields from string schemas."""
    removed = 0

    def walk(obj: Any) -> None:
        nonlocal removed
        if isinstance(obj, dict):
            if obj.get("type") == "string" and isinstance(obj.get("format"), str):
                if obj["format"] not in _STRING_FORMATS:
                    del obj["format"]
                    removed += 1
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return removed


def fix_misplaced_path_params(spec: dict[str, Any]) -> int:
    """Rewrite `in: path` params not referenced in their URL template to `in: query`.

    Handles both inline parameter definitions on operations and shared refs
    in `components.parameters`. Because the shared refs are used across
    multiple paths, we only demote a shared ref if it's never legitimately
    used as a path parameter anywhere.
    """
    # Index all paths that legitimately include each {placeholder}
    used_as_path: dict[str, bool] = {}

    # Helper: placeholders present in a URL template
    def path_placeholders(url: str) -> set[str]:
        return set(re.findall(r"\{([^}]+)\}", url))

    shared_params = spec.get("components", {}).get("parameters", {}) or {}

    # Pass 1: for every operation, scan parameters (both inline and refs) and
    # decide whether each path-declared one can resolve.
    fixed = 0

    for path, item in spec.get("paths", {}).items():
        if not isinstance(item, dict):
            continue
        placeholders = path_placeholders(path)
        placeholder_ci: dict[str, str] = {ph.lower(): ph for ph in placeholders}
        for method, op in item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(op, dict):
                continue
            params = op.get("parameters", [])
            for idx, p in enumerate(params):
                if not isinstance(p, dict):
                    continue
                ref = p.get("$ref")
                target: dict[str, Any]
                if ref and ref.startswith("#/components/parameters/"):
                    shared_name = ref.rsplit("/", 1)[-1]
                    target = shared_params.get(shared_name, {})
                    if target.get("in") == "path":
                        name = target.get("name")
                        if name and name not in placeholders:
                            # Case-only mismatch (e.g. {notificationID} vs
                            # notificationId)? Inline a corrected copy that
                            # matches the placeholder exactly.
                            if name and name.lower() in placeholder_ci:
                                corrected = dict(target)
                                corrected["name"] = placeholder_ci[name.lower()]
                                params[idx] = corrected
                                fixed += 1
                                continue
                            # True mismatch: demote to query.
                            corrected = dict(target)
                            corrected["in"] = "query"
                            corrected.pop("required", None)
                            corrected.pop("style", None)
                            corrected.pop("explode", None)
                            params[idx] = corrected
                            fixed += 1
                else:
                    if p.get("in") == "path":
                        name = p.get("name")
                        if name and name not in placeholders:
                            if name and name.lower() in placeholder_ci:
                                p["name"] = placeholder_ci[name.lower()]
                                fixed += 1
                                continue
                            p["in"] = "query"
                            p.pop("required", None)
                            p.pop("style", None)
                            p.pop("explode", None)
                            fixed += 1
            # Second pass: any placeholders still lacking a declared param?
            # Synthesise a string path parameter for them.
            declared = set()
            for p in params:
                if isinstance(p, dict):
                    if p.get("in") == "path":
                        declared.add(p.get("name"))
                    ref = p.get("$ref")
                    if ref and ref.startswith("#/components/parameters/"):
                        tgt = shared_params.get(ref.rsplit("/", 1)[-1], {})
                        if tgt.get("in") == "path":
                            declared.add(tgt.get("name"))
            for ph in placeholders - declared:
                params.append(
                    {
                        "name": ph,
                        "in": "path",
                        "required": True,
                        "schema": {"type": "string"},
                    }
                )
                fixed += 1
            op["parameters"] = params
    return fixed


_SUPPORTED_CONTENT_TYPES = {
    "application/json",
    "application/json; charset=utf-8",
    "application/problem+json",
    "application/x-www-form-urlencoded",
    "application/pdf",
    "application/octet-stream",
    "text/plain",
    "*/*",
}


def reconcile_enum_types(spec: dict[str, Any]) -> int:
    """Ensure every `enum` variant is compatible with the schema's `type`.

    Strategy: if `type` is `number`/`integer`, coerce variants to numeric.
    If `type` is `string`, coerce variants to their string repr. If coercion
    fails (e.g. a truly unparseable value in a numeric enum), drop that variant.
    """
    reconciled = 0

    def walk(obj: Any) -> None:
        nonlocal reconciled
        if isinstance(obj, dict):
            declared = obj.get("type")
            variants = obj.get("enum")
            if declared in ("number", "integer", "string") and isinstance(variants, list):
                new_variants: list[Any] = []
                changed = False
                for v in variants:
                    if declared == "integer":
                        coerced: Any
                        if isinstance(v, bool):
                            coerced = None
                        elif isinstance(v, int):
                            coerced = v
                        else:
                            try:
                                coerced = int(v)
                            except (TypeError, ValueError):
                                coerced = None
                        if coerced is not None and not isinstance(v, int):
                            changed = True
                        if coerced is None:
                            changed = True
                            continue
                        new_variants.append(coerced)
                    elif declared == "number":
                        if isinstance(v, bool):
                            changed = True
                            continue
                        if isinstance(v, (int, float)):
                            new_variants.append(v)
                        else:
                            try:
                                new_variants.append(float(v))
                                changed = True
                            except (TypeError, ValueError):
                                changed = True
                                continue
                    else:  # string
                        if isinstance(v, str):
                            new_variants.append(v)
                        elif v is None:
                            changed = True
                            continue
                        else:
                            new_variants.append(str(v))
                            changed = True
                if changed:
                    obj["enum"] = new_variants
                    reconciled += 1
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return reconciled


def rewrite_exotic_content_types(spec: dict[str, Any]) -> int:
    """Replace content types progenitor can't emit with `text/plain`.

    When we reassign a body to `text/plain`, we also replace the schema with
    a plain `{ "type": "string" }` — progenitor requires text bodies to be
    string-typed, not object/ref, and the original body was a string on the
    wire anyway (just wrapped in a JWT/encoded container).
    """
    rewritten = 0

    def walk(obj: Any) -> None:
        nonlocal rewritten
        if isinstance(obj, dict):
            content = obj.get("content")
            if isinstance(content, dict):
                for ct in list(content.keys()):
                    if ct not in _SUPPORTED_CONTENT_TYPES:
                        body = content.pop(ct)
                        if isinstance(body, dict):
                            body = dict(body)
                            body["schema"] = {"type": "string"}
                        content.setdefault("text/plain", body)
                        rewritten += 1
            for v in obj.values():
                walk(v)
        elif isinstance(obj, list):
            for x in obj:
                walk(x)

    walk(spec)
    return rewritten


def disambiguate_operation_ids(spec: dict[str, Any]) -> int:
    """Rename duplicate operationIds so each is unique. Returns number renamed.

    Strategy: first occurrence keeps the original id; subsequent occurrences
    get a path-derived suffix appended. Deterministic as long as paths are
    iterated in sorted order.
    """
    seen: dict[str, str] = {}  # operationId -> "METHOD PATH" of first owner
    renamed = 0

    for path in sorted(spec.get("paths", {}).keys()):
        path_item = spec["paths"][path]
        if not isinstance(path_item, dict):
            continue
        for method in sorted(path_item.keys()):
            if method.lower() not in HTTP_METHODS:
                continue
            op = path_item[method]
            if not isinstance(op, dict):
                continue
            opid = op.get("operationId")
            if not opid:
                continue
            if opid in seen:
                suffix = operation_id_from(method, path)
                new_id = f"{opid}_{suffix}"
                # Ensure the new id is itself unique
                base = new_id
                counter = 2
                while new_id in seen:
                    new_id = f"{base}_{counter}"
                    counter += 1
                op["operationId"] = new_id
                seen[new_id] = f"{method.upper()} {path}"
                renamed += 1
            else:
                seen[opid] = f"{method.upper()} {path}"
    return renamed


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__, file=sys.stderr)
        return 2
    src, dst = sys.argv[1], sys.argv[2]
    with open(src, encoding="utf-8") as f:
        spec = json.load(f)

    nulls = strip_null_security_scopes(spec)
    ids = synthesise_operation_ids(spec)
    dedup = disambiguate_operation_ids(spec)
    enums = sanitize_ambiguous_enums(spec)
    cts = rewrite_exotic_content_types(spec)
    reconciled = reconcile_enum_types(spec)
    params = fix_misplaced_path_params(spec)
    fmts = drop_unknown_string_formats(spec)
    cookies = demote_cookie_params(spec)
    ws = drop_websocket_ops(spec)
    arrs = stringify_numeric_array_query_params(spec)
    multi = collapse_multi_content_responses(spec)

    with open(dst, "w", encoding="utf-8") as f:
        json.dump(spec, f, indent=2, sort_keys=True)
        f.write("\n")

    print(
        f"normalized: stripped {nulls} null scopes, "
        f"added {ids} operationIds, renamed {dedup} duplicates, "
        f"desugared {enums} ambiguous enums, "
        f"rewrote {cts} exotic content types, "
        f"reconciled {reconciled} enum/type mismatches, "
        f"demoted {params} misplaced path params, "
        f"dropped {fmts} unknown string formats, "
        f"rewrote {cookies} cookie params to headers, "
        f"dropped {ws} websocket upgrade ops, "
        f"stringified {arrs} numeric-array queries, "
        f"collapsed {multi} multi-content responses"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
