# Track 33 — Public Release and Registry Submission: Review

## Current State

**Status:** Completed
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
| MCP server manifest | `server.json` | MCP protocol server metadata |
| Glama manifest | `glama.json` | Glama registry metadata |
| Publishing guide | `docs/src/publishing.md` | Documents MCP registry, crate, and docs-site publishing |
| Release runbook | `docs/src/release-runbook.md` | Documents manual release sequence |
| SECURITY.md | `SECURITY.md` | Security policy present |
| Release status | `docs/src/release-status.md` | Registry completion table |

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

## Pre-Release Validation (2026-05-17, clean-tree re-validation)

Validation executed per the release runbook steps 2–5. Full results in `pre-release-validation.md`.

### Step 2: `cargo package --locked`

- **Strict:** PASSED on a clean tree. `cargo package --locked` packaged 248 files, 915.2 KiB uncompressed / 190.3 KiB compressed, then verified the crate successfully.
- **With `--allow-dirty`:** `cargo package --list --allow-dirty --locked` PASSED. Full manifest produced. All include-list directories accounted for. Test exclusion warnings expected for binary crate.

### Step 3: `cargo publish --dry-run --locked`

- **Strict:** PASSED on a clean tree. `cargo publish --dry-run --locked` synced the crates.io index, warned that `sourceright@0.1.20` already exists, verified the package, reached the upload step, and aborted upload due to dry run.

### Step 4: GitHub Release artifacts

- **Version:** v0.1.20 release at `https://github.com/edithatogo/sourceright/releases/tag/v0.1.20` with platform binaries + SHA-256 checksums. Accepted per `release-status.md`.

### Step 5: MCP image, server.json, and MCP registry metadata

- **server.json:** VALID against MCP 2025-12-11 schema (fetched, verified). `name` matches pattern (32 chars), `description` 86 chars (<=100), `version` 0.1.20 (semver, no ranges). Repository, Package, and transport all valid. Cross-artifact: version matches Cargo.toml, name matches Dockerfile MCP label, OCI identifier matches Dockerfile target, `$schema` URL resolves 200 OK.
- **glama.json:** VALID. `$schema` URL present, `maintainers: ["edithatogo"]`, dual LICENSE at root, `server.json` discoverable, public repo.
- **Dockerfile:** VALID. All 6 labels present and consistent. Multi-stage with pinned digests. ENTRYPOINT `["sourceright"]` + CMD `["mcp"]` = stdio transport. `--locked` in build. Minimal runtime.

### Summary

| Gate | Result |
|------|--------|
| `cargo package --locked` | PASSED |
| `cargo package --list --allow-dirty --locked` | PASSED |
| `cargo publish --dry-run --locked` | PASSED (dry-run upload aborted as expected) |
| `server.json` MCP 2025-12-11 schema | PASSED |
| `glama.json` structure | PASSED |
| `Dockerfile` OCI labels (6) | PASSED |
| Dockerfile build structure | PASSED |
| Cross-artifact consistency | PASSED |

## Remaining Boundaries

1. **Prepared, not accepted:** Direct GHCR package visibility, Glama, and
   Smithery remain prepared surfaces until external listings or package pages
   are verified.
2. **Future-version crate publish:** `cargo publish --dry-run --locked` must
   pass again before any version after `0.1.20` is published. The older
   2026-05-15 rerun encountered local disk exhaustion after package
   verification began, but the 2026-05-17 clean-tree re-validation passed for
   the already-published `0.1.20` package and aborted upload due to dry run as
   expected.
3. **Deferred marketplaces:** Homebrew, Scoop, winget, npm, PyPI, VS Code,
   Word, and LibreOffice remain deferred to their owning tracks and marketplace
   evidence gates.

## Key Findings

1. All 11 owned paths are present and populated with substantive content.
2. The release runbook is conservative: dry-run gates publishing, artifact verification happens before submission, and manual approval gates stay explicit.
3. Publishing guide covers the current accepted release path and keeps prepared/deferred channels separate.
4. Release-status.md provides the registry completion table used to avoid public overclaims.

## Evidence-Ledger Entry

A `"33-public-release-and-registry-submission"` entry has been registered in `conductor/evidence-ledger.json` with:

- **Category:** publication
- **Evidence level:** publicly-accepted for the core release surfaces; prepared or deferred for other registries.
- **Allowed claims:** GitHub Release, crates.io, docs.rs, and Official MCP Registry are accepted for `v0.1.20`; GHCR direct visibility, Glama, and Smithery remain prepared.
- **Blockers:** Future version publishes still require clean-tree review, successful dry run, and registry-specific approval. Prepared/deferred registries require their own external acceptance evidence.

## Status

- **Previous status:** in_progress
- **Current status:** completed
