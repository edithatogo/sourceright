---
title: Zotero Sync Installation
description: Configure Sourceright citation-sync for Zotero Web API preview, apply, and audit workflows.
---

Sourceright integrates with Zotero through the Zotero Web API v3, not a
traditional `.xpi` browser plugin. The same `sourceright citation-sync` command
handles preview, apply, and audit workflows from the command line, CI, or MCP
server runtime.

The API-first path is intentional: Sourceright is a CLI/server-side tool, so it
can work with shared group libraries and audit logs without requiring browser
extension packaging.

## Prerequisites

| Requirement | Notes |
| --- | --- |
| Zotero desktop | Keep Zotero running for local API access. |
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

## Security Notes

- Store API keys in environment variables or CI secrets, never in version
  control.
- Prefer local API access when possible.
- Always run preview before apply.
- Treat audit logs as provenance records.
