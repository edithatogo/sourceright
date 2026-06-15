# Track 80 — Test Matrix

## Legacy status documented — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| submission-core marked as legacy/inactive in release-status | `release-status.md` contains `arXiv/arxiv-submission-core` with status `legacy/inactive` and maintainer confirmation reference (issue #88) | `release-status.md` readback — status row for submission-core | Default-CI |
| Maintainer confirmation URL documented | Evidence packet records `https://github.com/arXiv/arxiv-submission-core/issues/88` with redirect comment `#issuecomment-4662122861` | `evidence-packet.md` — Issue record section contains both URLs | Default-CI |
| Active surface clearly identified | Evidence packet and release-status both state `arXiv/submit-ce` as the active intake surface | `evidence-packet.md` — Migration path section; `release-status.md` migration note | Default-CI |
| Claim boundary documented | All evidence docs contain claim boundary `"legacy-documented" not "active-integration"` and disclaimer text | Review of `evidence-packet.md`, `security-boundaries.md`, `spec.md` | Default-CI |
| Terminal evidence level noted | `spec.md` and `evidence-packet.md` state terminal evidence level is `contracted` with no elevation path | `spec.md` — Evidence level target section; `evidence-packet.md` — Terminal evidence level section | Default-CI |
| Track 79 referenced as active adapter | Migration path references `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/` and `arXiv/submit-ce` | `evidence-packet.md` — Migration path section | Default-CI |

## Security boundaries defined — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `security-boundaries.md` exists | File at `conductor/tracks/80-arxiv-submission-core-maturity-hardening/security-boundaries.md` | File exists and is readable | Default-CI |
| Read-only screening documented | `security-boundaries.md` states adapter performs read-only journal screening; no arXiv submission state is mutated | `security-boundaries.md` — Scope section | Default-CI |
| No arXiv API calls documented | `security-boundaries.md` states no arXiv API calls are made; fixture-backed only | `security-boundaries.md` — API boundary section | Default-CI |
| No CSL writeback documented | `security-boundaries.md` states no writeback to canonical CSL data | `security-boundaries.md` — Data flow section | Default-CI |
| Legacy warning notice present | `security-boundaries.md` contains prominent notice referencing issue #88 and warning that repo is legacy/inactive | `security-boundaries.md` — Warning section | Default-CI |
| Platform enum scope documented | `security-boundaries.md` states `arxiv_submission_core` platform enum is informational/migration only | `security-boundaries.md` — Platform enum section | Default-CI |

## Migration path documented — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Migration path from submission-core to submit-ce documented | `evidence-packet.md` contains a Migration path section directing users to `arXiv/submit-ce` | `evidence-packet.md` — Migration path section | Default-CI |
| Track 79 referenced | Migration path references Track 79 for submit-ce maturity hardening documentation | `evidence-packet.md` — Migration path section references `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/` | Default-CI |
| Track 81 referenced for upstream flow | Migration path references Track 81 for upstream submission and acceptance flow | `evidence-packet.md` — Migration path section references `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/` | Default-CI |
| Migration path is actionable | Migration notes include specific next steps for existing submission-core users (redirect to submit-ce, file issue on active repo) | `evidence-packet.md` — Migration path section contains actionable steps | Default-CI |

## Contract snapshot — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Submission fixture exists | `fixtures/journal/arxiv-submission-core-submission.json` exists and is valid JSON | File existence check; `cargo test` passes | Default-CI |
| Fixture validates against schema | Fixture JSON validates against `schemas/ojs-submission-fixture.json` | Schema validation check (manual or automated) | Default-CI |
| Platform enum variant exists | `JournalPlatform::ArxivSubmissionCore` exists in `src/journal.rs` and serializes to `arxiv_submission_core` | Code review of `src/journal.rs`; serialization roundtrip test | Default-CI |
| Screening report shape documented | Evidence packet describes expected screening report schema version, platform field, status field | `evidence-packet.md` — Contract snapshot section | Default-CI |
| Expected screening status correct | Fixture `expected_screening_report.status` is `screened_with_warnings` (matching 1 verified + 1 unverified reference) | `fixtures/journal/arxiv-submission-core-submission.json` — `expected_screening_report.status` field | Default-CI |
| Track 78 requirements matrix referenced | Evidence packet references `conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md` for broader requirements context | `evidence-packet.md` — Contract snapshot section contains reference | Default-CI |

## Handoff completeness — opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `arxiv-upstream.md` references Track 80 evidence | `conductor/submission-packets/arxiv-upstream.md` contains reference to Track 80 evidence packet | `arxiv-upstream.md` readback | opt-in-live |
| Track 81 implementer can verify legacy status | Track 81 implementer confirms Track 80 evidence packet provides sufficient documentation of submission-core's legacy/inactive status | Track 81 evidence references Track 80 | opt-in-live |
| No active users identified | No user requests, issues, or support tickets reference `arxiv-submission-core` platform in Sourceright | Search of repo issues, discussions, and changelog | opt-in-live |
| Evidence ledger updated | `conductor/evidence-ledger.json` contains Track 80 entry with `evidence_level: "contracted"` | `evidence-ledger.json` readback | opt-in-live |
| `tracks.md` updated | `conductor/tracks.md` contains Track 80 row with status `in_progress` or `completed` and purpose description | `tracks.md` readback | opt-in-live |

## Code integrity — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `cargo test` passes | All journal screening tests pass including submission-core fixture | `cargo test` exit code 0 | Default-CI |
| `cargo clippy` passes | No warnings with `-D warnings` | `cargo clippy --all-targets -- -D warnings` exit code 0 | Default-CI |
| `cargo fmt --check` passes | No formatting issues | `cargo fmt --check` exit code 0 | Default-CI |
| No regressions in Track 79 tests | submit-ce tests still pass after any documentation/code changes | `cargo test` — all `journal_screening` tests pass | Default-CI |