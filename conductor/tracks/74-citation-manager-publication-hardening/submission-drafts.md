# Citation-Manager Submission Drafts (Not Submitted)

Date: 2026-06-09

**Approval gate:** `external_submission_allowed: false` for Zotero and EndNote in
`conductor/submission-requirements.json`. Do not post externally without explicit
approval recorded in `conductor/submission-packets/live-evidence.json`.

## GitHub Release notes snippet (draft)

> **Citation managers (technical preview):** Sourceright ships Zotero support as a
> CLI/Web API adapter with preview/apply/audit semantics — not a Zotero `.xpi`.
> EndNote support is deterministic ENW/RIS export for file import — not an
> EndNote plugin. See `docs/src/zotero-plugin-install.md` and
> `docs/src/citation-manager-integrations.md`.

## Zotero forum / community post (draft)

Subject: Sourceright Zotero CLI/Web API adapter (preview)

Body outline:

- Sourceright verifies references locally, then can sync to a disposable Zotero
  library through the Web API when credentials are supplied.
- Install: GitHub Release or `cargo install sourceright`; follow
  `docs/src/zotero-plugin-install.md`.
- **Not** a Zotero plugin; no `.xpi` or Plugin Gallery listing.
- Live smoke is opt-in via documented workflows.

## EndNote user guide excerpt (draft)

- Export verified CSL to ENW or RIS: `sourceright export --format enw` / `ris`.
- Import the file in EndNote using standard reference import.
- No EndNote plugin is provided; no live library sync is claimed.

## Rollback

- Withdraw forum posts or release-note language that implies plugin acceptance.
- Revert `docs/src/release-status.md` Zotero row if claim language drifts.
- Keep `publication-decision-2026-05-18.md` as the canonical package decision.
