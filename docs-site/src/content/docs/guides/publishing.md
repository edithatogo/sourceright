---
title: Publishing
description: Release, registry, and provenance controls for public distribution.
---

Publishing is gated by clean-tree checks, dependency policy, and release
provenance.

- Crate publishing is manual and token-gated.
- MCP distribution metadata is tracked separately from the Rust crate.
- Registry publication should only happen after validation, checksums, and
  release notes are in place.
