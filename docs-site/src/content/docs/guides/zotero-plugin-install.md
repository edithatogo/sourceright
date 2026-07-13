---
title: Zotero Sync Installation
description: Configure Sourceright citation-sync for Zotero Web API preview, apply, and audit workflows.
---

Sourceright integrates with Zotero through the Zotero Web API v3, not a
traditional `.xpi` browser plugin. The same `sourceright citation-sync` command
handles preview, apply, and audit workflows from the command line or CI. It is
not advertised as a read-only MCP tool surface.

Zotero's current public plugin documentation says plugins are installed from
`.xpi` files, that Zotero does not currently provide a list of available
plugins, and that most plugins are announced and discussed in the Zotero Forums.
An official plugin directory is planned. Because Sourceright does not ship a
`.xpi`, there is no Zotero Plugin Gallery submission to make for the current
adapter.

The API-first path is intentional: Sourceright is a CLI/server-side tool, so it
can work with shared group libraries and audit logs without requiring browser
extension packaging.

## Prerequisites

| Requirement | Notes |
| --- | --- |
| Zotero desktop | Keep Zotero running only for local API access at `127.0.0.1:23119`. |
| Zotero API key | Create one from Zotero account settings. |
| Sourceright binary | Install from cargo or a GitHub Release. |

## Environment

For local Zotero desktop use:

```powershell
$env:SOURCERIGHT_ZOTERO_API_URL = "http://127.0.0.1:23119/api"
$env:SOURCERIGHT_ZOTERO_API_KEY = "your-24-char-api-key-here"
$env:SOURCERIGHT_ZOTERO_LIBRARY_ID = "your-user-id"
$env:SOURCERIGHT_ZOTERO_LIBRARY_TYPE = "user"
```

For remote or CI use, set `SOURCERIGHT_ZOTERO_API_URL` to
`https://api.zotero.org`.

## Preview First

Preview mode is the default safe workflow:

```bash
sourceright citation-sync --preview
sourceright citation-sync --preview /path/to/workspace/.sourceright
sourceright citation-sync --preview --remote-fixture zotero-items.json
```

Apply mode writes an audit log and updates Zotero only when explicitly
requested:

```bash
sourceright citation-sync --apply --audit-log ./sync-audit.jsonl
```

## CI and Live Smoke

Default CI uses fixture-backed Zotero JSON and does not call Zotero or mutate a
library. The manual `.github/workflows/zotero-live-smoke.yml` workflow runs
fixture-backed adapter tests and can run the ignored disposable-library Web API
smoke when the protected `zotero-live-smoke` environment supplies
`SOURCERIGHT_ZOTERO_LIVE_SMOKE=1`, `SOURCERIGHT_ZOTERO_API_KEY`, and
`SOURCERIGHT_ZOTERO_LIBRARY_ID`.

Use a disposable Zotero user or group library. The live smoke runs with
`apply=false`, so it fetches items and plans actions without writing to Zotero.

Running Zotero Desktop inside GitHub Actions is possible with `xvfb`, a
downloaded Zotero build, and a temporary profile. That is useful only for a
future `.xpi` plugin loading test; the current adapter is not a Zotero UI plugin.

The repository also includes `.github/workflows/zotero-desktop-smoke.yml` as an
experimental manual workflow for the local desktop API. It downloads Zotero on
an Ubuntu runner, starts it under `xvfb` with an isolated temporary profile, sets
the Zotero HTTP server and local API preferences, then probes
`http://127.0.0.1:23119/api/users/0/items?limit=1`. This workflow is not a
plugin loading test and should remain manual until its runner reliability is
proven.

On a developer machine, Zotero returning `403 Local API is not enabled` means
the desktop app is listening on port 23119 but the local API preference is not
enabled. In Zotero, use Settings -> Advanced -> Config Editor and confirm
`extensions.zotero.httpServer.enabled` is `true`,
`extensions.zotero.httpServer.localAPI.enabled` is `true`, and
`extensions.zotero.httpServer.port` is `23119`.

## Security Notes

- Store API keys in environment variables or CI secrets, never in version
  control.
- Prefer local API access when possible.
- Always run preview before apply.
- Treat audit logs as provenance records.
