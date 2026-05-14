# Track 57 — Smithery Distribution Readiness — Review

**Date:** 2026-05-14
**Status:** Completed as prepared MCPB path; Smithery listing not accepted

## Analysis Completed

- [x] Readiness assessment updated with current Smithery requirements.
- [x] Path B (MCPB/local) selected over Streamable HTTP because Sourceright's
  implemented MCP runtime is stdio.
- [x] `smithery/mcpb/manifest.template.json` added for MCPB manifest v0.3.
- [x] `scripts/build-smithery-mcpb.ps1` added to stage a `.mcpb` bundle from an
  existing release binary.
- [x] `tests/smithery_distribution_policy.rs` added to validate the manifest and
  the prepared-not-accepted docs boundary.
- [x] Publishing and release-status docs updated.

## Key Decision

Smithery MCPB readiness is implemented. Public Smithery availability is still
not claimed until a `.mcpb` bundle is published and the external listing is
verified.

## Evidence

- `smithery/mcpb/manifest.template.json`
- `scripts/build-smithery-mcpb.ps1`
- `tests/smithery_distribution_policy.rs`
- `docs/src/publishing.md`
- `docs/src/release-status.md`
- `docs-site/src/content/docs/guides/publishing.md`
- `docs-site/src/content/docs/release-status.md`
- `conductor/tracks.md`

## Deferred

- No Smithery account-side publish was performed.
- No accepted Smithery listing has been verified.
- Bundle install smoke is skipped until a concrete release binary is supplied to
  `scripts/build-smithery-mcpb.ps1`.
