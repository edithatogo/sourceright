# Schema Contracts

Sourceright keeps canonical CSL JSON separate from verification, review,
reporting, journal, legal, provenance, export, and policy metadata. The schemas
in `schemas/` describe the current external contracts for those non-CSL files.

The schemas are additive contracts for agents, CI, MCP clients, journal
integrations, and future plugins. They do not replace the Rust structs as the
implementation source of truth, and they should be updated when serialized Rust
models change.

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
- `sourceright.export-manifest.schema.json`: export artifact inventories. The
  exported YAML/XML files themselves continue to use `sourceright.export.v1`.
- `sourceright.sync-manifest.schema.json`: planned citation-manager sync
  manifests.
- `sourceright.citation-sync.schema.json`: `sourceright.citation_sync.v1`
  Zotero-first preview/apply reports, including suppressed and review-required
  suggestion counts.
- `sourceright.policy.schema.json`: journal or workflow policy inputs for later
  style, recency, and integrity checks.
- `sourceright.policy-report.schema.json`: deterministic policy-check report
  output.
- `sourceright.plugin-manifest.schema.json`: plugin manifest metadata,
  contracts, capabilities, fixtures, and safety posture.
- `sourceright.mcp-status.schema.json`: machine-readable readiness output for
  the MCP placeholder/status surface.

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

The sample files under `examples/workspace/.sourceright/` are intentionally
small and deterministic. They are suitable for documentation, local CLI
experiments, and future schema validation scripts.
