# Remaining Live Submission Actions

Date: 2026-05-19  
Last browser sweep: 2026-06-10

This is the live-action runbook for the blockers that cannot be closed by local
repo evidence alone. Use it after the local readiness checks are green.

## Browser sweep (2026-06-10)

| Surface | Result | Operator action |
| --- | --- | --- |
| Smithery | Server-card fix **landed in repo** (`mcp/server-card.json`, docs `public/.well-known/...`). GitHub Pages URL still **404** until next `pages.yml` deploy. Homepage republish `5d60e7ac` scan failed **405** (no live card yet). Win32 MCPB still **400**. | Deploy docs to Pages, confirm `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json` → **200**, then `smithery mcp publish https://edithatogo.github.io/sourceright/ -n edithatogo/sourceright` |
| Glama | API and listing URLs return **404**; search has no match | Sign in at `glama.ai/mcp/servers`, use **Add Server** with `https://github.com/edithatogo/sourceright` |
| OJS/PKP | No `sourceright` entry in `pkp/plugin-gallery` | Open Gallery PR or run disposable OJS smoke when infrastructure exists |
| arXiv `submit-ce` | Issue **#72 open**; no maintainer acceptance yet | Monitor and reply on #72 if needed |
| arXiv `submission-core` | Issue **#88 closed**; legacy repo, no acceptance | Treat as informational; focus on `submit-ce` |

Evidence files:

- `conductor/tracks/73-mcp-directory-submission-hardening/browser-listing-verification-2026-06-10.md`
- `conductor/tracks/73-mcp-directory-submission-hardening/smithery-server-card-2026-06-10.md`
- `conductor/tracks/75-journal-platform-publication-hardening/browser-gallery-verification-2026-06-10.md`
- `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/upstream-monitor-2026-06-10.md`

## Current Blockers

| Surface | Packet | Current blocker | Action needed | Evidence to record |
| --- | --- | --- | --- | --- |
| Smithery | `mcp-directories` | Listing **200**; release scan blocked until Pages serves server-card (repo fix done 2026-06-10). | Deploy GitHub Pages, verify well-known URL, republish homepage URL to Smithery. | Public listing URL, passing release scan, install metadata, and date. |
| Glama | `mcp-directories` | No accepted Glama listing or API evidence (browser/API: 404). | Sign in, **Add Server**, paste `https://github.com/edithatogo/sourceright`, then claim via `glama.json`. | Listing URL or API result URL/body identifying Sourceright, plus date. |
| OJS/PKP | `journal-platforms` | Live OJS compatibility proof and PKP Gallery evidence missing (gallery search: no match). | Run disposable OJS smoke if infrastructure is available, then open a PKP Plugin Gallery PR or record why it is deferred. | Smoke log path, Gallery PR URL, or accepted Gallery listing URL. |
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
3. Glama listing/API verification.
4. Smithery listing verification or publish flow.
5. OJS disposable smoke and PKP Gallery PR.

This order records low-risk maintainer questions first and leaves the heavier
OJS live-instance work last.
