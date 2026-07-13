# Track 74 — Citation-Manager Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory row | Submission packet | Owning tracks |
| --- | --- | --- | --- |
| Zotero | `conductor/submission-requirements.json` → `zotero` | `conductor/submission-packets/citation-managers.md` | 58, 72, 74 |
| EndNote | `conductor/submission-requirements.json` → `endnote` | `conductor/submission-packets/citation-managers.md` | 59, 72, 74 |

## Official requirements sources

### Zotero

| Label | Kind | Path | Status | Retrieved |
| --- | --- | --- | --- | --- |
| Zotero plugin and extension distribution requirements | official-host-docs | https://www.zotero.org/support/dev/client_coding/plugin_development | searched | 2026-05-18 |

Zotero 7 developer docs describe `.xpi` plugin packaging and Plugin Gallery
distribution. Sourceright does **not** ship a `.xpi`; it ships a CLI/Web API
adapter documented in `docs/src/zotero-plugin-install.md`.

### EndNote

| Label | Kind | Path | Status | Retrieved |
| --- | --- | --- | --- | --- |
| EndNote import/export or plugin requirements | official-host-docs | https://endnote.com/ | searched | 2026-05-18 |
| EndNote RIS import (macOS) | official-host-docs | https://docs.endnote.com/docs/endnote/2025/macos/v1/content/08import/importing_ref_data_intoen.htm | searched | 2026-05-18 |
| EndNote RIS import (Windows) | official-host-docs | https://docs.endnote.com/docs/endnote/2025/v1/windows/en/content/08import/importing_refs_othbibsftwr.htm | searched | 2026-05-18 |

EndNote accepts correctly formatted RIS and related reference files. Sourceright
ships deterministic ENW/RIS exports, not an EndNote plugin.

## Package decisions (frozen)

See `publication-decision-2026-05-18.md`:

- **Zotero:** CLI/Web API adapter via Sourceright binary; no `.xpi`.
- **EndNote:** ENW/RIS file handoff; no EndNote plugin.

## Claim boundary

Allowed: "Zotero CLI/Web API adapter", "EndNote ENW/RIS file handoff",
"preview/apply/audit semantics", "fixture-backed export proof".

Disallowed without separate evidence: "Zotero plugin accepted",
"Zotero `.xpi`", "EndNote plugin", "EndNote live sync".
