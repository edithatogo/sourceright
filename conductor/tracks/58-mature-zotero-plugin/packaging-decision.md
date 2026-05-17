# Packaging Decision: CLI/Web API vs `.xpi` Browser Plugin

**Date:** 2026-05-14
**Track:** 58 — Mature Zotero Plugin
**Status:** Decided

---

## Decision

Sourceright integrates with Zotero via the **Zotero Web API (v3)** through a CLI command
(`sourceright citation-sync`) and Rust library. No `.xpi` browser plugin is built or
distributed.

The "installed package" is the **Sourceright binary** itself, distributed via
GitHub Releases (native binaries) and crates.io (Rust crate).

---

## Why `.xpi` Is Not Needed

### 1. Full API access without a browser plugin

The Zotero Web API (v3) exposes every operation needed for citation sync:

| Operation | Zotero API Endpoint | Sourceright Command |
|-----------|--------------------|--------------------|
| List items | `/users/{id}/items` | `citation-sync --preview` |
| Read item | `/users/{id}/items/{key}` | `citation-sync --preview` |
| Create item | `POST /users/{id}/items` | `citation-sync --apply` |
| Update item | `PATCH /users/{id}/items/{key}` | `citation-sync --apply` |
| Delete item | `DELETE /users/{id}/items/{key}` | `citation-sync --apply` |
| Collections | `/users/{id}/collections` | Future |
| Attachments | `/users/{id}/items/{key}/children` | Future |

No browser context is needed. The REST API works from CLI, CI/CD pipelines,
MCP server runtimes, and any HTTP-capable client.

### 2. Server-side/CLI architecture

Sourceright is a **server-side and CLI tool**, not a browser plugin. Its primary
use cases are:

- **Research integrity checks** in CI/CD pipelines (e.g., checking a manuscript's
  references against a Zotero library before submission).
- **Batch sync operations** across group/shared libraries.
- **MCP server runtime** for agent-driven workflows.
- **Automated audit trails** via JSONL audit logs.

A `.xpi` browser plugin would require a separate codebase, Firefox/Chrome
extension packaging, signing, and per-user installation — none of which add
value for the server-side/CLI use case.

### 3. Works without Zotero UI

The Web API can target a local Zotero desktop instance (`http://127.0.0.1:23119/api`)
or the public Zotero API (`https://api.zotero.org`). Users do not need to interact
with the Zotero UI at all during a sync — the CLI handles everything.

### 4. Group/Shared library support

The Zotero Web API supports group libraries (`/groups/{id}/items`). A `.xpi`
browser plugin typically operates in the user's local profile scope. The CLI/API
approach naturally handles team workflows.

---

## What "Installation" Means

Since there is no `.xpi`, "installation" means obtaining and configuring the
Sourceright binary:

| Step | Detail |
|------|--------|
| **Get the binary** | Download from GitHub Releases (pre-built for Linux, macOS, Windows) or `cargo install sourceright` |
| **Configure credentials** | Set `SOURCERIGHT_ZOTERO_API_URL`, `SOURCERIGHT_ZOTERO_API_KEY`, `SOURCERIGHT_ZOTERO_LIBRARY_ID` env vars |
| **Verify** | Run `sourceright citation-sync --preview` — no Zotero plugin installation needed |
| **Optional: Zotero must be running** | The local API (`127.0.0.1:23119`) requires Zotero desktop to be open. The public API (`api.zotero.org`) does not. |

See `docs/src/zotero-plugin-install.md` for full installation instructions.

---

## Distribution Model

| Channel | Artifact | Audience | Status |
|---------|----------|----------|--------|
| **GitHub Releases** | Native binaries (Linux, macOS, Windows) + source tarball | All users | 🔵 Intended shareable package channel |
| **crates.io** | `sourceright` Rust crate | Rust developers | 🔵 Intended Rust package channel |
| **Zotero Forums** | Support/discussion | Community | 🔵 Deferred — no browser plugin to announce |
| **Future official Zotero plugin directory** | `.xpi` package | Zotero desktop users | ❌ Not applicable until Sourceright ships a Zotero UI plugin |

### Shareable package vs official acceptance

The **shareable package** is the Sourceright binary distributed through GitHub
Releases. This is the primary intended distribution channel and should remain
gated by release workflows and install smoke tests.

**Official acceptance** in a Zotero plugin directory is **not applicable** to
the current adapter because Sourceright is not a Zotero `.xpi` plugin. Zotero's
current public plugin page says plugins are installed from `.xpi` files, most
plugins are announced in the Zotero Forums, and an official plugin directory is
planned. If a future use case emerges for a Zotero UI plugin (e.g., in-plugin
citation checking), a separate track would handle `.xpi` packaging, Zotero
Desktop loading tests, signing/release hosting, forum announcement, and future
directory submission.

## CI Verification Path

The repository includes `.github/workflows/zotero-live-smoke.yml` as a manual,
secrets-gated live smoke path for the current Web API adapter. It runs fixture
tests by default and only calls Zotero when the protected `zotero-live-smoke`
environment supplies a disposable library API key and ID.

Running Zotero Desktop in CI is possible with `xvfb`, a downloaded Zotero Linux
build, and a temporary profile, but that proves `.xpi` installation/loading
rather than this CLI/Web API adapter. Keep UI plugin loading tests out of the
default CI path until a real `.xpi` package exists.

`.github/workflows/zotero-desktop-smoke.yml` records an experimental manual
local-desktop API smoke. It starts Zotero with a temporary profile, enables the
HTTP server/local API preferences, and probes `/api/users/0/items?limit=1`.
Treat it as local API availability evidence only, not plugin submission or
`.xpi` loading evidence.

---

## Deferred: `.xpi` Packaging

`.xpi` packaging is **deferred indefinitely** and will be revisited only if:

1. A proven demand for browser-based citation checking emerges.
2. Zotero 7+ removes or significantly changes the Web API.
3. An institutional requirement mandates a browser plugin.

Until then, the CLI/Web API model provides full functionality without the
overhead of browser extension development, signing, and distribution.

---

## References

- [Zotero Web API v3 Documentation](https://www.zotero.org/support/dev/web_api/v3/start)
- [Zotero API Key Settings](https://www.zotero.org/settings/keys)
- [Installation Documentation](../../../docs/src/zotero-plugin-install.md)
- [ADR 0003: Zotero Plugin After Preview/Apply Proof](../../../adrs/0003-zotero-plugin-after-proof.md)
