# VS Code Marketplace / Open VSX Submission Drafts (Not Submitted)

Date: 2026-06-09

**Approval gate:** `vscode-open-vsx` has `external_submission_allowed: false` in
`conductor/submission-requirements.json`.

## VS Code Marketplace (draft)

- Publisher: `edithatogo`
- Extension id: `edithatogo.sourceright`
- Package: `dist/edithatogo.sourceright-0.1.20.vsix` (built via `vsce package`)
- Metadata body: `marketplace-metadata-draft.md`
- **Not submitted** — no Marketplace listing URL or acceptance receipt

## Open VSX (draft)

- Same VSIX artifact and metadata draft
- Separate Open VSX publisher token and registry workflow required
- **Not submitted** — no Open VSX listing URL or acceptance receipt

## Rollback

- Remove any release-status wording that implies Marketplace/Open VSX acceptance.
- Revert `submission_ready` or `publicly_accepted` gates if external listing is
  claimed without URL evidence.
