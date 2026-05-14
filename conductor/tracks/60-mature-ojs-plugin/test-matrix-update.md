# Track 60 — OJS Fixture Test Matrix Update

> Generated: 2026-05-14
> Updates: `conductor/tracks/60-mature-ojs-plugin/test-matrix.md`
> Fixture source: `fixtures/journal/ojs-submission.json`

---

## 1. Fixture Overview

The OJS submission fixture (`fixtures/journal/ojs-submission.json`) covers 5 references
with distinct integrity states. Each reference exercises a specific branch of the
screening pipeline defined in `src/journal.rs` -> `screen_journal_submission()`.

### Reference States in Fixture

| Reference ID | DOI | Integrity State | Severity |
|---|---|---|---|
| `martinez-2025-trial` | 10.1234/example-2025-001 | ✅ Verified (high confidence) | none |
| `phelps-2023-hallucination` | 10.5678/example-2023-089 | ⚠️ Provider conflict (title mismatch) | warning |
| `zhang-2024-retracted` | 10.9012/retracted-2024-001 | ⛔ Retracted per Crossref | error |
| `thompson-2022-foundations` | 10.1111/books-2022-001 | 🔍 Queued for manual review | info |
| `kumar-2025-no-doi` | (missing) | ❓ Unverified, no identifier | warning |

## 2. Test Scenarios & Acceptance Criteria

### Scenario A: Submission Intake

**Fixture input**: The `submission`, `manuscript`, and `csl_references` fields.

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| A1 | `submission_id` is propagated from fixture to screening request | Load fixture, assert `request.submission_id == "OJS-SUB-2025-0042"` | Exact match |
| A2 | `platform` is serialized as `"ojs"` | Assert `request.platform == JournalPlatform::Ojs` | OJS variant |
| A3 | All 5 CSL references are extracted without error | Assert `request.csl_references.len() == 5` | Full fixture |
| A4 | Author names are correctly parsed from JSON into CSL `author` format | Assert `martinez-2025-trial` has 2 authors (Martinez, Chen) | Author parsing |

### Scenario B: Verified Reference (martinez-2025-trial)

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| B1 | No issues are raised for verified reference | Assert no issue with `reference_id == "martinez-2025-trial"` | Happy path |
| B2 | Provider confidence (0.97) does not trigger conflict | Assert `conflict_count == 1` (only the Phelps conflict) | Threshold |
| B3 | Review status `"not_required"` produces no queue entry | Assert `review_queue_count == 1` (only Thompson queued) | Status routing |

### Scenario C: Provider Conflict (phelps-2023-hallucination)

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| C1 | Conflict between Crossref and OpenAlex titles is detected | Assert issue with `code == "report.provider_conflict"` and `reference_id == "phelps-2023-hallucination"` | Conflict detection |
| C2 | Conflict is surfaced as `severity: "warning"` | Assert issue severity is `"warning"` | Severity mapping |
| C3 | Conflict is flagged as AI-risk signal | Assert `ai_risk_signal == true` | Risk classification |
| C4 | OpenAlex title variant is included in issue message | Assert message contains provider names | Transparency |

### Scenario D: Retracted Reference (zhang-2024-retracted)

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| D1 | Retraction status from Crossref is surfaced as error | Assert issue with `code == "policy.recency.provider.retraction"` | Retraction signal |
| D2 | Retraction notice DOI is referenced in issue | Assert message references `"10.9012/retraction-2025-001"` | Evidence linking |
| D3 | Publication year mismatch (2018 vs. 2024) does not produce separate issue | Assert exactly 1 issue for this reference | Noise reduction |

### Scenario E: Queued for Review (thompson-2022-foundations)

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| E1 | Reference with no provider candidates and status `"queued"` produces info issue | Assert issue with `code == "report.manual_review_needed"` | Review queue |
| E2 | Issue severity is `"info"` (not `"warning"`) | Assert issue severity is `"info"` | Severity grading |
| E3 | `ai_risk_signal` is `false` for queued manual review | Assert `ai_risk_signal == false` | Risk classification |


