# Track 81 — Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
|---|---|---|---|
| Issue submission record | Issues filed with URLs | `submission-drafts.md` — evidence table with repo, URL, date, status, and maintainer response columns | opt-in-live |
| Maintainer response tracking | Response URL/comment recorded for each issue | `upstream-monitor-*.md` — per-date monitor file with issue state, notes, and links to maintainer comments | opt-in-live |
| Acceptance boundary | Not claimed as accepted without PR merge, maintainer acceptance comment, or equivalent public acceptance URL | `submission-drafts.md` — Acceptance boundary section explicitly documents that open issue != accepted | default-CI |
| Rollback readiness | Docs updated if withdrawn, no release-status wording implies upstream acceptance without URL evidence | `submission-drafts.md` — Rollback section with revert instructions; verify no `accepted` claims in docs without evidence | default-CI |
| Live evidence recording | Evidence file exists and passes schema validation | `conductor/submission-packets/live-evidence.json` — entry for arXiv upstream with status, URL, and date | opt-in-live |
| Fixture JSON schema validity | Fixture JSON validates against OJS submission fixture schema | `fixtures/journal/arxiv-submit-ce-submission.json` validates against `schemas/ojs-submission-fixture.json` | default-CI |
| Claim boundary documented in all evidence docs | All track evidence docs include disclaimer: fixture-backed, not arXiv-reviewed | Review of all evidence docs in this track | default-CI |
