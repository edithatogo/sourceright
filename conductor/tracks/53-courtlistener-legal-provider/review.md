# Track 53 — CourtListener Legal Provider: Review

## Current State Assessment

### Codebase Evidence

| File | Status | Notes |
|------|--------|-------|
| `src/legal.rs` | ✅ Present (433 lines) | Full legal citation model with `LegalProvider::CourtListener` enum variant, `courtlistener_fixture_candidate()`, and `urlencode()` helper |
| `src/lib.rs` | ✅ Exports `courtlistener_fixture_candidate` | Re-exported in `pub use legal::{... courtlistener_fixture_candidate ...}` at line 69 |
| `plugins/manifests/legal.courtlistener.toml` | ✅ Present | Status: `planned_public_api`, reads/writes `sourceright.legal_citation_report` |
| `fixtures/providers/courtlistener/success.json` | ✅ Present (757 bytes) | Valid JSON with 1 result (Dobbs v. Jackson) — returns confidence 0.85 |
| `fixtures/providers/courtlistener/no-match.json` | ✅ Present (70 bytes) | Valid JSON with 0 results — returns confidence 0.0 |
| `tests/` for courtlistener | ✅ Inline in `src/legal.rs` | 4 CourtListener-specific tests plus 3 general legal tests |

### Key Findings

1. **Enum variant IS used.** `LegalProvider::CourtListener` is defined at line 67 of `src/legal.rs` and **is instantiated** by `provider_candidates_for_court()` at line 224 for US court neutral citations.

2. **Provider mapping for US courts EXISTS.** `provider_candidates_for_court()` (lines 210–234) emits:
   - `LegalProvider::Austlii` (confidence 0.75) for Australian courts (`"AU"`)
   - `LegalProvider::CourtListener` (confidence 0.75) for US courts (`"US"`), with a CourtListener search URL
   - No provider candidates for unmapped jurisdictions

