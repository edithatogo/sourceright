# Track 80 — arXiv submission-core maturity hardening

Do not submit upstream from this legacy-contract track.

## Objective

Record a migration-safe, security-bounded local contract for the legacy
`arXiv/arxiv-submission-core` surface. This track is informational and does
not claim active upstream support, maintainer acceptance, or live integration.

## Phase 1: Discover [x]

- Confirmed the repository and existing fixture-backed submission-core slice.
- Recorded maintainer issue #88 and its redirect to `arXiv/submit-ce`.
- Confirmed the legacy surface has no supported elevation path.

## Phase 2: Lock the specification [x]

- Preserved the canonical CSL / verification / review-queue boundaries.
- Preserved the no-writeback, no-credential, and no-claim-truth boundaries.
- Pinned the migration target to Track 79 and the active `submit-ce` surface.

## Phase 3: Implement local hardening [x]

- Maintained the event-variant fixtures and contract snapshot.
- Added requirements evidence, migration mapping, security boundaries, and
  the evidence packet with the legacy-use disclaimer.
- Added mirrored `legacy/inactive` release-status rows with issue #88 and the
  Track 79 migration reference.
- Updated the upstream submission packet, track registry, and evidence ledger
  to describe the terminal `contracted` boundary.

## Phase 4: Run checks [x]

- Parsed all edited JSON registries and fixture files.
- Checked the required legacy disclaimer and release-status references.
- Ran `cargo fmt --check` and focused whitespace validation.
- Full Rust policy-test execution remains an environment-dependent check and
  must not be represented as passed unless executable evidence is available.

## Phase 5: Review and fixes [x]

- Corrected references to the canonical Track 78 directory.
- Removed stale review wording that implied this legacy track still had an
  upstream submission step.
- Reconciled the Track 80 ledger entry and refreshed the inventory date.

## Phase 6: External progress [deferred]

No upstream issue, pull request, credential use, or writeback is performed for
the inactive repository. Any active upstream coordination belongs to Track 81
and `arXiv/submit-ce` only.

## Terminal state

Track 80 is complete at the `contracted` evidence level. The adapter remains
legacy-documented and has not been reviewed or accepted by arXiv maintainers
for continued use. No active integration or acceptance claim is permitted.
