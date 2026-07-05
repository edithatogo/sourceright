# Track 81 — Implementation Plan

## Phase 1: Discover [x]

- [x] Track 78 requirements reconnaissance: arXiv upstream requirements documented (repo topology, contact surfaces, gate criteria).
- [x] Track 79 evidence packet: submit-ce maturity hardening evidence, drift/security artifacts, maintainer draft.
- [x] Track 80 evidence packet: submission-core maturity hardening evidence, migration/security artifacts, maintainer draft.
- [x] Track 72 submission-requirements-contracts: approval boundary defined, approval gate `external_submission_allowed` criteria documented.
- [x] Identify `arXiv/submit-ce` and `arXiv/arxiv-submission-core` as the two upstream surfaces.

## Phase 2: Lock spec [x]

- [x] Approval boundary defined: explicit approval required before filing any upstream issue.
- [x] Readiness review recorded: Track 78/79/80 evidence packets reviewed and gated.
- [x] Claim boundary agreed: "submitted" is not "accepted".
- [x] Evidence level target set to `publicly-accepted`.
- [x] Parallelization plan: submit-ce and submission-core monitoring can run in parallel.
- [x] spec.md and test-matrix.md drafted.

## Phase 3: Implement [x]

- [x] Issue drafts prepared with title, body, and supporting evidence for both repos.
- [x] `submission-drafts.md` created with issue URLs and evidence record table.
- [x] Explicit approval recorded (`approval-record-2026-06-09.md`).
- [x] Draft bodies cross-reference Track 79/80 evidence packets for full context.

## Phase 4: Run checks [x]

- [x] Approval boundary verified: `external_submission_allowed` gate passes.
- [x] Local smoke passes for both repos (submit-ce contract snapshot, submission-core contract snapshot).
- [x] Claim boundary reaffirmed in all evidence docs: fixture-backed, not arXiv-reviewed.
- [x] Security boundaries affirmed: no arXiv writes, no CSL mutation, read-only screening.

## Phase 5: Submit [x]

- [x] Issue filed — `arXiv/submit-ce` #72 (2026-06-09).
- [x] Issue filed — `arXiv/arxiv-submission-core` #88 (2026-06-09).
- [x] Follow-up posted on submit-ce #72 with local smoke + integration hook proposal.
- [x] Both URLs recorded in `submission-drafts.md` evidence table.

## Phase 6: Monitor [ ]

- [ ] Track maintainer responses on both issues.
- [ ] Record response URLs, comments, and state changes in `upstream-monitor-*.md`.
- [ ] Update `remaining-live-actions.md` with latest monitor status.
- [ ] If submission-core #88 is closed as legacy/inactive (confirmed 2026-06-10), focus monitoring on submit-ce #72.
- [ ] If maintainer redirects to a different repo/surface, create new issue and update this track.

## Phase 7: Accept [ ]

- [ ] Record PR merge URL, maintainer acceptance comment URL, or equivalent public acceptance URL.
- [ ] Record acceptance evidence in `conductor/submission-packets/live-evidence.json`.
- [ ] Update `evidence-ledger.json` track entry with `publicly-accepted` evidence level.
- [ ] Update `docs/src/release-status.md` to reflect arXiv acceptance.
- [ ] Run `scripts/verify-live-submission-evidence.ps1` to confirm evidence file passes validation.
- [ ] Run `scripts/verify-submission-readiness.ps1` to confirm gate promotion.

- [ ] Do not submit upstream. This track only documents requirements; upstream submission is handled by Track 81 after Tracks 79/80 hardening evidence packets are complete.
