# CourtListener Legal Provider Plan

## Current State (reconciled 2026-05-17)

### What exists
- **Legal citation model**: Full implementation in `src/legal.rs` with `LegalCitationReport`, `LegalCitationRecord`, `LegalCitationType`, `LegalProvider`, `LegalProviderCandidate`, and `LegalCitationIssue` types.
- **CourtListener as enum variant**: `LegalProvider::CourtListener` is defined in the `LegalProvider` enum (line 67), alongside `CaselawAccessProject`, `Austlii`, `LegislationRegister`, and `Other`.
- **CourtListener provider candidates**: `provider_candidates_for_court()` now emits `LegalProvider::CourtListener` for United States court hints such as `US`, `SCOTUS`, and `USCA*`; Australian court hints still route to AustLII.
- **Plugin manifest**: `plugins/manifests/legal.courtlistener.toml` exists with status `planned_public_api`, reads/writes `sourceright.legal_citation_report`.
- **Provider fixtures**: CourtListener fixture files exist at `fixtures/providers/courtlistener/success.json` and `fixtures/providers/courtlistener/no-match.json`.
- **CourtListener tests**: Inline Rust tests in `src/legal.rs` exercise US provider assignment and fixture-backed candidate parsing.

### Gap analysis

| Requirement | Status |
|------------|--------|
| Enum variant defined | ✅ `LegalProvider::CourtListener` exists |
| Plugin manifest | ✅ `legal.courtlistener.toml` exists |
| Identifier / jurisdiction mapping | ✅ US court hints map to CourtListener candidates |
| Fixture-backed test cases | ✅ Present for success and no-match payloads |
| Opt-in live smoke | ❌ Missing |
| US court provider candidates | ✅ Implemented for SCOTUS/USCA-style hints |
| Legal roadmap updated | ✅ Documents CourtListener as the first public US provider path |

## Implementation Plan

### Slice 1: Mapping and provider candidates
1. [x] Add `provider_candidates_for_court()` entries for US federal courts (SCOTUS, US Circuit, US District).
2. [x] Map `jurisdiction_for_court()` for common US court abbreviations.
3. [x] Emit `LegalProvider::CourtListener` as candidate for US courts with appropriate confidence.

### Slice 2: Fixtures
4. [x] Create `fixtures/providers/courtlistener/` directory.
5. [x] Add fixture files:
   - `success.json` — known US case resolved via CL API.
   - `no-match.json` — citation that yields no CL result.
   - `ambiguous.json` — citation matching multiple CL records (deferred to live-adapter maturity).
   - `api-error.json` — simulated rate-limit or auth failure (deferred to live-adapter maturity).
6. [x] Add deterministic Rust test cases using fixture data.

### Slice 3: Live smoke
7. [ ] Add opt-in live smoke (`COURTLISTENER_API_KEY` env gate).
8. [ ] Add rate-limit awareness and cache headers.

### Slice 4: Documentation
9. [x] Update `docs/src/legal-roadmap.md` with CourtListener status.
10. [ ] Keep plugin manifest status at `planned_public_api` until the live-provider adapter path and opt-in smoke are stable; fixture-backed helper evidence is recorded separately.

### Slice 5: Review
11. [x] Run `cargo test` on legal module.
12. [x] Run `$conductor-review`.
13. [x] Apply local fixes; defer any legal advice/compliance claims.

### Slice 6: Claude For Legal Compatibility
14. Treat Claude-for-Legal as connector/workflow design reference only, not a
    code dependency.
15. Add a legal citation audit MCP pack that explains Sourceright's narrow
    connector role.
16. Tighten MCP descriptions so `legal.analyze_citations` is framed as draft
    citation audit evidence for attorney review.
17. Preserve licensed-provider separation: Westlaw/Practical Law-style systems
    remain external subscriber-provided research connectors.
