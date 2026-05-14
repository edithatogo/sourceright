# OJS Journal Screening Proof — External Proof Suite

## Purpose

This document defines how to verify the Open Journal Systems (OJS) journal
screening integration. It serves as a **human-readable proof transcript** and a
**verification checklist** for the OJS Proof family.

## Prerequisites

- A built `sourceright` binary on `$PATH`, or accessible via `cargo run --bin sourceright`
- The repository root containing `fixtures/journal/ojs-submission.json`
- A workspace directory for running the journal screening pipeline
- `jq` for JSON formatting (optional, for readability)


## Proof Sections

### 1. Validate the OJS Submission Fixture

**Purpose:** Confirm the fixture file is well-formed and parseable.

**Command:**
```text
cat fixtures/journal/ojs-submission.json | jq '.fixture_name'
```

**Expected output:**
```
"ojs-submission-synthetic-001"
```

**Key assertions:**
- `fixture_name` is `ojs-submission-synthetic-001`
- `submission.submission_id` is `OJS-SUB-2025-0042`
- `submission.platform` is `ojs`
- `csl_references` contains 5 references
- `verification_sidecar.references` has entries for all 5 reference IDs

**Exit code:** `0`

---

### 2. Initialise a Workspace with the OJS Fixture CSL

**Purpose:** Create a `.sourceright` workspace with the CSL references from the
OJS submission fixture, preparing for the screening pipeline.

**Command:**
```text
sourceright init
```

**Then copy CSL from fixture:**
```text
jq '.csl_references' fixtures/journal/ojs-submission.json > .sourceright/references.csl.json
```

**Command (validate):**
```text
sourceright validate-csl .sourceright/references.csl.json
```

**Expected output:** `valid`

**Exit code:** `0`

---

### 3. Run the Journal Screening Pipeline

**Purpose:** Exercise the journal screening pipeline against the OJS submission
fixture and verify the output matches the expected screening report.

**Command:**
```text
sourceright journal-screen --submission-id OJS-SUB-2025-0042 --platform ojs --json .sourceright
```

**Expected JSON output (abridged):**
```json
{
  "schema_version": "sourceright.journal_screening.v1",
  "submission_id": "OJS-SUB-2025-0042",
  "platform": "ojs",
  "status": "screened_with_warnings",
  "editorial_summary": "...",
  "author_action_checklist": ["...", "..."],
  "reference_report": {
    "schema_version": "sourceright.reference_report.v1",
    "report_type": "reference_integrity",
    "summary": {
      "total_references": 5,
      "verified_references": 2,
      "review_queue_count": 1,
      "unresolved_count": 0,
      "conflict_count": 1,
      "ai_risk_issue_count": 4,
      "error_count": 0,

---

### 4. Verify Screening Report Matches the Fixture's Expected Output

**Purpose:** Confirm that the screening report output structure aligns with the
`expected_screening_report` embedded in the OJS fixture.

**Command (extract expected from fixture):**
```text
jq '.expected_screening_report' fixtures/journal/ojs-submission.json
```

**Expected output (abridged):**
```json
{
  "schema_version": "sourceright.journal_screening.v1",
  "submission_id": "OJS-SUB-2025-0042",
  "platform": "ojs",
  "status": "screened_with_warnings",
  "editorial_summary": "Submission OJS-SUB-2025-0042 (manuscript-v3-final.docx) screened with status ScreenedWithWarnings: 5 references, 5 issues, 4 AI-risk citation-error signals.",
  "author_action_checklist": [
    "Review missing identifiers, unverified references, and provider conflicts.",
    "Resolve references currently queued for manual review."
  ]
}
```

**Key assertions against the expected report:**
- `status` equals `screened_with_warnings`
- `reference_report.summary.total_references` is `5`
- `reference_report.summary.ai_risk_issue_count` is `4`
- Issues include:
  - `report.missing_doi` for `kumar-2025-no-doi` (severity: `warning`, ai_risk_signal: `true`)
  - `report.unverified_reference` for `kumar-2025-no-doi` (severity: `warning`, ai_risk_signal: `true`)
  - `report.provider_conflict` for `phelps-2023-hallucination` (severity: `warning`, ai_risk_signal: `true`)
  - `policy.recency.provider.retraction` for `zhang-2024-retracted` (severity: `warning`, ai_risk_signal: `true`)
  - `report.manual_review_needed` for `thompson-2022-foundations` (severity: `info`, ai_risk_signal: `false`)

---

### 5. MCP Tool: `journal.screen_submission`

**Purpose:** Verify the OJS screening surface is exposed as an MCP tool.

**Command:**
```text
sourceright mcp tools --json | jq '.tools[] | select(.name == "journal.screen_submission")'
```

**Expected output:**
```json
{
  "name": "journal.screen_submission",
  "read_only": true
}
```

**Key assertions:**
- The tool `journal.screen_submission` exists
- It is marked `read_only: true`

**Exit code:** `0`

---

### 6. MCP Resource: `sourceright://reports/journal-screening`

