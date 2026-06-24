# MCP Directory Submission Packet

## Surfaces

- Official MCP Registry
- Smithery
- Glama

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| Official MCP Registry | <https://modelcontextprotocol.io/registry/about> and <https://modelcontextprotocol.io/registry/authentication> | 2026-05-18 | `server.json` must use standardized metadata and public install/distribution data; GitHub auth names must match the `io.github.*` form. |
| Smithery | <https://smithery.ai/docs/build/publish> | 2026-05-18 | Local stdio servers are distributed as prebuilt MCPB bundles; bundle build and install smoke are required before listing claims. |
| Glama | <https://glama.ai/> and Glama MCP API/listing examples | 2026-05-18 | Glama can inspect stdio/SSE/Streamable HTTP MCP servers; accepted status needs listing/API evidence, not just `glama.json`. |
| Cline MCP Marketplace | <https://github.com/cline/mcp-marketplace> | 2026-06-10 | Submit MCP server via marketplace issue (logo, repo URL, install proof); Track 90 owns acceptance evidence. |

## Local Gates

- Validate `server.json`, `glama.json`, and Smithery MCPB metadata against the
  target release.
- Build Smithery MCPB from a concrete release binary.
- Run stdio MCP transcript smoke.
- Record directory URL, version/date, and install metadata before acceptance.

## Blockers

- Glama listing/API evidence missing

## Draft Submission Body

Draft listing bodies with rollback notes are in
`conductor/tracks/73-mcp-directory-submission-hardening/submission-drafts.md`.
Smithery MCPB publication is recorded in `smithery-mcpb-publish-2026-06-10.md`
(listing https://smithery.ai/servers/edithatogo/sourceright, live evidence in
`live-evidence.json`). Glama local metadata verification is in
`glama-metadata-verification.md`. Glama external publication remains
approval-gated.

## Approval Gate

External Smithery or Glama account-side publication requires explicit approval.
