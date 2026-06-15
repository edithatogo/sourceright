# MCP Directory Submission Hardening Spec

## Goal

Mature Official MCP Registry refresh, Smithery, and Glama submission evidence with bundle, metadata, smoke, and listing gates so that each directory has a verified listing URL, passing release scan, and recorded install metadata.

## User outcome

MCP directories show verified listing URLs for Sourceright, release scans pass (green), and install metadata is recorded for published servers — not just local preparation evidence.

## Scope

- **Smithery server-card fix**: `mcp::server_card()` implementation, CLI emission (`sourceright mcp server-card`), `scripts/generate-mcp-server-card.ps1`, and dual-location card writing (repo root + docs-site public).
- **Browser listing verification**: Manual Smithery and Glama listing probes recorded in `browser-listing-verification-*.md`, including CLI publish attempts, registry page status codes, and search result probes.
- **Glama listing**: `glama.json` repo-root metadata, Add Server flow navigation, and sign-up gating documented.
- **Release scan gates**: Smithery release scan result (422/405/green) recorded per publish attempt; policy tests enforce checked-in card parity.
- **Live evidence recording**: Accepted listing URLs and scan status written to `conductor/submission-packets/live-evidence.json`.
- **GitHub Pages dot-path 404 resolution**: Workaround or infrastructure fix so that `/.well-known/mcp/server-card.json` returns 200 on the Pages host.

## Out of scope

- HTTP MCP transport implementation.
- AI client packaging (Claude Desktop, Codex, GitHub Copilot).
- VS Code/Open VSX extension packaging.
- Win32 MCPB bundle deployment (Smithery `400 No values to set` error deferred).
- Homebrew/Scoop/winget packaging.

## Data contracts

| Contract | Source | Format |
|---|---|---|
| SEP-1649 server card | `mcp.rs` → `mcp::server_card()` | JSON with `serverInfo`, `tools`, `resources`, `prompts`, `authentication` |
| MCPB bundle format | `smithery/mcpb/manifest.template.json` | `.mcpb` ZIP archive |
| Smithery listing API | `https://smithery.ai/servers/{namespace}/{name}` | 200 (listed) / 404 (not found) / 422 (scan failed) |
| Glama listing API | `https://glama.ai/mcp/servers/{namespace}/{name}` | 200 (listed) / 404 (not found) |
| Live evidence ledger | `conductor/submission-packets/live-evidence.json` | JSON array of `{surface, url, status, version, date}` |

## Claim boundary

"Prepared" not "accepted" until a **public listing URL** exists in `live-evidence.json` and a **release scan returns green**. A registry record without a public listing page (Smithery 200 registry but 404 listing) must not be marketed as "listed on Smithery."

## Evidence level target

`opt-in-live-proven` — each directory listing must be verified by a human browser probe or automated probe at the live public URL.

## Parallelization plan

- **Subagent A**: Smithery server-card generation, CLI emission, script, and policy test.
- **Subagent B**: Browser listing verification for Smithery (listing probe, CLI publish, release scan).
- **Subagent C**: Browser listing verification for Glama (listing probe, Add Server flow, sign-up gating).
- **Subagent D**: GitHub Pages dot-path 404 investigation and resolution.
- **Subagent E**: Live evidence recording in `live-evidence.json`.

Subagents B and C (browser listing probes) can run in parallel. Subagents A and D must complete before Subagent E (live evidence recording).
