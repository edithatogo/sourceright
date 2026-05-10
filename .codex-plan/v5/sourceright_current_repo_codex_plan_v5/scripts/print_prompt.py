#!/usr/bin/env python3
from __future__ import annotations
import argparse
from pathlib import Path
PROMPTS = {
    "00": "00-inspect-current-repo.md",
    "01": "01-baseline-checks.md",
    "02": "02-schema-contracts.md",
    "03": "03-plugin-registry.md",
    "04": "04-demonstrators.md",
    "05": "05-benchmark-harness.md",
    "06": "06-policy-style-recency.md",
    "07": "07-provider-fixture-expansion.md",
    "08": "08-citation-manager-profiles.md",
    "09": "09-mcp-readonly-plan.md",
    "10": "10-final-review.md",
}
parser = argparse.ArgumentParser()
parser.add_argument("slice", choices=sorted(PROMPTS))
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
print((root / "codex" / "prompts" / PROMPTS[args.slice]).read_text(encoding="utf-8"))
