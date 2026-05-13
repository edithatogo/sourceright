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

## Current publication evidence

As of 2026-05-13, the latest verified public release surface is `v0.1.20`.

| Surface | Status | Evidence |
| --- | --- | --- |
| GitHub Release | Accepted | `https://github.com/edithatogo/sourceright/releases/tag/v0.1.20`, published 2026-05-11. |
| crates.io | Accepted | `https://crates.io/crates/sourceright`, default/newest version `0.1.20`, not yanked. |
| docs.rs | Accepted | `https://docs.rs/crate/sourceright/0.1.20` returns HTTP 200. |
| Official MCP Registry | Accepted | `https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.edithatogo/sourceright` lists `0.1.20` as active and latest. |
| GHCR MCP image | Indirectly evidenced | The accepted MCP Registry listing points to `ghcr.io/edithatogo/sourceright-mcp:0.1.20`; direct GHCR package listing may require package read permissions. |
| Glama | Prepared, not verified accepted | `glama.json` is present, but no accepted external listing is recorded in this repo. |
| Smithery | Deferred | Smithery remains a future path until an HTTP or MCPB/local distribution decision is made. |
| Homebrew, Scoop, Chocolatey, winget, npm, PyPI | Not active | No package-manager manifests are currently maintained in this repo. |

This page is linked from [Operations Status](guides/operations-status).
