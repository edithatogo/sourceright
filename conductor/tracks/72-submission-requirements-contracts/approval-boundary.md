# External Submission Approval Boundary

Track 72 defines when Sourceright may create external issues, pull requests,
marketplace listings, or registry submissions. This boundary applies to every
surface in `conductor/submission-requirements.json`.

## Default Posture

- `approval_required: true` for every surface
- `external_submission_allowed: false` until explicit maintainer approval is
  recorded
- No surface may move to `submitted` or `publicly_accepted` without a concrete
  URL or reproducible smoke log in live evidence

## What Counts As Approval

Approval is explicit human consent to execute one named external action, for
example:

- publish a Smithery listing for release `vX.Y.Z`
- open arXiv issue `#draft-submit-ce` against `arXiv/submit-ce`
- submit OJS plugin package to PKP Plugin Gallery

Approval does **not** mean:

- local package build success
- draft submission body completion
- CI green on submission-readiness checks
- prepared metadata or account-side drafts

Record approval in maintainer notes or in
`conductor/submission-packets/live-evidence.json` under the relevant surface
before flipping `external_submission_allowed` or promoting gate status.

## Promotion Rules

1. Requirements must be searched and contracted before package work.
2. Hardened local package evidence must pass default-CI policy tests.
3. Submission-ready requires zero inventory blockers for that surface.
4. Submitted requires a public issue, PR, listing, or registry URL with date
   and artifact id.
5. Publicly accepted requires maintainer-verifiable acceptance evidence.

See `conductor/submission-packets/remaining-live-actions.md` for the current
live-action runbook and `scripts/verify-live-submission-evidence.ps1` for
evidence validation.

## Track 72 Claim Boundary

Track 72 may claim the submission requirements model exists. It must not claim
that any new external submission has been made or accepted.
