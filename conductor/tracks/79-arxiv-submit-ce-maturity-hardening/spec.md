# arXiv submit-ce Maturity Hardening Spec

## Goal

Raise the arXiv `submit-ce` adapter to upstream-ready maturity, stability, and
testing evidence so that the integration surface is documented, fixture-backed,
and reviewable by arXiv maintainers via an upstream issue or PR.

## User outcome

arXiv maintainers and the Sourceright publication team can review a
fixture-backed integration proposal for `submit-ce` with:

- A documented adapter contract (platform enum, submission fixture, screening
  output schema).
- A recorded `journal-screen` smoke transcript exercising the synthetic
  submit-ce submission bundle.
- A proposed integration hook description (post-extraction / pre-submit
  invocation point and TeX log pipeline complement).
- A clear claim boundary that the adapter is **fixture-backed** and not
  **arXiv-approved integration**.

## Scope

- **submit-ce adapter fixtures**: `fixtures/journal/arxiv-submit-ce-submission.json`
  with synthetic CSL references, verification sidecar, and expected screening
  report. Covers `screened_with_warnings` status (1 verified reference, 1
  review-queue item, 1 AI-risk citation signal, missing DOI).
- **`journal-screen` command**: Local, read-only screening on the `arxiv-submit-ce`
  platform. Emits `sourceright.journal_screening.v1` with `editorial_summary`,
  `author_action_checklist`, and `reference_report`.
- **Synthetic submission bundle**: `source-package.tar.gz` analogue represented
  by fixture CSL + verification sidecar, simulating post-extraction bibliography
  metadata.
- **Integration hook proposal**: Documented in both the smoke evidence doc and
  as an upstream issue comment on `arXiv/submit-ce` (Issue #72), proposing:
  1. Post-extraction / pre-submit invocation of `sourceright journal-screen`.
  2. Complement to `submit_ce/ui/filters/tex_filters.py` citation warnings with
     structured CSL + verification evidence.
- **Platform detection**: `JournalPlatform::ArxivSubmitCe` enum variant in
  `src/journal.rs`, serialized as `arxiv_submit_ce`.
- **Evidence recording**: `journal-screen-smoke-2026-06-10.md` with smoke
  command, result, boundary, and proposed upstream hook.

## Out of scope

- arXiv API integration (no live API calls, no credentials, no submission state
  mutation).
- Writeback to canonical CSL or arXiv submission state.
- Live arXiv submission or acceptance flow (handled by Track 81).
- `arXiv/arxiv-submission-core` adapter — confirmed **legacy/inactive** by
  maintainer @dginev on issue #88; Track 80 covers migration hardening.
- OJS, ScholarOne, Editorial Manager, or other journal platforms.
- CI pipeline integration within arXiv's infrastructure.
- PHP/TeX plugin code for arXiv's own codebase.

## Data contracts

| Contract | Source | Format |
|---|---|---|
| arXiv submit-ce submission fixture | `fixtures/journal/arxiv-submit-ce-submission.json` | JSON — `{$schema, fixture_name, description, created, submission, manuscript, csl_references, verification_sidecar, expected_screening_report}` |
| Journal screening report | `src/journal.rs` → `screen_journal_submission()` | `sourceright.journal_screening.v1` — JSON with `submission_id`, `platform`, `status`, `editorial_summary`, `author_action_checklist`, `reference_report` |
| Platform enum | `src/journal.rs` → `JournalPlatform::ArxivSubmitCe` | Serialized as `arxiv_submit_ce` (snake_case) |
| Smoke evidence doc | `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/journal-screen-smoke-2026-06-10.md` | Markdown — command, result, boundary, proposed upstream hook |

## Claim boundary

**"Fixture-backed" not "arXiv-approved integration".** This track may claim that
the submit-ce adapter has a documented fixture, a recorded smoke transcript, and
a proposed integration hook. It must not claim that arXiv has reviewed, accepted,
or integrated the adapter. No claim of arXiv state mutation or live API
compatibility may be made without upstream acceptance evidence (Track 81
responsibility).

All evidence docs must include the disclaimer that the adapter is fixture-backed
and has not been reviewed or accepted by arXiv maintainers.

## Evidence level target

`fixture-backed` — All evidence is derived from local, deterministic, read-only
screening of synthetic fixture data. No live arXiv API calls, no arXiv
credentials, and no upstream acceptance evidence are required for this track's
completion.

Track 81 (`81-arxiv-upstream-submission-and-acceptance`) is responsible for
elevating from `fixture-backed` to `opt-in-live-proven` by recording upstream
issue/PR submission and maintainer response.

## Parallelization plan

- **Subagent A**: Verify `journal-screen` smoke against the arXiv submit-ce
  fixture — confirm exit code 0, correct platform detection, and expected
  `screened_with_warnings` status. Record evidence in
  `journal-screen-smoke-2026-06-10.md`.
- **Subagent B**: Draft integration hook proposal as upstream issue comment on
  `arXiv/submit-ce` Issue #72, referencing the local smoke evidence.
- **Subagent C**: Monitor upstream issue for maintainer feedback and record
  response in track evidence.

Subagent A must complete before Subagent B (smoke evidence must exist before
upstream proposal). Subagent C runs after B and continues asynchronously.
