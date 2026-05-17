# Zotero Sync: Installation & Configuration

Sourceright integrates with Zotero via the **Zotero Web API (v3)** — not a traditional
`.xpi` browser plugin. This means a single `sourceright citation-sync` command handles
preview, apply, and audit workflows against your Zotero library from the command line,
CI pipeline, or MCP server runtime.

Zotero's current public plugin documentation says plugins are installed from
`.xpi` files, that Zotero does not currently provide a list of available
plugins, and that most plugins are announced and discussed in the Zotero Forums.
An official plugin directory is planned. Because Sourceright does not ship a
`.xpi`, there is no Zotero Plugin Gallery submission to make for the current
adapter.

> **Architecture note:** The CLI/Web API approach was chosen over a `.xpi` browser plugin
> because Sourceright is a server-side/CLI tool. The Web API integration works in CI/CD
> pipelines, MCP server runtimes, and shared group libraries without requiring a browser
> extension. See the [Architecture Decision section](#architecture-decision-cliweb-api-mode) below
> and the full [Packaging Decision document](../../conductor/tracks/58-mature-zotero-plugin/packaging-decision.md).

---

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| Zotero desktop | 7.x+ for local API smoke | Zotero desktop must be running only when targeting the local API at `127.0.0.1:23119`. |
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

## CI and Live Smoke

Default CI uses fixture-backed Zotero JSON and does not call Zotero or mutate a
library. A manual GitHub Actions workflow, `.github/workflows/zotero-live-smoke.yml`,
adds two gates:

1. `fixture-smoke` runs deterministic Zotero adapter tests.
2. `disposable-library-smoke` runs the ignored live Web API preview test only
   when the protected `zotero-live-smoke` environment exposes:
   `SOURCERIGHT_ZOTERO_LIVE_SMOKE=1`,
   `SOURCERIGHT_ZOTERO_API_KEY`, and
   `SOURCERIGHT_ZOTERO_LIBRARY_ID`.

Use a disposable Zotero user or group library for this workflow. The live smoke
fetches Zotero items and plans citation-sync actions with `apply=false`; it does
not write to Zotero.

Running Zotero Desktop itself inside GitHub Actions is possible with a Linux
runner, `xvfb`, a downloaded Zotero build, and a temporary profile. That belongs
to a future `.xpi` plugin track because it tests plugin installation/loading in
the Zotero UI. The current Sourceright adapter is not a Zotero UI plugin, so the
more relevant CI proof is the Web API disposable-library smoke above.

The repository also includes `.github/workflows/zotero-desktop-smoke.yml` as an
experimental manual workflow for the local desktop API. It downloads Zotero on
an Ubuntu runner, starts it under `xvfb` with an isolated temporary profile, sets
`extensions.zotero.httpServer.enabled`,
`extensions.zotero.httpServer.localAPI.enabled`, and
`extensions.zotero.httpServer.port`, then probes
`http://127.0.0.1:23119/api/users/0/items?limit=1`. This workflow is not a
plugin loading test and should remain manual until its runner reliability is
proven.

On a developer machine, Zotero returning `403 Local API is not enabled` means
the desktop app is listening on port 23119 but the local API preference is not
enabled. In Zotero, use Settings -> Advanced -> Config Editor and confirm
`extensions.zotero.httpServer.enabled` is `true`,
`extensions.zotero.httpServer.localAPI.enabled` is `true`, and
`extensions.zotero.httpServer.port` is `23119`.

## Compatibility Matrix

| Zotero Version | API Version | Local API | Public API | Notes |
|----------------|-------------|-----------|------------|-------|
| 7.x+ | v3 | Expected | Yes | Fixture-backed locally; live Web API smoke is opt-in. |
| 8.x | v3 | Expected | Yes | Local API should be rechecked before claiming desktop smoke evidence. |
| Standalone/Desktop | v3 | Expected when local HTTP server is enabled | N/A | Use `http://127.0.0.1:23119/api/users/0/items` for the personal local library path. |

---

## Architecture Decision: CLI/Web API Mode

### Detailed rationale

For full reasoning, see:
[`conductor/tracks/58-mature-zotero-plugin/packaging-decision.md`](../../conductor/tracks/58-mature-zotero-plugin/packaging-decision.md)

### Comparison table

| Criterion | CLI/Web API (chosen) | .xpi Plugin (deferred) |
|-----------|---------------------|------------------------|
| CI/CD use | Yes (no browser needed) | No (requires Firefox/Chrome runtime) |
| MCP integration | Yes (stdio/HTTP transport) | No (browser-scoped API) |
| Group libraries | Yes (via `/groups/{id}` endpoint) | No (user profile scope) |
| Audit log on disk | Yes (JSONL, configurable path) | No (browser storage) |
| Installation | Binary + API key; no signing needed | `.xpi` packaging, signing, per-user install |
| Zotero UI required | No (CLI works headless) | Yes (in-browser extension) |
| Distribution channel | GitHub Releases, crates.io | Zotero Forums now; official plugin directory when available |
| Write permissions | API key scoping | Extension permissions manifest |

### What this means for users

1. **No browser plugin to install.** You get the `sourceright` binary via GitHub
   Releases or `cargo install`, configure a few environment variables, and run
   `sourceright citation-sync --preview` to verify.
2. **Works everywhere.** The same binary works on your desktop, in CI/CD pipelines,
   in MCP server runtimes, and on remote/headless servers (using the public API).
3. **Same preview/apply contract.** The `--preview`/`--apply` flags and JSONL audit
   logs behave identically regardless of whether you target the local Zotero API
   (`127.0.0.1:23119`) or the public API (`api.zotero.org`).

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
