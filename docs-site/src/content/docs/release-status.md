---
title: Release Status
description: Shared release-status artifact emitted by the tag-triggered publish workflows.
---

The release-status artifact is the single human-readable summary emitted by the
tag-triggered publish workflows. It captures the release surface, tag, run URL,
and the primary evidence expected for a public release.

Each of the following workflows writes a `release-status.md` artifact:

- `release.yml`
- `publish-crate.yml`
- `publish-mcp-registry.yml`

Use the artifact together with the GitHub Release page, crate publication
result, MCP registry submission result, checksums, and attestations.

This page is linked from [Operations Status](guides/operations-status).
