# OJS/PKP Gallery Submission Drafts (Not Submitted)

Date: 2026-06-09

**Approval gate:** `external_submission_allowed: false` for `ojs-pkp` in
`conductor/submission-requirements.json`. Do not open a PKP Plugin Gallery PR or
listing without explicit approval in `conductor/submission-packets/live-evidence.json`.

## Gallery listing draft (outline)

**Name:** Sourceright  
**Category:** Generic plugin  
**Summary:** Thin OJS generic plugin that calls the Sourceright CLI for
fixture-backed reference screening in editorial workflows.  
**Install:** Extract to `plugins/generic/sourceright`, run
`installPluginVersion.php`, configure `sourcerightCliPath`, enable in admin.  
**Compatibility:** OJS 3.3+ (see `ojs-compatibility-matrix.md`).  
**License:** Apache-2.0/MIT (repository license)  
**Support:** GitHub Issues on `edithatogo/sourceright`

## Release notes snippet (draft)

> **OJS plugin (technical preview):** Generic plugin source and install-test
> archive are available under `plugins/ojs/sourceright/`. Screening uses the
> Sourceright CLI with preview/apply/audit boundaries. No PKP Plugin Gallery
> acceptance is claimed. Live OJS install smoke remains opt-in.

## Rollback

- Withdraw any Gallery PR or public listing claim if compatibility smoke fails.
- Revert `docs/src/release-status.md` OJS row if wording implies Gallery acceptance.
- Keep arXiv upstream work on Tracks 78–81; do not bundle arXiv modules as OJS packages.
