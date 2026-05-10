# Current repo snapshot for Codex context

Last checked by the assistant: 2026-05-10.

This snapshot is a guide only. Codex must inspect the local checkout before modifying files.

## Observed public repo state

- Repository: `edithatogo/sourceright`.
- The README describes Sourceright as Rust-first reference verification infrastructure for academic writing, agent workflows, and future legal citation work.
- The README workflow includes extraction, CSL JSON, standardisation/cleaning/verification/enrichment, verification sidecar, review queue, conflict resolution, citation reconciliation, reports, and exports.
- The README current status says the Rust core includes canonical CSL handling, verification sidecars, intake segmentation, provider evidence normalization, cleaning, conflict resolution, citation reconciliation, manual review queues, reporting, exports, journal screening contracts, legal citation records, and claim/source provenance graphs.
- Public `Cargo.toml` still shows a single package named `sourceright`, version `0.1.0`, not a workspace.
- Public `src/lib.rs` exports modules for cleaning, conflict, CSL, export, intake, journal, legal, provenance, providers, reconcile, report, review, sidecar, and workspace.
- Public `conductor/tracks.md` marks tracks 00-16 completed.

## Consequence

The older implementation overlay is stale. The updated Codex strategy is audit-first and additive.
