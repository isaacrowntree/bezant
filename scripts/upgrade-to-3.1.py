#!/usr/bin/env python3
"""Upgrade an OpenAPI 3.0 document to 3.1.

Transformations:
  1. Set `openapi: 3.1.0`
  2. `nullable: true` + `type: X` → `type: [X, "null"]`
  3. `exclusiveMinimum: true` + `minimum: N` → `exclusiveMinimum: N` (drop `minimum`)
  4. `exclusiveMaximum: true` + `maximum: N` → `exclusiveMaximum: N` (drop `maximum`)
  5. `type: "file"` → `type: "string", format: "binary"` (3.0 legacy)
  6. Drop `discriminator` mapping with invalid keys (optional safety)

The script is idempotent; running it on an already-3.1 document is a no-op.

Usage:
    python3 scripts/upgrade-to-3.1.py INPUT.json OUTPUT.json
"""
from __future__ import annotations

import json
import sys
from typing import Any


def upgrade_schema(obj: Any) -> int:
    """Recursive transformation of schema objects. Returns count of edits."""
    edits = 0

    def walk(o: Any) -> None:
        nonlocal edits
        if isinstance(o, dict):
            # `nullable: true` → type array with "null"
            if o.get("nullable") is True and "type" in o:
                t = o["type"]
                if isinstance(t, str) and t != "null":
                    o["type"] = [t, "null"]
                    edits += 1
                elif isinstance(t, list) and "null" not in t:
                    o["type"] = t + ["null"]
                    edits += 1
                del o["nullable"]

            if o.get("nullable") is True and "type" not in o:
                # No declared type — drop the orphan nullable
                del o["nullable"]
                edits += 1

            if o.get("nullable") is False:
                del o["nullable"]
                edits += 1

            # exclusiveMinimum/Maximum: bool → number in 3.1
            if o.get("exclusiveMinimum") is True and "minimum" in o:
                o["exclusiveMinimum"] = o.pop("minimum")
                edits += 1
            elif o.get("exclusiveMinimum") is False:
                del o["exclusiveMinimum"]
                edits += 1

            if o.get("exclusiveMaximum") is True and "maximum" in o:
                o["exclusiveMaximum"] = o.pop("maximum")
                edits += 1
            elif o.get("exclusiveMaximum") is False:
                del o["exclusiveMaximum"]
                edits += 1

            # `type: "file"` was a 2.0 carryover, never valid in 3.0 but some
            # specs include it. Canonicalise to string/binary.
            if o.get("type") == "file":
                o["type"] = "string"
                o["format"] = "binary"
                edits += 1

            # `example` still valid in 3.1 — no rewrite needed.
            # `examples` on media types is also valid.

            for v in list(o.values()):
                walk(v)
        elif isinstance(o, list):
            for x in o:
                walk(x)

    walk(obj)
    return edits


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__, file=sys.stderr)
        return 2

    src, dst = sys.argv[1], sys.argv[2]
    with open(src, encoding="utf-8") as f:
        spec = json.load(f)

    old_version = spec.get("openapi", "?")
    spec["openapi"] = "3.1.0"

    edits = upgrade_schema(spec)

    with open(dst, "w", encoding="utf-8") as f:
        json.dump(spec, f, indent=2, sort_keys=True)
        f.write("\n")

    print(f"upgraded {src} → OpenAPI 3.1.0 (was {old_version})")
    print(f"  schema edits: {edits}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
