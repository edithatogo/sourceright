# VS Code Extension Packaging Decision

## Decision

Track 66 is closed as an explicit deferral with a stable future-extension
contract. Sourceright does not currently ship a VSIX, VS Code Marketplace
listing, or Open VSX listing.

The current VS Code-related assets are:

- `.vscode/settings.json`: development tooling for this repository only.
- `examples/mcp-clients/vscode-mcp.json`: VS Code MCP client configuration for
  launching the existing local stdio MCP server.

Neither asset is a Sourceright VS Code extension.

## Future Extension Contract

A future VS Code package must be a thin adapter over the Rust core. It must:

- invoke `sourceright` CLI JSON commands, the local stdio MCP server, or a thin
  LSP wrapper around those contracts;
- map stable Sourceright JSON diagnostics into VS Code Problems without
  re-parsing or reimplementing reference verification logic in TypeScript;
- respect VS Code Workspace Trust before reading manuscript or workspace files;
- keep live providers opt-in and off by default;
- keep write-capable actions preview-only until explicit apply semantics and
  audit logs are implemented; and
- document Marketplace and Open VSX publication evidence separately from local
  package scaffolding.

## Diagnostic Mapping

Initial diagnostics should come from these core outputs:

| Source | VS Code surface | Notes |
| --- | --- | --- |
| `sourceright validate-csl --json` | Problems for malformed CSL records | Range mapping starts at file/record path until document spans are available. |
| `sourceright report --json` | Problems for reference integrity warnings/errors | Severity maps from Sourceright report severity. |
| `sourceright citations <manuscript>` | Problems for missing/uncited/ambiguous citation issues | Range mapping should use citation spans only when provided by the core. |
| MCP resources/tools | Optional agent-facing diagnostics | MCP support is configuration, not extension packaging. |

## Publication Boundary

VS Code Marketplace and Open VSX remain deferred until a VSIX exists, installs
locally, runs fixture diagnostics, documents workspace trust, and has accepted
listing evidence. Track 69 records accepted/prepared/deferred marketplace
evidence.
