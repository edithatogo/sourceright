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
| Smithery | `mcp-directories` | Listing **200**; release scan blocked until Pages serves server-card (repo fix done 2026-06-10). | Deploy GitHub Pages, verify well-known URL, republish homepage URL to Smithery. | Public listing URL, passing release scan, install metadata, and date. |
| Glama | `mcp-directories` | No accepted Glama listing or API evidence (browser/API: 404). | Sign in, **Add Server**, paste `https://github.com/edithatogo/sourceright`, then claim via `glama.json`. | Listing URL or API result URL/body identifying Sourceright, plus date. |
| OJS/PKP | `journal-platforms` | Live OJS compatibility proof and PKP Gallery evidence missing (gallery search: no match). | Run disposable OJS smoke if infrastructure is available, then open a PKP Plugin Gallery PR or record why it is deferred. | Smoke log path, Gallery PR URL, or accepted Gallery listing URL. |
| arXiv `submit-ce` | `arxiv-upstream` | ~~No upstream issue~~ **Done 2026-06-09** | Monitor [arXiv/submit-ce issue #72](https://github.com/arXiv/submit-ce/issues/72) for maintainer response. | Maintainer acceptance or PR merge URL when available. |
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
1. Replace placeholder URLs with the real listing, API, PR, issue, or smoke log
  evidence.
1. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-live-submission-evidence.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-submission-readiness.ps1
```

1. Promote the relevant gates only after the evidence verifier passes.

## Browser Submission Order

1. ~~arXiv `submit-ce` issue~~ (filed 2026-06-09, issue #72).
1. ~~arXiv `arxiv-submission-core` issue~~ (filed 2026-06-09, issue #88).
1. Glama listing/API verification.
1. Smithery listing verification or publish flow.
1. OJS disposable smoke and PKP Gallery PR.

This order records low-risk maintainer questions first and leaves the heavier
OJS live-instance work last.
