# VS Code Extension Packaging Plan

1. Decide whether the first editor surface is diagnostics-only VSIX, MCP client
   configuration, or an explicit deferral.
2. Reuse CLI/MCP JSON contracts for diagnostics and previewed write plans.
3. Add extension packaging, Marketplace/Open VSX metadata, and smoke fixtures
   only after the contract is stable.
4. Keep development-only VS Code settings out of public extension claims.