**Command:**
```text
sourceright mcp resources --json | jq '.resources[] | select(.uri == "sourceright://reports/journal-screening")'
```

**Expected output:**
```json
{
  "uri": "sourceright://reports/journal-screening",
  "mime_type": "application/json"
}
```

**Exit code:** `0`

## Transcript Template

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== OJS Proof Transcript ==="
WORKSPACE=$(mktemp -d)
cd "$WORKSPACE"

echo "1. Init workspace"
sourceright init
test -d .sourceright || { echo "FAIL: .sourceright not created"; exit 1; }

echo "2. Load OJS fixture CSL into workspace"
REPO_ROOT="${1:-.}"
jq '.csl_references' "$REPO_ROOT/fixtures/journal/ojs-submission.json" > .sourceright/references.csl.json
sourceright validate-csl .sourceright/references.csl.json | grep -q "valid"

echo "3. Run journal screening"
SCREENING=$(sourceright journal-screen --submission-id OJS-SUB-2025-0042 --platform ojs --json .sourceright)
echo "$SCREENING" | jq -e '.schema_version == "sourceright.journal_screening.v1"' > /dev/null
echo "$SCREENING" | jq -e '.platform == "ojs"' > /dev/null
echo "$SCREENING" | jq -e '.status == "screened_with_warnings"' > /dev/null
echo "$SCREENING" | jq -e '.reference_report.summary.total_references == 5' > /dev/null
echo "$SCREENING" | jq -e '.reference_report.summary.ai_risk_issue_count == 4' > /dev/null

echo "4. Verify MCP tool surface"
sourceright mcp tools --json | jq -e '.tools[] | select(.name == "journal.screen_submission")' > /dev/null

echo "5. Verify MCP resource surface"
sourceright mcp resources --json | jq -e '.resources[] | select(.uri == "sourceright://reports/journal-screening")' > /dev/null

echo "=== ALL PASSED ==="
rm -rf "$WORKSPACE"
```

## Overclaim Guard

The OJS proof must not claim:

- "Live OJS instance integration" — the proof uses a synthetic fixture, not a live OJS server.
- "OJS plugin deployed" — the `journal.ojs` plugin has status `planned_adapter`; no OJS plugin binary exists.
- "All platforms supported" — the fixture is OJS-specific; ScholarOne, Editorial Manager, eJournalPress, and Manuscript Manager are schema-enumerated but unproven.

Only claim: **"Fixture-backed OJS screening pipeline produces correct
`sourceright.journal_screening.v1` output for a synthetic submission with 5 CSL
references, 4 AI-risk signals, 1 retraction flag, 1 provider conflict, and 1
manual-review-queued reference."**

## Proof Family Status

| Surface | Fixture | CI Coverage | Transcript Evidence | Status |
|---------|---------|-------------|-------------------|--------|
| OJS submission fixture | `fixtures/journal/ojs-submission.json` | ❌ Not yet in CI | ✅ This document | 🔶 Partial |
| `journal-screen` CLI | N/A (uses workspace) | ❌ Not yet in CI | ✅ This document | 🔶 Partial |
| `journal.screen_submission` MCP tool | N/A | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `sourceright://reports/journal-screening` resource | N/A | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| Live OJS instance smoke | N/A | N/A (opt-in only) | 🔄 Opt-in | ⏸️ Deferred |

The MCP discovery surfaces (tool + resource) are proven via CI. The fixture-backed
screening pipeline is documented but not yet automated in CI. Live OJS smoke is
deferred pending a disposable OJS test instance.
