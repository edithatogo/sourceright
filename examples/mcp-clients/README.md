# Sourceright MCP Client Examples

These examples show how local AI clients can launch the existing Sourceright
stdio MCP server:

```text
sourceright mcp
```

They are configuration examples only. They do not prove Claude, Codex, GitHub
Copilot, VS Code, Glama, or Smithery marketplace acceptance, and they do not
add a hosted HTTP endpoint.

## Files

- `claude-desktop.json` - Claude Desktop local stdio MCP configuration.
- `codex-config.toml` - Codex MCP server stanza.
- `codex-mcp.json` - JSON-shaped Codex-compatible launcher example for clients
  that import `mcpServers` blocks.
- `vscode-mcp.json` - VS Code and GitHub Copilot MCP configuration example.
- `generic-mcp-client.json` - generic `mcpServers` configuration.
- `generic-stdio.json` - transport-focused generic stdio manifest.
- `github-copilot-coding-agent.md` - Copilot coding-agent boundary note.
- `host-manifest.json` - host status and claim boundaries for Track 65.
- `smoke-requests.jsonl` - JSON-RPC requests for a manual stdio smoke test.

## Preconditions

Build or install the `sourceright` binary first, then confirm it is on `PATH`:

```bash
sourceright --version
sourceright mcp status
sourceright mcp tools --json
```

For a repo-local smoke test before installation, replace `command` with the
absolute path to the built binary, such as `target/debug/sourceright`.

## Manual Stdio Smoke

The checked-in smoke requests initialize the server, list tools, list
resources, list prompts, and call `workspace.init` without `apply: true`.

Run the server and paste the requests from `smoke-requests.jsonl` one line at a
time:

```bash
sourceright mcp
```

Expected results:

- `initialize` returns `serverInfo.name` as `sourceright`;
- `tools/list` includes read-only tools such as `mcp.status`;
- `resources/list` includes `sourceright://reports/reference-integrity`;
- `prompts/list` includes `manual_reference_review`;
- `workspace.init` returns `apply_requested: false`, `applied: false`, and no
  `audit_log`.

Only send a write-capable tool with `apply: true` when the mutation is
intentional and the affected workspace is disposable or already approved for
changes.
