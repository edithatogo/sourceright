# Browser Listing Verification

Date: 2026-06-10

## Smithery

| Check | URL | Result |
| --- | --- | --- |
| Listing probe | `https://smithery.ai/servers/edithatogo/sourceright` | **404 — Server Not Found or Removed** |
| Search | `https://smithery.ai/search?q=sourceright` | No Sourceright result in first page |
| Publish entry | `https://smithery.ai/new` | Redirects to Smithery sign-in (GitHub/Google/email) |

Prepared local artifact: `dist/sourceright-smithery-0.1.20-win32.mcpb` (rebuilt
2026-06-10 from release binary).

### Smithery CLI publish attempt (2026-06-10)

Authenticated Smithery CLI (`namespace: edithatogo`) ran:

```powershell
smithery mcp publish dist\sourceright-smithery-0.1.20-win32.mcpb -n edithatogo/sourceright
```

| Step | Result |
| --- | --- |
| Registry record | `✓ Created server "edithatogo/sourceright"` (first run) |
| Win32 MCPB deployment | `✗ 400 {"error":"No values to set"}` |
| GitHub URL publish (`smithery mcp publish https://github.com/edithatogo/sourceright`) | Registry page **200**; release scan **422** (needs `/.well-known/mcp/server-card.json` or scannable HTTP MCP) |
| Public listing | Still **404** at `smithery.ai/servers/edithatogo/sourceright` |
| Registry search | No `edithatogo/sourceright` match in `smithery mcp search` |

Retry with `--debug` failed at deployment with the same `400` error. Blocker
remains open until a public listing URL is reachable and recorded in
`live-evidence.json`. Next operator step: complete publish in the Smithery web
UI at `smithery.ai/new` or contact Smithery support with the deployment error.

## Glama

| Check | URL | Result |
| --- | --- | --- |
| Listing probe | `https://glama.ai/mcp/servers/edithatogo/sourceright` | **404** |
| API probe | `https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright` | **404** |
| Search `sourceright` | `https://glama.ai/mcp/servers?q=sourceright` | No Sourceright/edithatogo match |
| Search `edithatogo` | `https://glama.ai/mcp/servers?q=edithatogo` | No Sourceright/edithatogo match |
| Add Server UI | `https://glama.ai/mcp/servers` → Add Server | Button present; **Sign Up** required for submission |

`glama.json` is already in the repository root with maintainer `edithatogo`.
Indexing requires pasting `https://github.com/edithatogo/sourceright` into
Glama's Add Server flow while signed in.

## Current live probe (2026-06-23)

| Check | URL | Result |
| --- | --- | --- |
| Pages root | `https://edithatogo.github.io/sourceright/` | **200** |
| Hidden-file sentinel | `https://edithatogo.github.io/sourceright/.nojekyll` | **404** |
| Well-known card | `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json` | **404** |

This confirms the source build is correct but the published Pages artifact was
still dropping hidden files. The Pages workflow now needs a redeploy with
`include-hidden-files: true` before Smithery can re-scan the publish origin.
## Blocker status

Smithery and Glama blockers in `conductor/submission-requirements.json` remain
open. Do not promote either surface to `submitted` or `publicly_accepted` until
a public listing URL is recorded in `live-evidence.json`.
