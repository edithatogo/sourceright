# Artifact and Schema Guide

This guide maps the main Sourceright files and reports to their roles so you
can tell canonical data from derived output at a glance.

## Core artifacts

| Artifact | Role | Notes |
| --- | --- | --- |
| `references.csl.json` | Canonical academic reference data | Clean CSL only. No provider evidence or workflow state. |
| `references.verification.json` | Verification sidecar | Holds provider candidates, conflicts, provenance, and review state. |
| `review-queue.jsonl` | Operational review queue | Derived from the sidecar for manual or agent review. |
| `sourceright.reference_report.v1` | Reference integrity report | Used by `report` and MCP-ready resource payloads. |
| `sourceright.journal_screening.v1` | Editorial screening report | Used by `journal-screen`. |
| `sourceright.legal_citation_report` | Legal citation report | Separate from academic CSL. |
| `sourceright.provenance_report` | Claim/source graph output | Describes provenance without asserting claim truth. |
| `sourceright.export_manifest.v1` | Export preview manifest | Shows what export files would be written. |
| `sourceright.sync_manifest.v1` | Citation-manager sync plan | Preview-first contract for sync workflows. |
| `sourceright.citation_sync.v1` | Citation-manager sync report | Preview/apply result with low-noise suggestion classes and audit path. |
| `sourceright.policy_report.v1` | Deterministic policy report | Used for style and recency checks. |

## Schema files

The JSON schemas in `schemas/` describe the external contract for each artifact.
They are additive contracts for CI, agents, and downstream tooling. The Rust
models remain the implementation source of truth.

## Boundary rules

- Canonical CSL stays clean.
- Verification sidecars store evidence, not bibliographic truth claims.
- Review queues are derived operational work.
- Export files are generated from canonical CSL, not from provider payloads.

## Validation order

1. Validate the canonical CSL file.
2. Validate the verification sidecar.
3. Validate any derived report or export manifest that the workflow emits.
4. Only then hand the artifact to a downstream consumer.

## Example data

The checked-in `examples/workspace/` tree and the benchmark fixtures are the
best small inputs for understanding the file shapes without needing live
providers or large collections.
