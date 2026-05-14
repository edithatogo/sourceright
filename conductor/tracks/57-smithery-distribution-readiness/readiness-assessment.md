# Smithery Distribution Readiness Assessment

**Date:** 2026-05-14
**Decision:** MCPB/local path prepared; public listing still gated

## Publishing Path Analysis

### Path A: Streamable HTTP
- **Requires:** MCP server to support Streamable HTTP transport
- **Current state:** Server uses stdio transport — no HTTP `/mcp` endpoint
- **Effort:** Medium — requires additional transport layer implementation
- **Dependency:** A future HTTP transport track would need to add the hosted MCP
  threat model, CORS/auth behavior, and scan-safe metadata.

### Path B: MCPB/Local Packaging (Recommended)
- **Requires:** Pre-built binary package plus an MCPB `manifest.json`.
- **Current state:** `smithery/mcpb/manifest.template.json` and
  `scripts/build-smithery-mcpb.ps1` are present. The script stages a `.mcpb`
  bundle from an existing platform release binary.
- **Effort:** Low for release packaging; no protocol change required.
- **Dependency:** A release binary must be supplied to the builder before
  publishing the bundle to Smithery.

## Recommendation

**Path B (MCPB/Local)** is the selected approach. It now has:
1. A checked-in MCPB manifest template for the stdio runtime.
2. A PowerShell package builder that creates a `.mcpb` bundle from a release
   binary.
3. A Rust policy test that keeps the manifest/docs boundary from drifting.

## Gap Items

- [x] MCPB manifest template exists.
- [x] Local `.mcpb` package builder exists.
- [x] Docs classify Smithery as prepared, not accepted.
- [ ] No automated Smithery publish workflow.
- [ ] No accepted Smithery listing has been verified.
- [ ] Bundle install smoke still requires a concrete release binary artifact.

## Revisit Triggers

- When a platform release binary is available for a target bundle.
- When Smithery listing publication is explicitly approved.
- When a Streamable HTTP transport exists and remote Smithery URL publishing is
  preferred over local MCPB.

## Dependencies

- Track 56 (MCP registry release binding) — remains the official MCP registry
  binding path.
- Track 33 (public release) — supplies the stable binary artifacts used by the
  MCPB builder.
