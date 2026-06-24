# Remaining Live Submission Actions

Date: 2026-05-19  
Last browser sweep: 2026-06-10  
Last operator session: 2026-06-10 (host plugins 83–90 + GitHub release assets)

This is the live-action runbook for the blockers that cannot be closed by local
repo evidence alone. Use it after the local readiness checks are green.

## Browser sweep (2026-06-10)

| Surface | Result | Operator action |
| --- | --- | --- |
| Smithery | **Accepted** — listing + registry gateway install smoke (`smithery-install-smoke-2026-06-10.md`). | Optional: Linux MCPB parity; restart Cursor to pick up `~/.cursor/mcp.json` if not already active |
| Glama | **Accepted** — id `c7qsbvekc1` listing/API **200** (`glama-acceptance-2026-06-10.md`) | None |
| OJS/PKP | Fixture + plugin archive smoke **passed** (`ojs-colab-smoke-2026-06-10.md`); Docker disposable skipped | Optional Docker + PKP Gallery PR |
| arXiv `submit-ce` | Issue **#72 open**; no maintainer acceptance yet | Monitor and reply on #72 if needed |
| arXiv `submission-core` | Issue **#88 closed**; legacy repo, no acceptance | Treat as informational; focus on `submit-ce` |
| Host plugins 83–90 | Packages on **v0.1.20** release; Cline **#1764 open** | Set `VSCE_PAT`/`OVSX_PAT`/`NPM_TOKEN` for marketplace/npm publish (`host-plugins-publish-runbook.md`) |

Evidence files:

- `conductor/tracks/73-mcp-directory-submission-hardening/browser-listing-verification-2026-06-10.md`
- `conductor/tracks/73-mcp-directory-submission-hardening/smithery-install-smoke-2026-06-10.md`
- `conductor/tracks/73-mcp-directory-submission-hardening/glama-acceptance-2026-06-10.md`
- `conductor/tracks/73-mcp-directory-submission-hardening/smithery-server-card-2026-06-10.md`
- `conductor/tracks/75-journal-platform-publication-hardening/browser-gallery-verification-2026-06-10.md`
- `conductor/tracks/75-journal-platform-publication-hardening/ojs-colab-smoke-2026-06-10.md`
- `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/upstream-monitor-2026-06-10.md`

## Current Blockers

| Surface | Packet | Current blocker | Action needed | Evidence to record |
| --- | --- | --- | --- | --- |
| Smithery | `mcp-directories` | ~~Gateway install~~ **Done** 2026-06-10 (`smithery-install-smoke-2026-06-10.md`, `live-evidence.json`). | None for win32 stdio path. | N/A |
| Glama | `mcp-directories` | ~~No listing~~ **Done** 2026-06-10 (`glama-acceptance-2026-06-10.md`). | None. | N/A |
| OJS/PKP | `journal-platforms` | PKP Gallery acceptance missing (fixture smoke done 2026-06-10). | Open PKP Plugin Gallery PR; optional disposable Docker via WSL/`pkp/containers`. | Gallery PR URL or accepted listing URL. |
| arXiv `submit-ce` | `arxiv-upstream` | ~~No upstream issue~~ **Done 2026-06-09** | Monitor https://github.com/arXiv/submit-ce/issues/72 for maintainer response. | Maintainer acceptance or PR merge URL when available. |
| arXiv `arxiv-submission-core` | `arxiv-upstream` | ~~No upstream issue~~ **Done 2026-06-09**; issue **closed** without acceptance | No further action unless maintainers reopen or redirect to another repo. | Maintainer acceptance or PR merge URL when available. |

## Exact Promotion Rules

- `verified` or `submission_ready` evidence can clear a local blocker only when
  it records a concrete URL or reproducible smoke log.
- `submitted` evidence can move a surface to `submitted`, but not accepted.
- `accepted` evidence requires a public listing, merged PR, maintainer
  acceptance comment, or equivalent public acceptance URL.
- Do not remove a blocker from `conductor/submission-requirements.json` until
  the corresponding evidence row exists in `live-evidence.json` or in the
  owning track evidence file.

## Evidence Capture

1. Copy `conductor/submission-packets/live-evidence.template.json` to
   `conductor/submission-packets/live-evidence.json`.
2. Replace placeholder URLs with the real listing, API, PR, issue, or smoke log
   evidence.
3. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-live-submission-evidence.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-submission-readiness.ps1
```

4. Promote the relevant gates only after the evidence verifier passes.

## Browser Submission Order

1. ~~arXiv `submit-ce` issue~~ (filed 2026-06-09, issue #72).
2. ~~arXiv `arxiv-submission-core` issue~~ (filed 2026-06-09, issue #88).
3. ~~Glama listing/API verification.~~ (accepted 2026-06-10)
4. ~~Smithery listing verification or publish flow.~~ (accepted 2026-06-10)
5. OJS PKP Gallery PR (fixture smoke done 2026-06-10; disposable Docker optional).

## Infrastructure note (Glama, OJS, disposable smoke)

Probe automation is checked in:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\azure\run-directory-probes.ps1
```

Latest probe (2026-06-10): Smithery listing **200**; Glama id `c7qsbvekc1` listing/API **200**
(slug-path URLs still **404**).

| Path | When to use |
| --- | --- |
| `scripts/azure/run-directory-probes.ps1` | **Windows default** — local Python probes + Azure account metadata |
| [google-colab-cli](https://github.com/googlecolab/google-colab-cli) | Linux/macOS/WSL — `colab run scripts/colab/directory-probes.py` |
| Azure Cloud Shell + `scripts/azure/directory-probes.sh` | Linux egress without WSL |
| `python scripts/colab/ojs-docker-smoke.py` | OJS fixture smoke on Windows (no Colab/Docker) |
| WSL + `colab run scripts/colab/ojs-docker-smoke.py` | Same smoke on Colab Linux VM |
| Azure VM / Container Apps + Docker | OJS disposable smoke when PKP containers are needed |

Pick one surface at a time; do not claim listing acceptance from probe scaffolding alone.

This order records low-risk maintainer questions first and leaves the heavier
OJS live-instance work last.
