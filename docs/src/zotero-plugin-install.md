# Zotero Sync: Installation & Configuration

Sourceright integrates with Zotero via the **Zotero Web API (v3)** — not a traditional
`.xpi` browser plugin. This means a single `sourceright citation-sync` command handles
preview, apply, and audit workflows against your Zotero library from the command line,
CI pipeline, or MCP server runtime.

> **Architecture note:** The CLI/Web API approach was chosen over a `.xpi` browser plugin
> because Sourceright is a server-side/CLI tool. The Web API integration works in CI/CD
> pipelines, MCP server runtimes, and shared group libraries without requiring a browser
> extension. See [Architecture Decision: API Mode](#architecture-decision-api-mode).

---

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| Zotero desktop | 5.x or 6.x | Zotero desktop must be running so the Web API is accessible. |
| Zotero API key | N/A | Create one in [Zotero Settings](https://www.zotero.org/settings/keys). |
| Sourceright binary | latest | `cargo install --path .` or download a GitHub Release. |

---

## Step 1: Get a Zotero API Key

1. Log into the [Zotero Settings page](https://www.zotero.org/settings/keys).
2. Scroll to **API Keys** and click **Create new private key**.
3. Permissions needed:
   - **Allow library access**: Yes
   - **Notes**: Yes
   - **Files**: No
   - **Write access**: Yes (for apply mode)
4. Click **Save Key**. Copy the generated key.

> Keep your API key secure. Treat it like a password.

---

## Step 2: Configure Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `SOURCERIGHT_ZOTERO_API_URL` | Yes | -- | Zotero API base URL |
| `SOURCERIGHT_ZOTERO_API_KEY` | Yes | -- | Your 24-character Zotero API key |
| `SOURCERIGHT_ZOTERO_LIBRARY_ID` | Yes | -- | Library identifier |
| `SOURCERIGHT_ZOTERO_LIBRARY_TYPE` | No | `user` | Library type: user or group |

### Local API (recommended)

```powershell
$env:SOURCERIGHT_ZOTERO_API_URL = "http://127.0.0.1:23119/api"
$env:SOURCERIGHT_ZOTERO_API_KEY = "your-24-char-api-key-here"
$env:SOURCERIGHT_ZOTERO_LIBRARY_ID = "your-user-id"
$env:SOURCERIGHT_ZOTERO_LIBRARY_TYPE = "user"
```

```bash
export SOURCERIGHT_ZOTERO_API_URL="http://127.0.0.1:23119/api"
export SOURCERIGHT_ZOTERO_API_KEY="your-24-char-api-key-here"
export SOURCERIGHT_ZOTERO_LIBRARY_ID="your-user-id"
export SOURCERIGHT_ZOTERO_LIBRARY_TYPE="user"
```

### Public API (for remote/CI use)

```bash
export SOURCERIGHT_ZOTERO_API_URL="https://api.zotero.org"
export SOURCERIGHT_ZOTERO_API_KEY="your-24-char-api-key-here"
export SOURCERIGHT_ZOTERO_LIBRARY_ID="your-user-id"
export SOURCERIGHT_ZOTERO_LIBRARY_TYPE="user"
```

### Finding your Library ID

1. Open Zotero desktop.
2. Go to **Settings (or Preferences) > Advanced > Data Directory > Open**.
3. Open `prefs.js` and search for `extensions.zotero.users`.
4. Or visit `https://api.zotero.org/keys/<YOUR_API_KEY>` -- response includes userID.

---

## Step 3: Verify the Connection

```bash
sourceright citation-sync --preview
```

Expected output (truncated):

```json
{
  "schema_version": "sourceright.citation_sync.v1",
  "preview": true,
  "applied": false,
  "create_count": 0,
  "update_count": 3,
  "skip_count": 12,
  "conflict_count": 1,
  "actions": [ ... ],
  "audit_log_path": null
}
```

If connection fails: check Zotero is running, API key is valid, library ID matches.

---

## Basic Usage

### Preview mode (safe, no writes)

```bash
sourceright citation-sync
sourceright citation-sync --preview
sourceright citation-sync --preview /path/to/workspace/.sourceright
sourceright citation-sync --preview --remote-fixture zotero-items.json
```

### Apply mode (writes audit log + updates Zotero)

```bash
sourceright citation-sync --apply
sourceright citation-sync --apply --audit-log ./sync-audit.jsonl
sourceright citation-sync --apply --remote-fixture ./captured-items.json
```

### Audit log inspection

```bash
cat ./sync-audit.jsonl | jq '.action, .reference_id, .explanation'
```

---

## Example Workflow

1. **Init**: `sourceright init` creates `.sourceright/`.
2. **Populate**: Add CSL items to `references.csl.json`.
3. **Preview**: `sourceright citation-sync --preview`.
4. **Apply**: `sourceright citation-sync --apply --audit-log ./sync.jsonl`.
5. **Inspect**: `jq -r '.action' ./sync.jsonl | sort | uniq -c`.

---

## Compatibility Matrix

| Zotero Version | API Version | Local API | Public API | Notes |
|----------------|-------------|-----------|------------|-------|
| 5.x | v3 | Yes | Yes | Fully tested |
| 6.x | v3 | Yes | Yes | Fully tested |
| 7.x (beta) | v3 | Yes | Yes | Review preview output |
| Standalone | v3 | Yes | N/A | Same as desktop |

---

## Architecture Decision: API Mode

| Criterion | CLI/Web API (chosen) | .xpi Plugin (deferred) |
|-----------|---------------------|------------------------|
| CI/CD use | Yes | No (needs browser) |
| MCP integration | Yes | No |
| Group libraries | Yes | No (user-scoped) |
| Audit log on disk | Yes (JSONL) | No |
| Installation | No signing needed | Needs packaging |
| Zotero UI | No | Yes (in-browser) |

---

## Common Issues

- **Connection failed**: Zotero desktop not running, bad API key, wrong library ID.
- **No env vars set**: Sourceright works with `--remote-fixture` without live API.
- **Write denied**: API key needs write permission for `--apply`.
- **Library not found**: Check `SOURCERIGHT_ZOTERO_LIBRARY_ID` and `_TYPE`.

---

## Security Notes

- Store API keys in env vars or CI secrets, never in version control.
- Prefer local API (`127.0.0.1:23119`) over public API when possible.
- Always run `--preview` before `--apply`.
- Treat audit logs (`*.jsonl`) as provenance records.
