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

## Pre-Release Validation (2026-05-14)

Validation executed per the release runbook steps 2–5. Full results in `pre-release-validation.md`.

### Step 2: `cargo package --locked`

- **Strict:** BLOCKED by dirty working tree (8+ uncommitted files). This is correct gate behavior.
- **With `--allow-dirty`:** PASSED. Package assembles successfully. 21 test file exclusion warnings are expected (binary crate, tests not in `Cargo.toml` `include`).

### Step 3: `cargo publish --dry-run --locked`

- **Strict:** BLOCKED by same dirty-tree gate.
- **With `--allow-dirty`:** PASSED (warning: v0.1.20 already exists on crates.io). Package structure valid.

### Step 4: GitHub Release artifacts

- **Version:** v0.1.20 release exists at `https://github.com/edithatogo/sourceright/releases/tag/v0.1.20` with platform binaries + SHA-256 checksums. Accepted status confirmed in `release-status.md`.

### Step 5: MCP image, server.json, and MCP registry metadata

- **server.json:** VALID against MCP 2025-12-11 schema. All required fields (`name`, `description`, `version`) present. `name` matches regex pattern. `description` ≤ 100 chars. Package block has `registryType: oci`, `identifier`, `transport: { type: stdio }`. Cross-artifact: version matches Cargo.toml, name matches Dockerfile MCP label.
- **glama.json:** VALID. `$schema` URL present, `maintainers: ["edithatogo"]`, public LICENSE confirmed.
- **Dockerfile:** VALID. All 6 OCI labels present and consistent with Cargo.toml and server.json. Multi-stage build with pinned digests. Correct ENTRYPOINT/CMD for stdio MCP server.

### Summary

| Gate | Result |
|------|--------|
| `cargo package --locked` | ⚠️ BLOCKED (dirty tree) — passes with `--allow-dirty` |
| `cargo publish --dry-run --locked` | ⚠️ BLOCKED (dirty tree) — passes with `--allow-dirty` |
| `server.json` MCP schema | ✅ PASSED |
| `glama.json` structure | ✅ PASSED |
| `Dockerfile` OCI labels | ✅ PASSED |
| Cross-artifact consistency | ✅ PASSED |

## Gaps

1. **No first release cut** — All infrastructure validated, release runbook steps 2–5 verified against v0.1.20 artifacts. A clean-tree release cut for a future version (>0.1.20) remains the next live execution step.
2. **No registry submission evidence** — `release-status.md` now shows 4 accepted registries (GitHub Release, crates.io, docs.rs, Official MCP Registry). Glama and Smithery remain "prepared."
3. **Dependency chain** — This track gates on three dependencies (25, 27, 32). Pre-release validation confirms release infrastructure is sound regardless of dependency status.

## Key Findings

1. All 11 owned paths are present and populated with substantive content.
2. The release runbook is conservative (dry-run gates publishing, artifact verification before submission).
3. Publishing guide covers all three distribution channels with clear documentation.
4. Release-status.md provides a comprehensive registry completion table.

## Status

- **Previous status:** in_progress
- **Current status:** in_progress
