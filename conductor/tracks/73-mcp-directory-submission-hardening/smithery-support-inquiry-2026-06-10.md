# Smithery Support Inquiry Draft

Date: 2026-06-10

Use this draft when contacting Smithery support (Discord, GitHub `smithery-ai/cli`, or
in-app support) about Sourceright stdio/MCPB publication blockers.

## Summary

We are publishing `edithatogo/sourceright`, a Rust stdio MCP server (MCPB bundle path).
Two blockers remain after implementing SEP-1649 server-card metadata in-repo.

## Issue 1 — MCPB deploy returns `400 {"error":"No values to set"}`

**CLI:** `smithery` v4.11.1  
**Command:**

```powershell
smithery mcp publish dist\sourceright-smithery-0.1.20-win32.mcpb -n edithatogo/sourceright
```

**Observed:**

- Registry record creation succeeds (`✓ Created server "edithatogo/sourceright"`).
- Deployment immediately fails with `400 {"error":"No values to set"}`.
- Reproduces on fresh qualified names (`edithatogo/sourceright-test-mcpb`, etc.).
- `--debug` shows failure originates in the deploy API call after server creation.

**Bundle details (after 2026-06-10 hardening):**

- `manifest_version`: `0.3`
- `server.type`: `binary` (`bin/sourceright.exe mcp` on Windows)
- `user_config.workspace_root` (optional directory)
- `tools` / `resources` / `prompts` embedded from `mcp/server-card.json`

**Ask:** What deploy payload fields are required for stdio MCPB releases? Is this a
known CLI/API regression for bundles without hosted runtime metadata?

## Issue 2 — GitHub project Pages cannot serve `/.well-known/*`

**Publish URL attempts:**

| URL | Scan result |
| --- | --- |
| `https://edithatogo.github.io/sourceright/` | **405** — no server-card at publish origin |
| `https://github.com/edithatogo/sourceright` | **422** — server-card not at publish-origin well-known path |

**Evidence:**

- Astro prerender builds `dist/.well-known/mcp/server-card.json` in CI.
- Live probe: `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json` → **404** (Starlight HTML 404).
- `https://edithatogo.github.io/sourceright/.nojekyll` → **404** (dot-prefixed paths not served).
- Raw GitHub works: `https://raw.githubusercontent.com/edithatogo/sourceright/main/.well-known/mcp/server-card.json` → **200**.

**Workaround prepared in-repo:**

- Cloudflare Worker proxy: `smithery/well-known-worker/` (serves well-known path on a
  non-Pages origin).
- Minimal HTTP command: `sourceright mcp serve-http` (local smoke / self-host).

**Ask:** For URL publish scans, can Smithery ingest the checked-in server card from
`raw.githubusercontent.com` (or another canonical repo path) when the declared publish
URL is a GitHub repo/docs link that cannot serve dot-prefixed static paths?

## References

- Repo: https://github.com/edithatogo/sourceright
- Server card: https://raw.githubusercontent.com/edithatogo/sourceright/main/.well-known/mcp/server-card.json
- Smithery listing (currently 404): https://smithery.ai/servers/edithatogo/sourceright
- Prior evidence: `smithery-server-card-2026-06-10.md`, `browser-listing-verification-2026-06-10.md`