### Scenario F: Missing Identifier (kumar-2025-no-doi)

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| F1 | Missing DOI produces `code == "report.missing_doi"` | Assert issue with `code == "report.missing_doi"` | Missing field |
| F2 | Missing provider verification produces `code == "report.unverified_reference"` | Assert issue with `code == "report.unverified_reference"` | Missing verification |
| F3 | Both issues are `severity: "warning"` and `ai_risk_signal: true` | Assert both issues have warning severity + AI risk flag | Risk classification |
| F4 | No DOI produces no attempt at external provider lookup | Assert no provider_candidates for this reference | Boundary |

### Scenario G: Summary Report Integrity

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| G1 | `total_references` matches fixture reference count (5) | Assert `summary.total_references == 5` | Counting |
| G2 | `verified_references` counts references without report warnings or errors | Assert `summary.verified_references == 3` | Counting |
| G3 | `review_queue_count` matches queued references (1) | Assert `summary.review_queue_count == 1` | Counting |
| G4 | `conflict_count` matches conflict references (1) | Assert `summary.conflict_count == 1` | Counting |
| G5 | `warning_count` matches warning issues (3) | Assert `summary.warning_count == 3` | Counting |
| G6 | `info_count` matches info issues (1) | Assert `summary.info_count == 1` | Counting |
| G7 | `error_count` matches retraction issues (1) | Assert `summary.error_count == 1` | Boundary |

### Scenario H: Editorial & Author Outputs

| # | Acceptance Criterion | Verification Method | Coverage |
|---|---------------------|---------------------|----------|
| H1 | `editorial_summary` contains status `screened_with_errors` | Assert `status == ScreenedWithErrors` | Status enum |
| H2 | `editorial_summary` includes total reference count | Assert string contains `"5 references"` | Content |
| H3 | `editorial_summary` includes warning count | Assert string contains `"5 issues"` | Content |
| H4 | `editorial_summary` includes AI-risk signal count | Assert string contains `"4 AI-risk"` | Content |
| H5 | `author_action_checklist` contains at least 2 items | Assert `checklist.len() >= 2` | Completeness |
| H6 | `author_action_checklist` references missing identifiers | Assert any item contains `"identifiers"` | Actionability |
| H7 | `author_action_checklist` references manual review queue | Assert any item contains `"queued"` or `"manual review"` | Actionability |


## 3. Priority Matrix

| Priority | Scenarios | Rationale |
|----------|-----------|-----------|
| **P0 — Critical** | A1-A3, G1-G7, H1 | Core contract: intake, counting, status assignment |
| **P1 — High** | B1-B3, C1-C4, F1-F4 | Individual reference integrity states |
| **P2 — Medium** | D1-D3, E1-E3 | Edge cases: retraction, review queue |
| **P3 — Low** | A4, H2-H7 | Content-level assertions |

## 4. Automation Readiness

| Criterion | Assessment |
|-----------|------------|
| **Fixture loading** | ✅ `ojs-submission.json` is self-contained with `$schema`, all required fields |
| **Deterministic** | ✅ No randomness; all expected states are hardcoded |
| **Schema validation** | ✅ `$schema` references `../../schemas/ojs-submission-fixture.json` |
| **Rust test framework** | ✅ `src/journal.rs` has `screen_journal_submission()` — tests can deserialize fixture and assert |
| **No live dependency** | ✅ Fixture includes inline `verification_sidecar` — no provider calls needed |
| **CI-compatible** | ✅ No external services; runs entirely from fixture data |

**Recommendation**: Automate scenarios A1-A3, G1-G7, and H1 as P0 integration tests
in `tests/journal_fixture_contract.rs`. Add P1 scenarios after core contract is stable.
Defer P2 and P3 to subsequent iterations.

## 5. Matrix Update Record

| Date | Change | Author |
|------|--------|--------|
| 2026-05-14 | Initial test matrix update mapping ojs-submission.json scenarios | codex |
