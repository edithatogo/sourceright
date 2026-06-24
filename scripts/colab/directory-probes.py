#!/usr/bin/env python3
"""MCP directory URL probes for Colab or any Linux Python runtime."""

from __future__ import annotations

import json
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone

PROBES = [
    ("smithery_listing", "https://smithery.ai/servers/edithatogo/sourceright"),
    ("smithery_search", "https://smithery.ai/search?q=sourceright"),
    ("glama_listing", "https://glama.ai/mcp/servers/c7qsbvekc1"),
    ("glama_api", "https://glama.ai/api/mcp/v1/servers/c7qsbvekc1"),
    ("glama_slug_listing", "https://glama.ai/mcp/servers/edithatogo/sourceright"),
    ("glama_search", "https://glama.ai/mcp/servers?q=sourceright"),
    (
        "mcp_registry",
        "https://registry.modelcontextprotocol.io/v0.1/servers/edithatogo/sourceright",
    ),
]


def probe(label: str, url: str) -> dict[str, object]:
    request = urllib.request.Request(url, headers={"User-Agent": "sourceright-probe/1.0"})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            body = response.read(240)
            return {
                "label": label,
                "url": url,
                "http_code": response.status,
                "body_snippet": body.decode("utf-8", errors="replace"),
            }
    except urllib.error.HTTPError as exc:
        body = exc.read(240) if exc.fp else b""
        return {
            "label": label,
            "url": url,
            "http_code": exc.code,
            "body_snippet": body.decode("utf-8", errors="replace"),
        }
    except Exception as exc:  # noqa: BLE001 - probe should continue
        return {
            "label": label,
            "url": url,
            "http_code": 0,
            "error": str(exc),
        }


def main() -> int:
    stamp = datetime.now(timezone.utc).isoformat()
    results = {
        "stamp": stamp,
        "runner": "colab-or-linux-python",
        "probes": [probe(label, url) for label, url in PROBES],
    }
    json.dump(results, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
