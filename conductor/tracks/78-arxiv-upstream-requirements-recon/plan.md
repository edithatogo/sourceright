# Track 78 — Implementation Plan

## Phase 1: Discover

- [ ] Inspect `arXiv/submit-ce` repo for contribution docs, CI config, schema
      contracts, testing patterns, security boundaries, and submission process.
- [ ] Inspect `arXiv/arxiv-submission-core` repo for contribution docs, CI config,
      schema contracts, testing patterns, security boundaries, and submission process.
- [ ] Confirm `arXiv/submit-ce` as the active intake surface per maintainer @dginev
      on `arXiv/arxiv-submission-core` issue #88.
- [ ] Confirm `arXiv/arxiv-submission-core` is legacy/inactive and document its
      last release (v0.6.1, Jan 2019) and last commit dates.

## Phase 2: Lock spec

- [ ] Define scope: both repos' requirements documented across 6 dimensions.
- [ ] Claim boundary locked: "requirements-documented" not "arXiv-compatible".
- [ ] Evidence level target set to `contracted`.
- [ ] Parallelization plan: submit-ce and submission-core inspection run in parallel.
- [ ] Spec locked in `spec.md` with parallelization plan (Subagent A → C, B → C).

## Phase 3: Implement

- [ ] Create `requirements-matrix.md` with structured table covering:
  - Contribution docs section
  - CI/CD config section
  - Schema/contracts section
  - Testing patterns section
  - Security boundaries section
  - Submission process section
  - Active vs legacy status row for each repo
- [ ] For each requirement dimension, document:
  - `arXiv/submit-ce` (active repo) findings
  - `arXiv/arxiv-submission-core` (legacy/inactive per maintainer) findings
- [ ] Add claim boundary section at bottom of `requirements-matrix.md`.
- [ ] Add handoff links for Track 79 and Track 80 implementers.
- [ ] **Do not submit upstream** — this track only documents requirements;
      upstream submission is handled by Track 81 after Tracks 79/80 hardening
      evidence packets are complete.

## Phase 4: Run checks

- [ ] `requirements-matrix.md` covers all 6 requirement dimensions for both repos.
- [ ] Claim boundary verified: no claim of "arXiv-compatible" or "arXiv-accepted".
- [ ] Active vs legacy status correctly noted for each repo.
- [ ] All referenced URLs are valid GitHub paths.
- [ ] Evidence level `contracted` is appropriate for reference-able GH files.
- [ ] Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` in workspace.
- [ ] Run `cargo test` to ensure no regressions.

## Phase 5: conductor-review

- [ ] `requirements-matrix.md` reviewed:
  - All 6 dimensions have documented findings for both repos.
  - Active vs legacy status correctly attributed.
  - No overclaims about arXiv compatibility.
  - Handoff links for Track 79/80 are correct.
- [ ] `spec.md` reviewed for claim boundary and scope accuracy.
- [ ] `plan.md` reviewed for completeness.
- [ ] `test-matrix.md` reviewed for acceptance criteria.
- [ ] `$conductor-review` applied before any surface promotion.

## Phase 6: Apply fixes

- [ ] Address any findings from conductor-review:
  - Fix incorrect repo URLs, config details, or schema references.
  - Clarify claim boundary language if overclaim detected.
  - Add missing requirement dimensions if gaps found.
- [ ] Ensure all files pass `cargo fmt --check`,
      `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Phase 7: Progress — Lock spec for downstream tracks

- [ ] Finalize `requirements-matrix.md` with complete findings.
- [ ] Update `conductor/tracks.md` with Track 78 status and purpose.
- [ ] Update `conductor/evidence-ledger.json` with Track 78 entry.
- [ ] Hand off `requirements-matrix.md` to Track 79 (submit-ce maturity hardening)
      and Track 80 (submission-core maturity hardening) implementers.
- [ ] Commit all track files.
