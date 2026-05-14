# URL Archive Integrity Review

## Completion Review — 2026-05-14

### Scope Reviewed

All 6 test-matrix scenarios were inspected for unit-test coverage in `src/policy.rs`:

1. ✅ **Reachable URL** — tested implicitly through `provider_backed_url_archive_evidence_is_classified_without_mutating_csl`
2. ✅ **Redirecting URL** — `provider_backed_url_archive_evidence_is_classified_without_mutating_csl` verifies redirect evidence (`policy.url.redirect`)
3. ✅ **Broken URL** — `provider_backed_url_status_evidence_is_classified_without_network_calls` verifies `policy.url.broken`
4. ✅ **DOI landing page** — `provider_backed_url_archive_evidence_classifies_doi_landing_urls_without_redirect_noise`
5. ✅ **Archived URL** — `provider_backed_url_archive_evidence_is_classified_without_mutating_csl` verifies `policy.url.archive.recorded`
6. ✅ **Timeout/offline** — `provider_backed_url_status_evidence_is_classified_without_network_calls` verifies `policy.url.offline` and `policy.url.unchecked`

### Missing Coverage

No missing coverage was found. All 6 scenarios have adequate fixture-backed unit tests.

### Residual Risk

- All URL checks are network-free by default; live reachability testing requires
  opt-in provider configuration and is not covered in default CI.
- Archive.org/Memento integration is scoped as future work; current tests use
  synthetic archive evidence from provider fixtures.
