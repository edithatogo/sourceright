# Track 53 — CourtListener Legal Provider: Review

## Current State Assessment

### Codebase Evidence

| File | Status | Notes |
|------|--------|-------|
| `src/legal.rs` | ✅ Present (293 lines) | Full legal citation model with `LegalProvider::CourtListener` enum variant |
| `plugins/manifests/legal.courtlistener.toml` | ✅ Present | Status: `planned_public_api`, reads/writes `sourceright.legal_citation_report` |
| `provider-fixtures/courtlistener/` | ❌ Missing | No fixture directory exists |
| `fixtures/` for courtlistener | ❌ Missing | None found |
| `tests/` for courtlistener | ❌ Missing | No test file for CourtListener |
| `docs/src/legal-roadmap.md` | ✅ Present (842 bytes) | Needs verification of CourtListener content |

### Key Findings

1. **Enum variant exists but is unused.** `LegalProvider::CourtListener` is defined in `src/legal.rs` line 67, but `provider_candidates_for_court()` (line 206–221) only emits `LegalProvider::Austlii` candidates. CourtListener is never instantiated.

2. **No provider mapping for US courts.** `jurisdiction_for_court()` maps `"HCA"`, `"FCA"`, `"FCAFC"`, `"NSWCA"`, `"VSCA"`, `"QCA"`, `"SASC"`, `"WASCA"`, `"TASSC"`, `"NTSC"`, `"ACTSC"` to `"AU"`, and `"US"` / `"SCOTUS"` to `"US"`. But no provider candidates are emitted for US courts.

3. **No fixtures or tests.** The `provider-fixtures/` directory has subdirs for `arxiv`, `crossmark-status`, `europepmc`, `opencitations`, `repositories`, `unpaywall` — but not `courtlistener`.

4. **Existing legal tests pass.** The two unit tests in `src/legal.rs` (`neutral_case_citation_uses_separate_legal_model` and `legislation_mentions_are_flagged_for_jurisdictional_review`) both pass.

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

The manifest accurately reflects current state. The `planned_public_api` status is honest.

## Recommendations

1. **Immediate (this sprint):** Add US court provider candidates in `provider_candidates_for_court()` and `jurisdiction_for_court()`.
2. **Next:** Create `provider-fixtures/courtlistener/` with success, no-match, ambiguous, and error cases.
3. **Next:** Add Rust test cases that exercise CourtListener as a fixture-backed provider.
4. **Later:** Add opt-in live smoke test gated by `COURTLISTENER_API_KEY`.
5. **Track dependency:** Blocked on `13-legal-citations` (complete) and `46-plugin-and-provider-roadmap-delivery`.

## Status

- **Previous status**: planned
- **New status**: planned (implementation deferred — foundation exists but provider logic is incomplete)
