# Sourceright (Gemini CLI extension)

This extension registers the Sourceright stdio MCP server (`sourceright mcp`).

## Preconditions

Install the Sourceright CLI and ensure `sourceright` is on `PATH`:

```bash
sourceright --version
sourceright mcp status
```

## Install

```bash
gemini extensions install https://github.com/edithatogo/sourceright --ref main
# or from a local clone:
gemini extensions link /path/to/sourceright/extensions/gemini-sourceright
```

After install, restart Gemini CLI. MCP tools appear as `sourceright_*`.

## Claim boundary

Write-capable MCP tools remain preview-first; use explicit `apply` only in disposable workspaces.
