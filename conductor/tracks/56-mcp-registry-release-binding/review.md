# Track 56 — MCP Registry Release Binding — Review

**Date:** 2026-05-14
**Status:** Completed

## Evidence Checked

| Artifact | Status |
|---|---|
| `server.json` | Passed — name, schema, version, repository, stdio transport, and OCI image target are guarded by `tests/mcp_distribution_checks.rs`. |
| `glama.json` | Passed — Glama schema and maintainer metadata are validated separately from MCP Registry acceptance. |
| `.github/workflows/publish-mcp-registry.yml` | Passed — follows successful `Release` workflow completion, validates metadata, waits for the versioned GHCR image, and publishes through GitHub OIDC. |
| `.github/workflows/release.yml` | Passed — derives the release version, builds versioned/latest OCI tags, labels the image with the version, and emits provenance attestations. |
| `Dockerfile` | Passed — includes source, version, license, description, and MCP server labels. |
| `docs/src/release-status.md` and docs-site mirror | Passed — record Official MCP Registry as accepted for `0.1.20` while preserving the GHCR direct-visibility boundary as prepared. |
| Track 43 (registry completion) | ✅ Completed — provides registry status foundation |

## Local Regression Gates

`tests/mcp_distribution_checks.rs` now guards:

- `server.json` schema, name, repository, Cargo version, stdio transport, and versioned OCI image.
- `Dockerfile` source, server-name, and Cargo-version labels.
- Release workflow version-derived OCI labels.
- MCP registry workflow release-success gate, GHCR image wait, GitHub OIDC login, and publisher invocation.
- Release-status docs declaring Official MCP Registry acceptance while keeping direct GHCR listing status prepared.
- Glama metadata as an independent prepared directory surface.

## Boundaries

1. No external publication was attempted during this implementation slice.
2. The direct GHCR package page still requires separate visibility verification.
3. Glama and Smithery acceptance remain out of scope for Track 56.
4. Future version bumps must update `server.json`, `Dockerfile`, and release-status docs together or the regression tests will fail.
