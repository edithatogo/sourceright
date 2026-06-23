# Plan

## Phases

### 1. [x] Discover
- Confirmed `arXiv/submit-ce` is the active intake surface per maintainer
  @dginev on `arXiv/arxiv-submission-core` issue #88.
- `arXiv/arxiv-submission-core` is **legacy/inactive** — all effort targets
  `submit-ce` only.
- `JournalPlatform::ArxivSubmitCe` enum variant exists in `src/journal.rs`
  (serialized as `arxiv_submit_ce`).
- Synthetic submission fixture exists at
  `fixtures/journal/arxiv-submit-ce-submission.json` with 2 CSL references
  (1 verified, 1 unverified), verification sidecar, and expected
  `screened_with_warnings` status.
- `screen_journal_submission()` in `src/journal.rs` handles the
  `JournalPlatform::ArxivSubmitCe` variant via the generic platform-agnostic
  pipeline.
- Evidence: `journal-screen-smoke-2026-06-10.md` records live smoke transcript.
- Claim boundary locked: "fixture-backed" not "arXiv-approved integration."

### 2. [x] Lock spec
- Scope defined: submit-ce adapter fixtures, `journal-screen` command, synthetic
  submission bundle, integration hook proposal.
- Claim boundary: "fixture-backed" not "arXiv-approved integration."
- Evidence level target: `fixture-backed` (Track 81 handles upstream elevation).
- Data contracts derived: submission fixture schema, journal screening schema,
  platform enum, smoke evidence doc format.
- Spec locked in `spec.md` with parallelization plan (Subagent A → B → C).

### 3. [x] Implement
- `src/journal.rs` already has `JournalPlatform::ArxivSubmitCe` variant — no
  code change required for platform detection.
- `fixtures/journal/arxiv-submit-ce-submission.json` — synthetic fixture with:
  - Submission ID `ARXIV-CE-2026-0001`, platform `arxiv-submit-ce`.
  - 2 CSL references: `ng-2026-preprint-screening` (verified, DOI present)
    and `unverified-2026-preprint` (unverified, missing DOI).
  - Verification sidecar with Crossref provider candidate (confidence 0.96).
  - Expected screening report: `screened_with_warnings`, `arxiv_submit_ce`.
- `journal-screen-smoke-2026-06-10.md` — evidence doc recording command,
  result, boundary, and proposed upstream hook.

### 4. [x] Run checks
- `cargo test` — `journal_screening_returns_editorial_and_author_outputs`
  passes; `journal_screening_blocks_when_extraction_has_no_references` passes.
- `cargo clippy --all-targets -- -D warnings` — no warnings.
- `cargo fmt --check` — no formatting issues.
- Fixture JSON validates against `schemas/ojs-submission-fixture.json`.
- Smoke command structure verified: correct platform flag, submission ID,
  manifest path.

### 5. [ ] Integration hook proposal
- Post follow-up comment on `arXiv/submit-ce` Issue #72
  (https://github.com/arXiv/submit-ce/issues/72) with:
  - Link to local smoke evidence and Track 79 spec.
  - Proposed integration surface:
    1. Post-extraction / pre-submit: export bundle JSON → invoke
       `sourceright journal-screen --platform arxiv-submit-ce` as external
       read-only step; surface `author_action_checklist` in submit UI.
    2. Complement `submit_ce/ui/filters/tex_filters.py` citation warnings with
       structured CSL + verification evidence.
  - Clear disclaimer: fixture-backed, not arXiv-reviewed.
- Reference upstream issue in `journal-screen-smoke-*.md` and Track 79 docs.
- Record comment URL in track evidence.

### 6. [ ] Maintainer feedback
- Monitor Issue #72 for maintainer response (preferred insertion point,
  PR requirements, compatibility notes).
- If maintainer provides guidance:
  - Update integration hook proposal based on feedback.
  - Adjust fixture or smoke evidence if needed.
- If no response within track window, document as "pending maintainer feedback"
  and proceed to record acceptance evidence at current maturity level.

### 7. [ ] conductor-review
- Smoke evidence doc (`journal-screen-smoke-*.md`) reviewed:
  - Command structure matches real CLI invocation.
  - Result matches expected `screened_with_warnings` status.
  - Proposed upstream hook is technically accurate and appropriately scoped.
  - Claim boundary verified: no claim of "arXiv-approved integration."
- Integration hook proposal reviewed for technical accuracy and completeness.
- All evidence docs checked against claim boundary.
- `$conductor-review` applied before any surface promotion.

### 8. [ ] Apply fixes
- Address any findings from conductor-review:
  - Fix smoke command flags or paths if incorrect.
  - Clarify claim boundary language if overclaim detected.
  - Update integration hook proposal if technical inaccuracies found.
- If maintainer feedback received:
  - Adjust integration hook description, fixture data, or evidence docs
    accordingly.
- Ensure all files pass `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`.

### 9. [ ] Progress — Record acceptance evidence
- Finalize `journal-screen-smoke-*.md` with:
  - Complete smoke command and result.
  - Upstream issue URL and comment reference.
  - Maintainer feedback status (if any).
  - Claim boundary reaffirmed.
- Update `conductor/submission-packets/live-evidence.json` with submit-ce
  fixture-backed evidence entry (status: `fixture_backed`).
- If Track 81 is ready, hand off evidence for upstream submission and
  acceptance elevation.
- Commit all evidence docs.

- [ ] Do not submit upstream. This track only documents requirements; upstream submission is handled by Track 81 after Tracks 79/80 hardening evidence packets are complete.
