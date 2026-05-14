# VS Code Extension Packaging Plan

1. [x] Decide whether the first editor surface is diagnostics-only VSIX, MCP
   client configuration, or an explicit deferral.
2. [x] Reuse CLI/MCP JSON contracts for diagnostics and previewed write plans.
3. [x] Add extension packaging, Marketplace/Open VSX metadata, and smoke
   fixtures only after the contract is stable.
4. [x] Keep development-only VS Code settings out of public extension claims.
