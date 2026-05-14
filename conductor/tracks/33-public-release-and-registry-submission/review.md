# Track 33 — Public Release and Registry Submission: Review

## Current State

**Status:** In Progress
**Priority:** High
**Dependencies:** 25-release-and-registry-readiness, 27-mcp-distribution, 32-publishing-governance-and-provenance

## Evidence Found

### Required Artifacts — All Present

| Artifact | Path | Evidence |
|----------|------|----------|
| Release workflow | `.github/workflows/release.yml` | CI/CD release pipeline exists |
| Release dry-run | `.github/workflows/release-dry-run.yml` | Dry-run verification workflow exists |
| Crate publish | `.github/workflows/publish-crate.yml` | Crate publication workflow exists |
| MCP registry publish | `.github/workflows/publish-mcp-registry.yml` | MCP registry submission workflow exists |
| Dockerfile | `Dockerfile` | Container image build definition |
| MCP server manifest | `server.json` | MCP protocol server metadata with tools, resources, prompts |
| Glama manifest | `glama.json` | Glama registry metadata |
| Publishing guide | `docs/src/publishing.md` | Documents MCP registry, crate, and docs-site publishing |
| Release runbook | `docs/src/release-runbook.md` | Documents manual release sequence with dry-run, validation, and artifact checks |
| SECURITY.md | `SECURITY.md` | Security policy present |
| Release status | `docs/src/release-status.md` | Registry completion table with accepted/prepared/deferred/NA |

### Release Runbook

The runbook at `docs/src/release-runbook.md` specifies a conservative 8-step release sequence:

1. Confirm tag/release candidate passed CI
2. Run `cargo package --locked`
3. Run `cargo publish --dry-run --locked`
4. Validate GitHub Release artifacts and checksums
5. Validate MCP image, `server.json`, and MCP registry metadata
6. Publish crate only after dry run succeeds
7. Publish GitHub Release only after artifacts are staged
8. Submit MCP metadata only after image/labels match manifest

On `v*.*.*` tags, crate publish runs automatically with manual dispatch override.

### Publishing Guide

`docs/src/publishing.md` documents three distribution channels:
- MCP registry submission (Smithery, Glama, Official MCP Registry)
- Crates.io publication
- GitHub Pages docs-site deployment

## Gaps

1. **No first release cut** — All infrastructure is in place but no release has been executed.
2. **No registry submission evidence** — Registry entries exist in docs/src/release-status.md as "prepared" but not "accepted" pending live submission.
3. **Dependency chain** — This track gates on three dependencies (25, 27, 32).

## Key Findings

1. All 11 owned paths are present and populated with substantive content.
2. The release runbook is conservative (dry-run gates publishing, artifact verification before submission).
3. Publishing guide covers all three distribution channels with clear documentation.
4. Release-status.md provides a comprehensive registry completion table.

## Status

- **Previous status:** in_progress
- **Current status:** in_progress
