# Track 81 — arXiv Upstream Approval Gates

Date: 2026-06-09

## Gate sequence

1. **Requirements and hardening** — Tracks 78–80 complete (verified in
   `readiness-review-2026-06-09.md`).
2. **Maintainer drafts** — Issue/PR bodies drafted in Tracks 79–80 evidence
   packets; consolidated checklist in `submission-drafts.md`.
3. **Explicit approval** — Human approval recorded before any external GitHub
   action (plan step 4).
4. **Submission** — Create issue or PR; record URL, date, branch/artifact id
   (plan step 5).
5. **Evidence ledger** — Update to `submitted` only with URL evidence; never
   `accepted` from an open issue/PR (plan step 6).

## Approval checklist

- [x] Track 78 requirements matrix exists and inventory rows are searched.
- [x] Track 79 submit-ce hardened local package evidence exists.
- [x] Track 80 submission-core hardened local package evidence exists.
- [x] Readiness review recorded (`readiness-review-2026-06-09.md`).
- [x] Draft bodies include compatibility, security, fixtures, rollback, and
  no-writeback sections.
- [x] Explicit approval for external upstream submission recorded
  (`approval-record-2026-06-09.md`).
- [ ] Upstream issue or PR URL recorded after submission.
- [ ] Maintainer response path tracked until acceptance or deferral.

## Rollback

- Do not create upstream issues/PRs without approval.
- If submission occurs, revert evidence-ledger `submitted` claims if the URL is
  withdrawn or the issue/PR is closed without maintainer acceptance.
- Keep local adapters fixture-backed; provider data must never silently overwrite
  canonical CSL.
