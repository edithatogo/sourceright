# Track 80 — arXiv submission-core Maturity Hardening Spec

## Goal

Raise the legacy `arXiv/arxiv-submission-core` adapter to migration-safe maturity,
stability, and testing evidence so that any existing submission-core users have
a documented migration path to the active `arXiv/submit-ce` surface.

## User outcome

Legacy submission-core users and arXiv maintainers can review:

- A **contract snapshot** (platform enum, submission fixture, screening output
  schema) showing the adapter's current state.
- An **evidence packet** documenting the adapter's legacy/inactive status, the
  upstream issue filed, and the maintainer's redirect response.
- A **security boundaries doc** confirming that the adapter is read-only, makes
  no arXiv API calls, and mutates no arXiv submission state.
- **Migration notes** directing any existing submission-core users to the active
  `arXiv/submit-ce` surface (Track 79) and Track 81 for upstream submission
  flow.
- A **legacy status documentation** entry in `release-status.md` and any
  relevant docs mirrors.

## Scope

- **Contract snapshot**: `JournalPlatform::ArxivSubmissionCore` enum variant
  (serialized as `arxiv_submission_core`), synthetic submission fixture at
  `fixtures/journal/arxiv-submission-core-submission.json`, and expected
  `sourceright.journal_screening.v1` report shape.
- **Evidence packet**: `evidence-packet.md` recording current state
  (legacy/inactive per maintainer), issue record (issue #88 filed 2026-06-09,
  closed with redirect), contract snapshot reference (from Track 78), security
  boundaries, and migration path.
- **Security boundaries**: `security-boundaries.md` documenting read-only
  screening, no arXiv state mutation, no arXiv API calls, and the legacy
  warning notice.
- **Migration notes**: Documented migration path from `arxiv-submission-core`
  to `arxiv-submit-ce`, referencing Track 79 (submit-ce maturity hardening)
  and Track 81 (upstream submission and acceptance).
- **Legacy status documentation**: Update `release-status.md` (or equivalent
  doc) with a legacy/inactive status note for the submission-core platform.
  Include the maintainer confirmation reference (issue #88).
- **Upstream issue record**: Cross-reference issue #88 on
  `arXiv/arxiv-submission-core` (filed 2026-06-09, closed 2026-06-10 with
  maintainer redirect to `submit-ce`).

## Out of scope

- Active development on `arXiv/arxiv-submission-core` (repo is confirmed
  legacy/inactive by maintainer @dginev on issue #88).
- Live arXiv API access, credentials, or account setup.
- Writeback to canonical CSL or arXiv submission state.
- `arXiv/submit-ce` adapter work (handled by Track 79).
- Upstream submission and acceptance flow (handled by Track 81).
- OJS, ScholarOne, Editorial Manager, or other journal platforms.
- CI pipeline integration within arXiv's infrastructure.
- Code changes to `src/journal.rs` beyond documentation comments or deprecation
  notices.

## Data contracts

| Contract | Source | Format |
|---|---|---|
| arXiv submission-core submission fixture | `fixtures/journal/arxiv-submission-core-submission.json` | JSON — `{$schema, fixture_name, description, created, submission, manuscript, csl_references, verification_sidecar, expected_screening_report}` |
| Journal screening report | `src/journal.rs` → `screen_journal_submission()` | `sourceright.journal_screening.v1` — JSON with `submission_id`, `platform`, `status`, `editorial_summary`, `author_action_checklist`, `reference_report` |
| Platform enum | `src/journal.rs` → `JournalPlatform::ArxivSubmissionCore` | Serialized as `arxiv_submission_core` (snake_case) |
| Requirements matrix (Track 78) | `conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md` | Markdown — 6-dimension comparison including submission-core |
| Evidence packet | `conductor/tracks/80-arxiv-submission-core-maturity-hardening/evidence-packet.md` | Markdown — current state, issue record, contract snapshot, security boundaries, migration path |
| Security boundaries | `conductor/tracks/80-arxiv-submission-core-maturity-hardening/security-boundaries.md` | Markdown — read-only scope, no arXiv API calls, legacy warning |
| Upstream issue | `https://github.com/arXiv/arxiv-submission-core/issues/88` | GitHub issue — filed 2026-06-09, closed 2026-06-10 with maintainer redirect |

## Claim boundary

**"Legacy-documented" not "active-integration".** This track may claim that the
submission-core adapter's current state, security boundaries, and migration path
have been documented and that the upstream maintainer has confirmed the repo is
legacy/inactive. It must not claim that the adapter is actively maintained,
arXiv-approved, or compatible with arXiv's current submission infrastructure.

All evidence docs must include the disclaimer that the adapter is
**legacy-documented** and has **not been reviewed or accepted by arXiv
maintainers for continued use**.

No claim of arXiv state mutation, live API compatibility, or active integration
may be made.

## Evidence level target

**contracted** — All evidence is derived from reference-able sources: the
Track 78 requirements matrix, the upstream GitHub issue #88, the synthetic
fixture file, and local documentation. No live arXiv API calls, credentials,
or upstream acceptance evidence are required for this track's completion.

Track 81 (`81-arxiv-upstream-submission-and-acceptance`) is responsible for
elevating the submit-ce (active) surface from `fixture-backed` to
`opt-in-live-proven` by recording upstream issue/PR submission and maintainer
response. No elevation path exists for the legacy submission-core adapter —
its terminal evidence level is `contracted` with a migration note to submit-ce.

## Parallelization plan

- **Subagent A**: Compile contract snapshot — verify
  `JournalPlatform::ArxivSubmissionCore` enum variant, the submission-core
  fixture at `fixtures/journal/arxiv-submission-core-submission.json`, and
  expected screening report shape. Reference the Track 78 requirements matrix
  for the broader requirements context.
- **Subagent B**: Draft evidence packet (`evidence-packet.md`) — document
  current state (legacy/inactive), issue record (#88 filed 2026-06-09, closed
  with redirect), contract snapshot reference, security boundaries, and
  migration path to submit-ce.
- **Subagent C**: Create security boundaries doc (`security-boundaries.md`) and
  update `release-status.md` (or equivalent) with legacy/inactive status note
  for the submission-core platform.

Subagent A must complete before Subagent B (contract snapshot must be verified
before evidence packet is drafted). Subagent C can run in parallel with
Subagent B (security boundaries and release-status updates are independent of
the evidence packet's narrative).

