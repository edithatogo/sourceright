---
title: Publishing
description: Release, registry, and provenance controls for public distribution.
---

The Astro site is the canonical public docs surface, and publication work
should describe the same release sequence that the repository uses.

Publishing is gated by clean-tree checks, dependency policy, and release
provenance.

- Crate publishing is manual and token-gated.
- MCP distribution metadata is tracked separately from the Rust crate.
- Registry publication should only happen after validation, checksums, and
  release notes are in place.
- Tag-triggered publish workflows emit a `release-status.md` artifact as the
  first-line release checklist.
- `docs/src/` remains the archival Markdown source for parity review.
- Duplicate-dependency checks use `cargo tree -d --locked`.
