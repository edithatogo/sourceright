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

## Deploy verification (2026-06-10)

- Astro prerender route builds on CI:
  `/.well-known/mcp/server-card.json (+5ms)` in Pages workflow `27230037375`.
- **GitHub project Pages still returns Starlight 404** for dot-prefixed paths
  (`/.well-known/...`, `/.nojekyll`) even though non-dot static assets such as
  `/pagefind/pagefind.js` return **200**.
- Repo-root card is live on raw GitHub:
  `https://raw.githubusercontent.com/edithatogo/sourceright/main/.well-known/mcp/server-card.json`

## Smithery republish attempts

| Publish URL | Release | Scan result |
| --- | --- | --- |
| `https://edithatogo.github.io/sourceright/` | `5d60e7ac`, `909ffec2` | **405** — no reachable server-card on homepage |
| `https://github.com/edithatogo/sourceright` | `8bbcb81c` | **422** — server-card not found at publish-origin well-known path |

Smithery requires `/.well-known/mcp/server-card.json` on the **publish URL origin**;
raw GitHub and prerendered dist output are not sufficient while project Pages
blocks dot paths.

## 2026-06-23 live probe

- GitHub Pages now serves `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json` with HTTP 200.
- Smithery publish against `https://edithatogo.github.io/sourceright/` still fails its scan with `405`.
- The deployment log says to advertise `/.well-known/mcp/server-card.json`, which suggests Smithery is checking the origin root rather than the project subpath.
- That keeps the publishing gate open even though the repo-side Pages fix is correct.

## 2026-06-23 bundle and URL publish probe

- `smithery mcp publish https://github.com/edithatogo/sourceright -n edithatogo/sourceright`
  still fails the scan with `422`.
- `smithery mcp publish dist\\sourceright-smithery-0.1.20-win32.mcpb -n edithatogo/sourceright --config-schema ...`
  is rejected because `--config-schema` can only be used when publishing a URL.
- The bundle path is therefore not an escape hatch for this repo's current Smithery
  configuration.
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
