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
- `sourceright.submission-packets.schema.json`: submission packet inventories
  and live evidence manifests for external submission surfaces.
- `sourceright.live-submission-evidence.schema.json`: live submission evidence
  records for external submission surfaces and their public URLs.
