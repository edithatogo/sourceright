# Install Sourceright MCP for Cline

Cline one-click marketplace install uses this file plus the repository README.

## Prerequisites

- [Sourceright CLI](https://github.com/edithatogo/sourceright/releases) on `PATH` (`sourceright --version`)
- Cline VS Code extension installed

## Stdio MCP (local)

Add to Cline MCP settings (`mcpServers`):

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"],
      "disabled": false
    }
  }
}
```

## Verify

```bash
sourceright mcp status
sourceright mcp tools --json
```

In Cline: open MCP Servers → confirm `sourceright` is enabled → run a read-only tool such as `mcp.status`.

## Smithery alternative

```
npx -y @smithery/cli@latest run edithatogo/sourceright
```

Use Smithery only when you prefer a managed bundle over a local CLI install.
