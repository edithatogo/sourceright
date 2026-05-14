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
- `docs/src/` remains the archival Markdown source for parity review.
- Duplicate-dependency checks use `cargo tree -d --locked`.
