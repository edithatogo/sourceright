---
title: Publishing
description: Release, registry, and provenance controls for public distribution.
---

The Astro site is the canonical public docs surface, and publication work
should describe the same release sequence that the repository uses.

Publishing is gated by clean-tree checks, dependency policy, and release
provenance.

The full registry completion table, including accepted, prepared, deferred,
and not-applicable registries, is documented in
[Release Status](../release-status).
Track 69 adds the repo-local marketplace evidence model at
`conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md`.

- Crate publishing is manual and token-gated.
- MCP distribution metadata is tracked separately from the Rust crate.
- Registry publication should only happen after validation, checksums, and
  release notes are in place.
- Smithery publication is tracked separately in track 57. Sourceright now uses
  the MCPB/local path for its stdio server, with
  `smithery/mcpb/manifest.template.json` and
  `scripts/build-smithery-mcpb.ps1` as the prepared package surface. This is
  not accepted-listing evidence.
- Tag-triggered publish workflows emit a `release-status.md` artifact as the
  first-line release checklist.
- Release dry runs execute `scripts/verify-release-surface-refresh.ps1` before
  publication wording changes can claim accepted surfaces.
- `docs/src/` remains the archival Markdown source for parity review.
- Duplicate-dependency checks use `cargo tree -d --locked`.

## Host packages

Host-specific packages are tracked separately from the Rust core.
[Host Packaging](host-packaging) records the current boundaries:

- Claude, Codex, and generic MCP clients can use the local stdio MCP contract,
  with prepared examples in `examples/mcp-clients/`, but that is client
  configuration rather than a host plugin package.
- GitHub Copilot is prepared as a coding-agent workflow through repository
  instructions and setup steps; entitlement remains a GitHub-side setting.
- Zotero is prepared as a CLI/Web API adapter with fixture-backed
  preview/apply/audit proof, not as a Zotero `.xpi` or Plugin Gallery listing.
- OJS/PKP is prepared as a generic-plugin source skeleton with fixture-backed
  screening, not as PKP Plugin Gallery acceptance.
- VS Code, Microsoft Word, and LibreOffice require separate package tracks
  before release notes can claim installable editor or office-suite support.
- Marketplace acceptance requires URL, version, date, and install metadata in
  the release-status evidence table.
