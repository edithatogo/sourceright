# MCP Distribution Test Matrix

| Scenario | Expected result |
| --- | --- |
| MCP metadata | `server.json` has stable name, version, package target, and stdio transport. |
| OCI labels | Container metadata declares the matching MCP server name. |
| Local stdio install | `sourceright mcp` remains the primary local server command. |
| Registry publish | Manual workflow can publish metadata after package artifacts exist. |
| Smithery readiness | Docs distinguish Streamable HTTP URL publishing from local MCPB distribution. |
| Glama readiness | Docs note that open-source listing depends on reproducible build and protocol introspection. |
