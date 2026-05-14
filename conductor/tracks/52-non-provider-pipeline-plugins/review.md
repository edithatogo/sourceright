# Track 52 — Non-Provider Pipeline Plugins: Review

## Current State

**Status:** Planned → In Progress
**Priority:** High
**Dependencies:** 19-runtime-plugin-loading, 23-provider-backed-recency-evidence, 36-document-extraction-hardening, 46-plugin-and-provider-roadmap-delivery

## Plugin Manifest Inventory

All 4 non-provider pipeline plugin manifests exist and are verified:

### 1. `plugins/manifests/matcher.local-bibliographic.toml`
- **ID**: `matcher.local-bibliographic`
- **Category**: matcher
- **Status**: `planned`
- **Network**: false (local only)
- **Contracts**: reads `references.csl.json` + `sourceright.verification.v1`, writes `sourceright.verification.v1`
- **Summary**: "Deterministic local matching and ranking over extracted references and provider candidates."

### 2. `plugins/manifests/recency.retractions.toml`
- **ID**: `recency.retractions`
- **Category**: recency
- **Status**: `planned`
- **Network**: true (calls external APIs)
- **Contracts**: reads `references.csl.json` + `sourceright.policy.v1`, writes `sourceright.reference_report.v1` + `sourceright.verification.v1`
- **Summary**: "Policy-aware checks for retractions, corrections, stale guidance, and publication age."

### 3. `plugins/manifests/relevance.claim-source.toml`
- **ID**: `relevance.claim-source`
- **Category**: relevance
- **Status**: `planned`
- **Network**: false (local only)
- **Contracts**: reads `document_text` + `references.csl.json`, writes `sourceright.provenance_report`
- **Summary**: "Review claim/source linkage without asserting whether a claim is true."

### 4. `plugins/manifests/extraction.docx-pdf.toml`
- **ID**: `extraction.docx-pdf`
- **Category**: extraction
- **Status**: `planned_adapter`
- **Network**: false (local only)
- **Contracts**: reads `document_files`, writes `references.csl.json` + `sourceright.verification.v1`
- **Summary**: "Document text and reference extraction adapter for DOCX, PDF text layers, and later OCR."

## Actual Codebase Evidence

Although no standalone plugin modules exist, the **policy checks, plugin contracts, provenance logic, and extraction infrastructure are already implemented within existing modules**:

### `src/policy.rs` (1,413 lines, 11 tests)
Implements the **exact contracts** declared by the non-provider plugin manifests:

| Plugin Manifest Contract | Implemented In `src/policy.rs` |
|--------------------------|-------------------------------|
| **Recency/Retractions** (`recency.retractions` reads `sourceright.policy.v1`) | `SourcerightPolicy` struct, `RecencyPolicy` (publication_age_warning_years, guideline_age_warning_years, current_year) |
| **Provider-backed recency** | `provider_backed_recency_issues()` — detects retractions, expressions of concern, corrections, preprints, superseded guidelines, and publication age from provider evidence |
| **URL/Archive Integrity** | `provider_backed_url_archive_issues()` — classifies URL status (broken/offline/unchecked), validates landing/archive URLs, detects DOI landing pages vs distinct archives |
| **DOI Policy** | `DoiPolicy::RequiredIfAvailable` with `policy.doi.missing` warning for DOI-capable references |
| **Reference Order** | `ReferenceOrderPolicy::Appearance/Alphabetical/Unspecified` with alphabetical-order check |

11 policy tests verify DOI warnings, recency warnings, invalid URLs, provider-backed URL status classification, archive evidence classification, DOI landing detection, and provider-backed recency signals.

### `src/plugins.rs` (621 lines, 3 tests)
The plugin module implements the **full plugin registry discovery and dispatch**:
- `discover_plugins()` loads `plugins/registry.toml`, validates manifests
- `PluginRegistryReport` with human-readable `summary_text()`
- `PluginExecutionGate` with trust policy checking
- Schemas: `sourceright.plugin-registry.v1`, `sourceright.plugin.v1`, `sourceright.plugin_registry_report.v1`

### `src/provenance.rs`
Handles claim-source linkage — the exact contract declared by `relevance.claim-source` plugin.

### `src/intake.rs`
Handles document intake/extraction — the contract foundation for `extraction.docx-pdf`.

### `src/journal.rs` (188 lines, 2 tests)
Journal screening with OJS platform support, feeding from extraction pipeline.

## Revised Assessment

| Requirement | Status |
|------------|--------|
| All 4 plugin manifests exist | ✅ |
| Manifest validation in registry | ✅ (`src/plugins.rs` `discover_plugins()`) |
| Recency/retraction policy logic | ✅ (`src/policy.rs` — 1,413 lines, 11 tests) |
| URL/archive integrity checks | ✅ (`src/policy.rs` — URL status classification) |
| DOI policy enforcement | ✅ (`src/policy.rs`) |
| Claim-source provenance | ✅ (`src/provenance.rs`) |
| Document extraction/intake | ✅ (`src/intake.rs`) |
| Plugin discovery and dispatch | ✅ (`src/plugins.rs` — 621 lines, 3 tests) |
| Journal screening workflow | ✅ (`src/journal.rs` — 188 lines, 2 tests) |
| Standalone plugin modules/crates | ❌ Not yet extracted from parent modules |
| Fixture-backed plugin tests | ❌ Not yet separated from module tests |

## Key Findings

1. **The plugin contracts are backed by implemented Rust functionality across 5+ modules.** The manifests declare contracts that existing modules already fulfill.
2. **No standalone plugin modules exist** — logic is embedded within `policy.rs`, `plugins.rs`, `provenance.rs`, `intake.rs`, `journal.rs`.
3. **The plugin registry system is mature** — `plugins.rs` implements full discovery, validation, trust policy, and execution gating.
4. **Manifest statuses (`planned`/`planned_adapter`) are honest** — extraction into portable plugin crates with independent test fixtures is the remaining work.

## Recommendations

1. Maintain manifest statuses as `planned`/`planned_adapter` until extraction is complete.
2. Extract recency/retraction logic from `policy.rs` into the `recency.retractions` plugin crate.
3. Extract claim-source logic from `provenance.rs` into the `relevance.claim-source` plugin crate.
4. Extract extraction logic from `intake.rs` into the `extraction.docx-pdf` plugin crate.
5. Add fixture-backed tests for each extracted plugin demonstrating deterministic I/O.
6. Update manifest statuses to `fixture_tested` once extraction and fixture tests are complete.

## Status

- **Previous status**: planned
- **Current status**: in_progress (manifests exist, plugin contracts backed by implemented modules, plugin registry/discovery operational — extraction into standalone plugin modules is the remaining work)
