# MCP transcript snippets

Initialize, then inspect the advertised surface:

```text
Client -> Server
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"example","version":"0.1.0"}}}

Server -> Client
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","serverInfo":{"name":"sourceright","version":"0.1.0"},"capabilities":{"tools":{},"resources":{},"prompts":{}},"instructions":"Read-only local reference verification server"}}

Client -> Server
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
```

Dry-run first:

```text
Client -> Server
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"workspace.init","arguments":{"workspace":".sourceright"}}}

Server -> Client
{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"{\"schema_version\":\"sourceright.mcp_write_plan.v1\",\"tool\":\"workspace.init\",\"apply_requested\":false,\"applied\":false,\"workspace\":\".sourceright\",\"changes\":[...]}"}],"isError":false}}
```

Apply only when the mutation is intended:

```text
Client -> Server
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"workspace.init","arguments":{"workspace":".sourceright","apply":true}}}
```

The applied response includes `applied: true` and an `audit_log` path.
