# Test Matrix

## Fixture smoke — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `arxiv-submit-ce` fixture produces journal screening report | `screen_journal_submission()` with `JournalPlatform::ArxivSubmitCe` and fixture CSL + sidecar produces `JournalScreeningReport` with `schema_version == "sourceright.journal_screening.v1"` | `src/journal.rs` `#[test] fn journal_screening_returns_editorial_and_author_outputs_without_ai_claims` passes | default-CI |
| Screening status is `screened_with_warnings` for fixture with unverified reference | Report `status == JournalScreeningStatus::ScreenedWithWarnings` when sidecar has 1 verified and 1 unverified reference (missing DOI) | Fixture `fixtures/journal/arxiv-submit-ce-submission.json` expected `screened_with_warnings`; test assertion passes | default-CI |
| `arxiv_submit_ce` platform string serializes correctly | `JournalPlatform::ArxivSubmitCe` serializes to `"arxiv_submit_ce"` (snake_case) | Serialization roundtrip test: `serde_json::to_string` → `serde_json::from_str` | default-CI |
| `arxiv-submit-ce` platform deserializes from CLI flag | CLI `--platform arxiv-submit-ce` maps to `JournalPlatform::ArxivSubmitCe` | `src/main.rs` platform parsing from `--platform` argument | default-CI |
| Screening report contains `editorial_summary` and `author_action_checklist` | Report has non-empty `editorial_summary` string and `author_action_checklist` vector | Smoke evidence doc `journal-screen-smoke-2026-06-10.md` records report contents | default-CI |
| Report does not assert AI authorship | `editorial_summary` contains "AI-risk citation-error signals" but not "AI-generated" | `#[test] fn journal_screening_returns_editorial_and_author_outputs_without_ai_claims` asserts `!report.editorial_summary.contains("AI-generated")` | default-CI |
| Reference report summary fields populated | `reference_report.summary` contains `total_references`, `verified_references`, `review_queue_count`, `ai_risk_issue_count` | Fixture expected screening report + test assertion on output | default-CI |

## Platform detection — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `arxiv-submit-ce` CLI flag accepted | `--platform arxiv-submit-ce` exits 0 with valid fixture workspace | `cargo run -- journal-screen --platform arxiv-submit-ce --submission-id ARXIV-CE-2026-0001 --manifest source-package.tar.gz .sourceright` exits 0 | default-CI |
| `arxiv_submit_ce` (underscore) alias accepted | `--platform arxiv_submit_ce` maps to same variant | `JournalPlatform::ArxivSubmitCe` parsing from snake_case input | default-CI |
| Unknown platform returns error | `--platform unknown-platform` returns non-zero exit with error message about invalid platform | CLI error handling test | default-CI |

## Upstream issue response — opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Upstream issue filed on `arXiv/submit-ce` | Issue #72 exists and is open on `https://github.com/arXiv/submit-ce/issues/72` | `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/submission-drafts.md` or Track 79 evidence records issue URL and date | opt-in-live |
| Local smoke evidence referenced in upstream comment | Comment on Issue #72 includes link to `journal-screen-smoke-2026-06-10.md` or equivalent evidence | Comment URL recorded in track evidence: `https://github.com/arXiv/submit-ce/issues/72#issuecomment-...` | opt-in-live |
| Maintainer response received (if any) | Issue #72 has response from arXiv maintainer (comment, reaction, or label change) | Screenshot or link to maintainer comment recorded in track evidence | opt-in-live |
| Integration hook proposal is technically accurate | Proposed hook (post-extraction / pre-submit, TeX log pipeline complement) is consistent with `submit-ce` source architecture | Reviewed in `journal-screen-smoke-*.md` and during conductor-review | opt-in-live |

## Integration hook proposal — default CI + opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Proposed hook documented in smoke evidence | `journal-screen-smoke-2026-06-10.md` contains "Proposed upstream hook (submit-ce)" section with 2 integration points | File exists in track directory; section present and reviewed | default-CI |
| Post-extraction hook is read-only only | Proposed hook invokes `journal-screen` as external read-only step; no CSL mutation, no arXiv state mutation | Hook description explicitly states read-only boundary | default-CI |
| Hook proposal distinguishes from TeX log pipeline | Hook description states structured CSL + verification evidence *complements* (not replaces) `tex_filters.py` citation warnings | Hook proposal section in smoke evidence doc | default-CI |
| Upstream issue comment posted with proposal | Comment on `arXiv/submit-ce` Issue #72 contains the integration hook proposal | Comment URL recorded in `journal-screen-smoke-*.md` or separate evidence doc | opt-in-live |

## Evidence recording — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Smoke evidence doc committed | `journal-screen-smoke-2026-06-10.md` exists in track directory with command, result, boundary, and proposed hook | File exists at `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/journal-screen-smoke-2026-06-10.md` | default-CI |
| Fixture JSON is valid against schema | `fixtures/journal/arxiv-submit-ce-submission.json` validates against `schemas/ojs-submission-fixture.json` | Schema validation check (manual or automated) | default-CI |
| Claim boundary documented in all evidence docs | All track evidence docs include disclaimer: fixture-backed, not arXiv-reviewed | Review of all evidence docs | default-CI |
| Live evidence ledger entry (on completion) | `conductor/submission-packets/live-evidence.json` contains submit-ce entry with `status: "fixture_backed"` | `live-evidence.json` readback | opt-in-live |

Default-CI provides the baseline submission gate coverage for this track.
