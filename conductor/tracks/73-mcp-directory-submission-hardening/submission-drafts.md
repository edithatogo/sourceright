# MCP Directory Submission Drafts

Date: 2026-06-09  
Status: Draft only — not submitted

## Smithery Listing Draft

**Title:** Sourceright  
**Summary:** Reference verification MCP server for CSL bibliographies, provider
evidence, and audited writes over stdio.  
**Package:** `dist/sourceright-smithery-0.1.20-win32.mcpb` (Windows; build
linux/darwin bundles before claiming cross-platform listing).  
**Install metadata:** Bundled binary runs `sourceright mcp`; 14 tools, 8
resources, 5 prompts in MCP status smoke.  
**Rollback:** Delist listing and publish a follow-up MCPB if tool/resource
counts or transport contract changes.

## Glama Listing Draft

**Repository:** `https://github.com/edithatogo/sourceright`  
**Metadata file:** `glama.json`  
**Maintainer:** `edithatogo`  
**Transport:** stdio via `sourceright mcp`  
**License:** MIT OR Apache-2.0  
**Rollback:** Remove or update Glama listing if `server.json`, OCI image tag, or
MCP surface contract diverges from the listed version.

## Approval Gate

Do not publish either draft until:

1. Explicit maintainer approval is recorded.
2. Target release version is frozen across `Cargo.toml`, `server.json`, and OCI
   labels.
3. `scripts/verify-live-submission-evidence.ps1` passes after recording URLs.
