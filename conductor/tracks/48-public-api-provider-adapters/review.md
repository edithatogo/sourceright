# Track 48 — Public API Provider Adapters — Implementation Review

**Reviewer:** Provider adapter verification specialist + lead
**Date:** 2026-05-14
**Status:** Completed (gap fixed)

## Evidence Checked

### Source Code (`src/live_providers.rs`)
| Function/Item | Status | Lines |
|---|---|---|
| `unpaywall_fixture_result` | ✅ Found | 108–131 |
| `open_citations_fixture_result` | ✅ Found | 133–161 |
| `arxiv_fixture_result` | ✅ Found | 163–203 |
| `europe_pmc_fixture_result` | ✅ Found | 205–243 |
| `smoke_unpaywall` | ✅ Found | 295–324 |
| `smoke_open_citations` | ✅ Found | 326–362 |
| `smoke_arxiv` | ✅ Found | 364–400 |
| `smoke_europe_pmc` | ✅ Found | 402–427 |
| All six smoke functions called in `live_provider_smoke_report` | ✅ Found | 93–101 |

### Enum and Provider Slug (`src/providers.rs`)
| Variant | Slug | Status |
|---|---|---|
| `AcademicProvider::Unpaywall` | `"unpaywall"` | ✅ |
| `AcademicProvider::OpenCitations` | `"opencitations"` | ✅ |
| `AcademicProvider::Arxiv` | `"arxiv"` | ✅ |
| `AcademicProvider::EuropePmc` | `"europe-pmc"` | ✅ |

### Unit Tests (`src/live_providers.rs`)
| Test | Status | Lines |
|---|---|---|
| `unpaywall_fixture_response_records_sidecar_evidence` | ✅ Found | 856–866 |
| `europe_pmc_fixture_response_records_sidecar_evidence` | ✅ Found | 868–878 |
| `arxiv_fixture_response_records_sidecar_evidence` | ✅ Found | 881–890 |
| `open_citations_fixture_response_records_sidecar_evidence` | ✅ Added | 891–901 |
| `repository_records_fixture_response_records_sidecar_evidence` | ✅ Found | 903–913 |
| `licensed_byo_key_fixture_response_records_sidecar_evidence` | ✅ Found | 915–929 |
| `default_smoke_report_skips_without_credentials` | ✅ Found | 781–797 |
| `live_provider_config_defaults_to_conservative_runtime_policy` | ✅ Found | 800–821 |
| `provider_cache_returns_evidence_payload_without_network` | ✅ Found | 824–853 |

### Fixture Files (`fixtures/providers/`)
| File | Status |
|---|---|
| `unpaywall.example.json` | ✅ Found (7 lines, valid JSON) |
| `opencitations.example.json` | ✅ Found (11 lines, valid JSON) |
| `arxiv.example.atom` | ✅ Found (12 lines, valid Atom XML) |
| `europe-pmc.example.json` | ✅ Found (13 lines, valid JSON) |

### Plugin Manifests (`plugins/manifests/`)
| File | Status |
|---|---|
| `provider.unpaywall.toml` | ✅ Found (23 lines) |
| `provider.opencitations.toml` | ✅ Found (23 lines) |
| `provider.arxiv.toml` | ✅ Found (23 lines) |
| `provider.europepmc.toml` | ✅ Found (23 lines) |

## Gap Fixed

The original review identified a missing OpenCitations fixture test. The test `open_citations_fixture_response_records_sidecar_evidence` has been added at line 891 in `src/live_providers.rs`. It invokes `open_citations_fixture_result` against `opencitations.example.json` and asserts the correct `AcademicProvider::OpenCitations` variant and `"opencitations"` candidate slug.

## Summary

All four public API providers (Unpaywall, OpenCitations, arXiv, Europe PMC) have complete implementations with:
- Fixture result parsing functions
- Live smoke endpoints
- Fixture test files
- Plugin manifests
- AcademicProvider enum variants and slugs
- Unit tests for fixture parsing
- Sidecar-only evidence writes (no CSL mutation)