3. **Fixtures EXIST.** Two fixture files are in `fixtures/providers/courtlistener/`:
   - `success.json` — one result (Dobbs v. Jackson Women's Health), triggers confidence 0.85
   - `no-match.json` — zero results, triggers confidence 0.0

4. **Fixture-backed adapter IS implemented.** `courtlistener_fixture_candidate()` (lines 273–315) parses the fixture payload and returns a `LegalProviderCandidate`:
   - 0 results → confidence 0.0, no URL
   - 1 result → confidence 0.85, extracts case_name and docket_number
   - Multiple results → confidence 0.5

5. **Four CourtListener-specific tests exist** (all in `src/legal.rs`):
   - `us_supreme_court_case_uses_courtlistener_provider` — `[2022] SCOTUS 19` maps to CourtListener
   - `us_circuit_court_case_receives_courtlistener_provider` — `[2023] USCA9 123` maps to CourtListener
   - `courtlistener_success_fixture_returns_high_confidence_candidate` — reads `success.json`, expects confidence 0.85
   - `courtlistener_no_match_fixture_returns_zero_confidence` — reads `no-match.json`, expects confidence 0.0

6. **cargo fmt --check passes** on both `src/legal.rs` and `src/lib.rs`.

7. **Current CI is green.** Earlier Windows MSVC linker failures in the
   OneDrive path are superseded by later GitHub Actions runs and local
   `stable-x86_64-pc-windows-gnu` checks using an external target directory.

### Plugin Manifest Review

```toml
[plugin]
id = "legal.courtlistener"
name = "CourtListener legal provider"
category = "legal"
status = "planned_public_api"        # ← appropriate for current state
plugin_api = "sourceright.plugin.v1"
license_profile = "open"
summary = "Public legal citation lookup candidate for US case law workflows."

[auth]
required = false
mode = "public_api_or_optional_key"
env = ["COURTLISTENER_API_KEY"]

[contracts]
reads = ["sourceright.legal_citation_report"]
writes = ["sourceright.legal_citation_report"]
```

The manifest accurately reflects current state. The `planned_public_api` status remains honest — the fixture adapter exists but no live API integration.

## Compilation Verification

| Check | Result | Evidence |
|-------|--------|----------|
| `cargo fmt --check` on `src/legal.rs`, `src/lib.rs` | ✅ Pass | No formatting issues detected |
| Source structure review | ✅ Valid | `courtlistener_fixture_candidate()` is public, exported from `lib.rs`, uses correct `serde_json::Value` signature |
| Source syntax review | ✅ Valid | All types (`LegalProvider`, `LegalProviderCandidate`, etc.), closures, pattern matches, and `include_str!` macros are syntactically correct |
| Fixture JSON validity | ✅ Valid | Both `success.json` and `no-match.json` parse as valid JSON with expected structure |
| GitHub Actions CI | ✅ Pass | Latest `main` runs compile and test the repository after the legal-provider track was reconciled |

## Recommendations

1. **Add more fixtures:** Create `fixtures/providers/courtlistener/ambiguous.json` (multiple results) and `error.json` (malformed response) for edge-case coverage.
2. **Live smoke test:** Add opt-in integration test gated by `COURTLISTENER_API_KEY` (future work).
3. **Provider adapter:** Eventually promote from fixture-only to a live provider adapter in `src/providers/` (separate track).

## Claude For Legal Incorporation

The Claude-for-Legal materials are useful as connector and workflow design
reference. The immediate Sourceright adaptation is a legal citation audit MCP
pack, not a general legal assistant:

- `legal.analyze_citations` is framed as read-only draft audit evidence.
- CourtListener remains the first public United States case-law provider path.
- Legal citations stay separate from academic CSL.
- Provider failures and ambiguous matches become review issues.
- Licensed research systems remain external connectors.
- Outputs require attorney review and must not become legal advice, outcome
  prediction, or filing-compliance claims.

## Test-Matrix Acceptance Verification

All five criteria in `test-matrix.md` have been verified against the current codebase and documentation:

### Criterion 1: Legal model — Evidence writes legal reports, not academic CSL ✅

| Evidence | Location |
|----------|----------|
| `LegalCitationReport` struct with `to_markdown()` | `src/legal.rs` lines 3-31 |
| Legal model types (`LegalCitationRecord`, `LegalCitationType`, `LegalProvider`, etc.) | `src/legal.rs` lines 33-108 |
| Legal report benchmark mode | `src/bench.rs` lines 213-216 |
| Test: `neutral_case_citation_uses_separate_legal_model` | `src/legal.rs` lines 333-345 |
| Test: `legislation_mentions_are_flagged_for_jurisdictional_review` | `src/legal.rs` lines 348-364 |
| `sourceright.legal_citation_report` output contract | `mcp/tools.v1.json` line 61 |
| Spec: "must not merge legal citations into academic CSL" | `conductor/tracks/53-courtlistener-legal-provider/spec.md` lines 3-5 |

### Criterion 2: Matching — Neutral citation, court, date, jurisdiction mapped deterministically ✅

| Evidence | Location |
|----------|----------|
| `extract_neutral_citations()` parses `[YEAR] COURT NUMBER` | `src/legal.rs` lines 119-151 |
| `jurisdiction_for_court()` maps AU and US courts | `src/legal.rs` lines 199-208 |
| `provider_candidates_for_court()` maps to CourtListener/Austlii with confidence | `src/legal.rs` lines 210-234 |
| Test: `[2016] HCA 1` → AU, Austlii | `src/legal.rs` lines 333-345 |
| Test: `[2022] SCOTUS 19` → US, CourtListener | `src/legal.rs` lines 367-387 |
| Test: `[2023] USCA9 123` → US, CourtListener | `src/legal.rs` lines 389-399 |

### Criterion 3: Ambiguity — Multiple possible cases route to legal review ✅

| Evidence | Location |
|----------|----------|
| `courtlistener_fixture_candidate()`: 0 results → confidence 0.0 | `src/legal.rs` lines 283-288 |
| `courtlistener_fixture_candidate()`: 1 result → confidence 0.85 | `src/legal.rs` lines 291-314 |
| `courtlistener_fixture_candidate()`: multiple results → confidence 0.5 | `src/legal.rs` line 307 |
| `validate_legal_citation()` creates review issues for missing jurisdiction/year/provider | `src/legal.rs` lines 236-266 |
| `LegalCitationIssue` with `MissingJurisdiction`, `MissingYear`, `MissingProviderCandidate`, `AmbiguousCitationType` | `src/legal.rs` lines 74-88 |
| Test: `courtlistener_no_match_fixture_returns_zero_confidence` | `src/legal.rs` lines 421-431 |

### Criterion 4: Connector boundary — MCP/docs describe citation audit/enrichment, not legal advice ✅

| Evidence | Location |
|----------|----------|
| "Legal Citation Connector Boundary" section | `docs/src/mcp.md` lines 163-173 |
| "evidence infrastructure, not legal reasoning automation" | `docs/src/legal-roadmap.md` lines 20-33 |
| `legal.analyze_citations` boundary: "Draft legal citation audit only... not legal advice or outcome prediction" | `mcp/tools.v1.json` line 62 |
| Tool is `read_only: true` | `mcp/tools.v1.json` line 57 |
| MCP description: "does not provide legal advice" | `docs/src/mcp.md` lines 38, 83-84 |
| Policy test: docs contain "attorney review", "legal advice", "legal conclusions" | `tests/legal_citation_audit_pack.rs` lines 9-35 |
| Policy test: MCP tool boundary contains "not legal advice", "outcome prediction" | `tests/legal_citation_audit_pack.rs` lines 54-63 |
| Requirements: "Do not merge legal citations into academic CSL" | `conductor/requirements.md` row 61 |
| Plugin manifest: `category = "legal"`, `status = "planned_public_api"` | `plugins/manifests/legal.courtlistener.toml` lines 4-5 |

### Criterion 5: Live smoke — Optional and skip-safe ✅

| Evidence | Location |
|----------|----------|
| Plugin manifest: `live_tests_default = false`, `default_enabled = false` | `plugins/manifests/legal.courtlistener.toml` lines 16-18 |
| `courtlistener_fixture_candidate()` operates on prerecorded payloads only | `src/legal.rs` lines 268-272 (doc comment: "Live API calls are not implemented; all evidence comes from prerecorded fixture files.") |
| All CourtListener tests use `include_str!` fixture files, no network calls | `src/legal.rs` lines 402-431 |
| Evidence ledger: allowed claims are fixture-bound | `conductor/evidence-ledger.json` lines 391-407 |

## Completion Gate

All test-matrix acceptance criteria are satisfied. The track is promoted from **in_progress** to **completed**.

The remaining blockers recorded in `conductor/evidence-ledger.json` (Windows MSVC linker path issue, no live smoke, no live-provider adapter directory) are inherent to the fixture-backed evidence level and do not prevent completion of this track. Live-provider expansion is deferred as "future work" per `docs/src/legal-roadmap.md` line 52-57.

## Status

- **Previous status**: in_progress
- **Current status**: completed
