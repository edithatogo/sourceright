# Track 80 — Implementation Plan

## Phase 1: Discover [x]

- [x] Track 78 requirements reconnaissance confirms `arXiv/arxiv-submission-core`
      is **legacy/inactive** — last release v0.6.1 (Jan 2019), Python 3.6,
      Travis CI, Pipenv build system.
- [x] Confirmed `arXiv/submit-ce` is the active intake surface per maintainer
      @dginev on `arXiv/arxiv-submission-core` issue #88 (2026-06-10).
- [x] Issue #88 filed on `arXiv/arxiv-submission-core` (2026-06-09) and closed
      (2026-06-10) with maintainer redirect to `submit-ce`.
- [x] Synthetic submission fixture exists at
      `fixtures/journal/arxiv-submission-core-submission.json` with 2 CSL
      references (1 verified, 1 unverified), verification sidecar, and expected
      `screened_with_warnings` status.
- [x] `JournalPlatform::ArxivSubmissionCore` enum variant exists in
      `src/journal.rs` (serialized as `arxiv_submission_core`).
- [x] Track 81 upstream monitor records submission-core issue #88 as closed
      with redirect comment:
      `https://github.com/arXiv/arxiv-submission-core/issues/88#issuecomment-4662122861`.

## Phase 2: Lock spec [x]

- [x] Scope defined: contract snapshot, evidence packet, security boundaries,
      migration notes, legacy status documentation, upstream issue record.
- [x] Claim boundary: **"legacy-documented" not "active-integration"**.
- [x] Evidence level target: **contracted** — no live API calls required.
- [x] Terminal evidence level: no elevation path — legacy adapter receives
      `contracted` with migration note to submit-ce.
- [x] Data contracts derived: submission fixture, journal screening schema,
      platform enum, evidence packet, security boundaries.
- [x] Spec locked in `spec.md` with parallelization plan (Subagent A → B, C ∥ B).

## Phase 3: Implement [ ]

- [ ] Create `evidence-packet.md` with sections:
  - Current state (legacy/inactive per maintainer)
  - Issue record (#88 filed 2026-06-09, closed with redirect)
  - Contract snapshot reference (from Track 78 + fixture)
  - Security boundaries (read-only, no arXiv API calls, no CSL mutation)
  - Migration path (redirect to submit-ce)
  - Claim boundary
  - Terminal evidence level note
- [ ] Create `security-boundaries.md` documenting:
  - Read-only screening — no arXiv submission state mutation
  - No arXiv API calls — fixture-backed only
  - No writeback to canonical CSL data
  - Platform enum `arxiv_submission_core` is informational only
  - Legacy warning notice cross-referencing issue #88
- [ ] Update `release-status.md` (or equivalent docs mirror) with:
  - `arXiv/arxiv-submission-core` → `legacy/inactive` status
  - Maintainer confirmation reference (issue #88)
  - Migration note directing users to `arXiv/submit-ce`
  - Reference to Track 79 (submit-ce maturity hardening)
- [ ] Cross-reference evidence packet and security boundaries in
      `conductor/submission-packets/arxiv-upstream.md` handoff doc.
- [ ] Verify `fixtures/journal/arxiv-submission-core-submission.json` is valid
      against `schemas/ojs-submission-fixture.json`.

## Phase 4: Run checks [ ]

- [ ] `cargo test` — verify `journal_screening` tests pass for submission-core
      fixture (no regressions from Track 79/78 changes).
- [ ] `cargo clippy --all-targets -- -D warnings` — no warnings.
- [ ] `cargo fmt --check` — no formatting issues.
- [ ] Fixture JSON validates against `schemas/ojs-submission-fixture.json`.
- [ ] Evidence packet references are correct (issue #88 URL, Track 78 matrix
      path, fixture path).
- [ ] Claim boundary verified in all evidence docs: no claim of "active
      integration" or "arXiv-approved".
- [ ] Legacy status note is accurate and references maintainer confirmation.

## Phase 5: conductor-review [ ]

- [ ] `evidence-packet.md` reviewed:
  - Current state accurately reflects legacy/inactive status.
  - Issue record includes issue URL, date filed, date closed, and maintainer
    redirect comment URL.
  - Contract snapshot references are correct and verifiable.
  - Security boundaries are complete and technically accurate.
  - Migration path directs users to the correct active surface (submit-ce).
  - Claim boundary reaffirmed: "legacy-documented" not "active-integration".
- [ ] `security-boundaries.md` reviewed:
  - Read-only scope is correctly described.
  - No arXiv API calls — confirmed fixture-backed only.
  - Legacy warning notice is prominent and references issue #88.
- [ ] `release-status.md` update reviewed:
  - Legacy/inactive status accurately noted.
  - Migration note is actionable and references Track 79.
- [ ] `spec.md` reviewed for claim boundary and scope accuracy.
- [ ] `plan.md` reviewed for completeness.
- [ ] `test-matrix.md` reviewed for acceptance criteria.
- [ ] `$conductor-review` applied before any surface promotion.

## Phase 6: Apply fixes [ ]

- [ ] Address any findings from conductor-review:
  - Fix incorrect issue URLs, fixture paths, or evidence references.
  - Clarify claim boundary language if overclaim detected.
  - Add missing security boundary details if gaps found.
  - Update migration path if migration notes are incomplete.
- [ ] Ensure all files pass `cargo fmt --check`,
      `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Phase 7: Progress — Record acceptance evidence [ ]

- [ ] Finalize `evidence-packet.md` with complete findings.
- [ ] Finalize `security-boundaries.md` with complete scope.
- [ ] Update `release-status.md` with legacy/inactive status for submission-core.
- [ ] Update `conductor/submission-packets/arxiv-upstream.md` with Track 80
      evidence cross-references.
- [ ] Update `conductor/tracks.md` with Track 80 status and purpose.
- [ ] Update `conductor/evidence-ledger.json` with Track 80 entry
      (evidence level: `contracted`).
- [ ] Hand off to Track 81 (upstream submission and acceptance) — submit-ce
      is the only active arXiv surface.
- [ ] Close Track 80 as **informational** — no active users identified, no
      further hardening required for legacy/inactive adapter.
- [ ] Commit all track files.
