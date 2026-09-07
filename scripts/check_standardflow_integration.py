#!/usr/bin/env python3
"""Validate the source-level StandardFlow integration contract and fixture."""
from __future__ import annotations

import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "integration/standardflow/contract.json"
FIXTURE = ROOT / "integration/standardflow/fixtures/citation-evidence.json"
HEX40 = re.compile(r"^[a-f0-9]{40}$")


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))

for key in ("automatic_revision_updates", "default_network", "default_external_writes", "default_telemetry"):
    if contract.get(key) is not False:
        fail(f"{key} must be false")

for key in ("producer_revision", "consumer_revision"):
    value = contract.get(key)
    if not isinstance(value, str) or HEX40.fullmatch(value) is None:
        fail(f"{key} must be an exact 40-character revision")

if contract.get("producer_revision") != fixture.get("producer_revision"):
    fail("producer revision differs between contract and fixture")
if fixture.get("schema_version") != "dev.standardflow.ecosystem-evidence.v1":
    fail("unexpected ecosystem evidence schema version")
if fixture.get("producer") != "sourceright" or fixture.get("evidence_type") != "citation_verification":
    fail("fixture producer or evidence type is incorrect")
if fixture.get("authority") not in {"advisory", "candidate_evidence"}:
    fail("fixture exceeds the permitted authority boundary")
if fixture.get("payload", {}).get("canonical_write_performed") is not False:
    fail("fixture must prove that no canonical write occurred")
if not contract.get("rollback") or not contract.get("claim_boundary"):
    fail("rollback and claim boundary are required")

print("Sourceright StandardFlow integration validation passed")
