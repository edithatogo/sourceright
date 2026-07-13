# Registry operator runbook

Refreshed: 2026-07-13

## Glama

The repository already contains a pinned `Dockerfile`, `glama.json`, canonical
MIT `LICENSE`, and an MCP server whose default container command is `mcp`.
Glama's documented release path is maintainer-controlled: claim the listing,
configure/deploy the Dockerfile, pass the handshake/security test, then publish
the release. After publication, verify:

```text
https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright
```

Record `tools`, `spdxLicense`, release state, and quality/TDQS fields. Do not
convert repository readiness into a live score until those fields change.

## Smithery

The public listing currently exposes the generated MCPB and the expected MCP
tools/resources/prompts. The remaining score requires a supported authenticated
release scan/install. The local builder is:

```powershell
pwsh -File scripts/build-smithery-mcpb.ps1 `
  -BinaryPath .\.tmp\sourceright-linux\sourceright `
  -Platform linux
```

Publish the resulting `.mcpb` using the Smithery account/API key, then record
the release identifier, status `SUCCESS`, scan output, and install smoke. The
unauthenticated release API is expected to return HTTP 401; never commit or
print the API key.

## Current boundary

As of this refresh, no Smithery API key is present in the local environment or
repository GitHub secrets. Glama and Smithery score changes therefore require
an authenticated registry-owner action after the repository-side fixes have
been published.
