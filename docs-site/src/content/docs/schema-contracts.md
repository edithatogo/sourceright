---
title: Schema contracts
description: Validation rules for the public JSON documents and reports.
---

Schema contracts define the stable JSON surfaces for references, reports, and
workflow artifacts.

- Validate inputs before they are accepted.
- Keep exported JSON deterministic and documented.
- Keep canonical CSL separate from verification, provenance, policy, and
  operational review metadata.

## Current Schemas

- `sourceright.verification.schema.json`: `references.verification.json`
  sidecar records keyed by CSL `id`.
- `sourceright.review-queue.schema.json`: JSONL review queue entries derived
  from the verification sidecar.
- `sourceright.reference-report.schema.json`: `sourceright.reference_report.v1`
  report JSON and MCP resource payloads.
- `sourceright.journal-screening.schema.json`:
  `sourceright.journal_screening.v1` editorial screening output.
- `sourceright.arxiv-submission-fixture.schema.json`: synthetic current and
  legacy arXiv submission-platform fixtures for the journal screening contract.
- `sourceright.submission-requirements.schema.json`: machine-readable
  submission-readiness inventory, evidence gates, blockers, and health target.
- `sourceright.submission-packets.schema.json`: machine-readable submission
  packet index for packet paths, local validation, blockers, and approval gates.
- `sourceright.legal-citation-report.schema.json`: legal citation reports that
  stay separate from CSL JSON.
- `sourceright.provenance-report.schema.json`: claim/source provenance graphs
  that describe linkage without claim-truth scoring.
- `sourceright.export-manifest.schema.json`: export artifact inventories.
- `sourceright.sync-manifest.schema.json`: planned citation-manager sync
  manifests.
- `sourceright.citation-sync.schema.json`: `sourceright.citation_sync.v1`
  preview/apply reports, including suppressed and review-required suggestion
  counts.
- `sourceright.policy.schema.json`: journal or workflow policy inputs for style,
  recency, and integrity checks.
- `sourceright.policy-report.schema.json`: deterministic policy-check report
  output.
- `sourceright.plugin-manifest.schema.json`: plugin manifest metadata,
  contracts, capabilities, fixtures, and safety posture.
- `sourceright.mcp-status.schema.json`: machine-readable readiness output for
  the MCP status surface.

## Compatibility Rules

`references.csl.json` remains canonical CSL data. Provider candidates,
extraction provenance, conflicts, review decisions, and report findings belong
in the sidecar or derived reports.

Schema versions match current Rust constants where those constants exist:

- `sourceright.verification.v1`
- `sourceright.reference_report.v1`
- `sourceright.journal_screening.v1`
- `sourceright.export.v1`
- `sourceright.citation_sync.v1`
- `sourceright.policy_report.v1`

The legal citation and provenance reports currently do not emit a top-level
`schema_version`, so their schemas describe the live output shape without
requiring one.
