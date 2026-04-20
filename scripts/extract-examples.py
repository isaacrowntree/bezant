#!/usr/bin/env python3
"""Extract real IBKR response examples from the vendored spec.

Output is keyed by operation ID (one entry per success example), which
makes it trivial to match against Rust response types that oas3-gen
already named after the operation ID.

Usage:
    python3 scripts/extract-examples.py [--only op1,op2,...] [-o OUTPUT]
"""
from __future__ import annotations

import argparse
import json
from typing import Any


def collect(spec: dict[str, Any], only: set[str] | None) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    for path, item in spec.get("paths", {}).items():
        if not isinstance(item, dict):
            continue
        for method, op in item.items():
            if not isinstance(op, dict):
                continue
            op_id = op.get("operationId")
            if not op_id:
                continue
            if only and op_id not in only:
                continue
            for code, r in (op.get("responses") or {}).items():
                if not isinstance(r, dict):
                    continue
                if not code.startswith("2"):
                    continue
                for ct, body in (r.get("content") or {}).items():
                    if not isinstance(body, dict):
                        continue
                    schema = body.get("schema") or {}
                    schema_ref = schema.get("$ref", "")
                    for ex_name, ex in (body.get("examples") or {}).items():
                        if isinstance(ex, dict) and "value" in ex:
                            out.append(
                                {
                                    "operationId": op_id,
                                    "method": method.upper(),
                                    "path": path,
                                    "status": code,
                                    "contentType": ct,
                                    "schemaRef": schema_ref,
                                    "exampleName": ex_name,
                                    "value": ex["value"],
                                }
                            )
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--spec",
        default="crates/bezant-spec/ibkr-openapi-3.1.json",
    )
    parser.add_argument(
        "-o",
        "--output",
        default="crates/bezant-core/tests/fixtures/examples.json",
    )
    parser.add_argument(
        "--only",
        help="Comma-separated list of operationIds to include (default: all)",
    )
    args = parser.parse_args()

    with open(args.spec, encoding="utf-8") as f:
        spec = json.load(f)

    only = set(args.only.split(",")) if args.only else None
    records = collect(spec, only)

    import os

    os.makedirs(os.path.dirname(args.output), exist_ok=True)
    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(records, f, indent=2, sort_keys=True)
        f.write("\n")

    by_op: dict[str, int] = {}
    for r in records:
        by_op[r["operationId"]] = by_op.get(r["operationId"], 0) + 1
    print(f"extracted {len(records)} examples across {len(by_op)} operations → {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
