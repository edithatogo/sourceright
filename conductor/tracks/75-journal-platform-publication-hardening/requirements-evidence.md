# Track 75 — Journal-Platform Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory row | Submission packet | Owning tracks |
| --- | --- | --- | --- |
| OJS/PKP | `conductor/submission-requirements.json` → `ojs-pkp` | `conductor/submission-packets/journal-platforms.md` | 60, 72, 75 |

arXiv upstream (`arxiv-submit-ce`, `arxiv-submission-core`) is **out of scope** for
this track. See Tracks 78–81 and `conductor/submission-packets/arxiv-upstream.md`.

## Official requirements sources

| Label | Kind | Path | Status | Retrieved |
| --- | --- | --- | --- | --- |
| PKP Plugin Gallery and OJS plugin requirements | official-host-docs | https://docs.pkp.sfu.ca/dev/plugin-guide/ | searched | 2026-05-18 |

## Local package contract

- Generic plugin source under `plugins/ojs/sourceright/`
- Installable archive via `scripts/build-ojs-plugin-package.ps1`
- Repo-local lint via `scripts/ojs-plugin-lint.ps1`
- Fixture-backed journal screening via `fixtures/journal/ojs-submission.json`
- Optional disposable OJS smoke via `scripts/ojs-docker-install-smoke.ps1` (opt-in)

## Claim boundary

Allowed: "OJS generic plugin source skeleton", "install-test archive",
"fixture-backed journal screening", "hardened local package".

Disallowed without separate evidence: "PKP Plugin Gallery accepted",
"live OJS compatibility verified", "production editorial workflow wired".
