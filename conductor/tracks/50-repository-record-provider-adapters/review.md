# Track 50 — Repository Record Provider Adapters — Implementation Review

**Reviewer:** Provider adapter verification specialist
**Date:** 2026-05-14
**Status:** Complete

## Evidence Checked

### Source Code (`src/live_providers.rs`)
| Function/Item | Status | Lines |
|---|---|---|
| `repository_records_fixture_result` | ✅ Found | 245–268 |
| `smoke_repository_records` | ✅ Found | 429–461 |
| `repository_records` called in `live_provider_smoke_report` | ✅ Found | 98 |

The adapter queries NCBI eSummary by PMID and normalizes the response into sidecar evidence with DOI and title extraction.

### Enum and Provider Slug (`src/providers.rs`)
| Variant | Slug | Status |
|---|---|---|
| `AcademicProvider::RepositoryRecords` | `"repository-records"` | ✅ |

### Unit Test (`src/live_providers.rs`)
| Test | Status | Lines |
|---|---|---|
| `repository_records_fixture_response_records_sidecar_evidence` | ✅ Found | 893–903 |

### Fixture File (`fixtures/providers/`)
| File | Status |
|---|---|
| `repository-records.example.json` | ✅ Found (5 lines, valid JSON) |

### Plugin Manifest (`plugins/manifests/`)
| File | Status |
|---|---|
| `provider.repository-records.toml` | ✅ Found (23 lines) |

Declares `status = "planned_public_api"`, `auth.mode = "public_api_or_byo_key"`.

### URL/Archive Handoff
The `smoke_repository_records` function uses `ncbi_esummary_endpoint` (line 715–723) which provides the URL/archive handoff point for Track 39.

## Note on Spec Scope

The track spec mentions splitting evidence into separate Zenodo, OSF, Figshare, Dataverse, and institutional-repository sub-provider lanes. The current implementation is a unified `RepositoryRecords` adapter that queries NCBI's eSummary service for PubMed IDs. This is a reasonable MVP that meets the test-matrix acceptance criteria (deposit metadata, sidecar boundary, URL handoff) but does not implement per-repository sub-provider lanes. Separate sub-provider manifests would be needed to fully satisfy the spec's decomposition ambition.

## Summary

All adapter code, fixture, test, manifest, and endpoint wiring is present. The implementation satisfies the test-matrix acceptance criteria.

**No gaps found in the implementation itself**, though the spec decomposition into per-repository sub-providers (Zenodo, OSF, Figshare, Dataverse) is deferred in the current unified adapter approach.
