#!/usr/bin/env python3
"""Validate the source-level StandardFlow integration contract and fixture."""
from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "integration/standardflow/contract.json"
FIXTURE = ROOT / "integration/standardflow/fixtures/citation-evidence.json"
HEX40 = re.compile(r"^[a-f0-9]{40}$")


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"{path.relative_to(ROOT)} cannot be read as JSON: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


contract = load_json(CONTRACT)
fixture = load_json(FIXTURE)

for key in ("automatic_revision_updates", "default_network", "default_external_writes", "default_telemetry"):
    if contract.get(key) is not False:
        fail(f"{key} must be false")

for key in ("producer_baseline_revision", "consumer_revision"):
    value = contract.get(key)
    if not isinstance(value, str) or HEX40.fullmatch(value) is None:
        fail(f"{key} must be an exact 40-character revision")

contract_provenance = contract.get("artifact_provenance")
if not isinstance(contract_provenance, dict):
    fail("contract artifact_provenance is required")
if contract_provenance.get("kind") != "pull_request_head":
    fail("review-stage contract provenance must be pull_request_head")
if contract_provenance.get("repository") != "edithatogo/sourceright":
    fail("contract provenance repository is incorrect")
if contract_provenance.get("pull_request") != 105 or contract_provenance.get("immutable") is not False:
    fail("contract pull-request provenance must be explicit and mutable")
paths = contract_provenance.get("paths")
if not isinstance(paths, list) or not paths or any(not isinstance(path, str) or not path for path in paths):
    fail("contract provenance must list retrievable artifact paths")

if contract.get("producer_baseline_revision") != fixture.get("producer_revision"):
    fail("producer baseline differs between contract and fixture")
if fixture.get("schema_version") != "dev.standardflow.ecosystem-evidence.v1":
    fail("unexpected ecosystem evidence schema version")
if fixture.get("producer") != "sourceright" or fixture.get("evidence_type") != "citation_verification":
    fail("fixture producer or evidence type is incorrect")
if fixture.get("authority") not in {"advisory", "candidate_evidence"}:
    fail("fixture exceeds the permitted authority boundary")
fixture_provenance = fixture.get("artifact_provenance")
if not isinstance(fixture_provenance, dict):
    fail("fixture artifact_provenance is required")
if fixture_provenance != {
    "kind": "pull_request_head",
    "repository": "edithatogo/sourceright",
    "pull_request": 105,
    "path": "integration/standardflow/fixtures/citation-evidence.json",
    "immutable": False,
}:
    fail("fixture provenance must identify its mutable pull-request path exactly")
if not isinstance(fixture.get("subject"), str) or not fixture["subject"]:
    fail("fixture subject is required")
if fixture.get("payload", {}).get("canonical_write_performed") is not False:
    fail("fixture must prove that no canonical write occurred")
limitations = fixture.get("limitations")
if not isinstance(limitations, list) or not limitations or any(not isinstance(item, str) or not item for item in limitations):
    fail("fixture limitations must contain non-empty strings")
if not contract.get("rollback") or not contract.get("claim_boundary"):
    fail("rollback and claim boundary are required")

print("Sourceright StandardFlow integration validation passed")
