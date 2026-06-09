# Smithery server-card scan fix (2026-06-10)

## Problem

GitHub URL publish created registry shell `edithatogo/sourceright` (listing **200**)
but release scan failed **422** because Smithery could not auto-scan a stdio MCP.
Deploy log required `/.well-known/mcp/server-card.json` or a scannable HTTP MCP.

## Fix implemented

1. `mcp::server_card()` in `src/mcp.rs` — derives SEP-1649 card from live
   `tools_list` / `resources_list` / `prompts_list` (14 / 8 / 5).
2. `sourceright mcp server-card [--json]` — CLI emission for regeneration.
3. `scripts/generate-mcp-server-card.ps1` — writes:
   - `mcp/server-card.json`
   - `docs-site/public/.well-known/mcp/server-card.json`
4. `tests/smithery_distribution_policy.rs` — policy test for checked-in card parity.
5. `docs/src/publishing.md` — documents URL-publish server-card contract.

Local docs build copies the card to
`docs-site/dist/.well-known/mcp/server-card.json` (verified 2026-06-10).

## Pre-deploy verification

- `npm run build` in `docs-site/` emits
  `dist/.well-known/mcp/server-card.json` (**11650** bytes, 2026-06-10).
- Live Pages URL still **404** before merge/deploy:
  `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json`

## Republish attempt (pre-deploy)

```text
smithery mcp publish https://edithatogo.github.io/sourceright/ -n edithatogo/sourceright
→ release 5d60e7ac-a5f2-4ef6-aa95-c041cf279a9e accepted (PENDING)
→ scan: Connection error 405 — server-card not yet reachable on homepage
```

## Remaining operator steps

1. Merge and deploy GitHub Pages (`pages.yml` uploads `docs-site/dist`).
2. Confirm live URL returns **200** with `serverInfo.version` **0.1.20**.
3. Republish Smithery against the **docs homepage**:

   ```bash
   smithery mcp publish https://edithatogo.github.io/sourceright/ -n edithatogo/sourceright
   ```

4. Wait for release scan to pass; record listing URL + scan status in
   `conductor/submission-packets/live-evidence.json`.

## Deferred

- Win32 MCPB `smithery mcp publish ./dist/*.mcpb` still returns **400**
  `No values to set` — separate from URL server-card scan.
