# Claude Desktop MCPB

Build a Claude Desktop–compatible MCPB bundle from a release binary:

```powershell
powershell -NoProfile -File scripts/build-smithery-mcpb.ps1 `
  -BinaryPath $env:USERPROFILE\.cargo\bin\sourceright.exe `
  -Platform win32 `
  -OutputPath dist/sourceright-claude-desktop-0.1.20-win32.mcpb
```

Install via Claude Desktop **Settings → Developer → Edit Config** using stdio config in `examples/mcp-clients/claude-desktop.json`, or import the MCPB when Claude supports bundle install on your platform.
