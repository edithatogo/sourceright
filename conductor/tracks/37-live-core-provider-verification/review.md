# Live Core Provider Verification Review

## Completion Review — 2026-05-14

### Scope Reviewed

All 6 test-matrix scenarios were inspected for unit-test coverage:

1. ✅ **Crossref DOI match** — `crossref_doi_lookup_records_candidate_and_confidence_inputs` in `src/providers.rs`
2. ✅ **DataCite dataset match** — `datacite_dataset_response_is_normalized_as_provider_candidate` in `src/providers.rs`
3. ✅ **OpenAlex title/DOI match** — `openalex_work_response_uses_doi_or_display_name_for_confidence` and `matched_identifier_with_conflicting_title_is_ambiguous`
4. ✅ **PubMed/NCBI record** — `pubmed_record_response_preserves_pmid_payload` in `src/providers.rs`
5. ✅ **DOI resolver reachable** — `doi_resolution_records_reachability_without_bibliographic_overwrite` in `src/providers.rs`
6. ✅ **Provider no-match/outage** — `conflicting_identifier_and_title_payload_is_no_match_but_keeps_sidecar_candidate` and `provider_result_diagnostic_classifies_outage_style_errors`

### Missing Coverage

No missing coverage was found. All 6 scenarios have adequate fixture-backed unit tests.

### Residual Risk

- Live provider network tests remain opt-in and skip by default (controlled via `SOURCERIGHT_LIVE_PROVIDERS` and `SOURCERIGHT_LIVE_PROVIDER_SMOKE` env vars).
- Provider response caching is tested in `provider_cache_returns_evidence_payload_without_network` but real rate-limit/retry behavior can only be validated with live endpoints.
