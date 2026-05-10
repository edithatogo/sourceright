# MCP Server Runtime Test Matrix

| Scenario | Expected result |
| --- | --- |
| Server startup | `sourceright mcp` starts a local MCP server and reports usable capability metadata. |
| Tool discovery | Existing read-only tools are discoverable through the MCP protocol. |
| Resource discovery | Existing read-only resources are discoverable without mutating workspace files. |
| Prompt discovery | Existing prompts are discoverable and stable. |
| Status compatibility | Existing status and manifest commands keep their current output contracts. |
| Invalid workspace | Missing or invalid workspace inputs return clear startup or request errors. |
| No implicit writes | MCP requests in this track do not modify `references.csl.json`, `references.verification.json`, or derived queues. |
| No live providers by default | Runtime tests do not require network access or provider credentials. |
