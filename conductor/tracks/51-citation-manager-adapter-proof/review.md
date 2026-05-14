# Track 51 — Citation Manager Adapter Proof: Review

## Current State

### Evidence Inventory

| Artifact | Path | Status |
|----------|------|--------|
| Plugin manifest (Zotero) | `plugins/manifests/citation-manager.zotero.toml` | ✅ `planned_adapter` |
| Plugin manifest (EndNote) | `plugins/manifests/citation-manager.endnote.toml` | ✅ `planned_adapter` |
| Registry entry (Zotero) | `plugins/registry.toml` | ✅ `planned_adapter` |
| Registry entry (EndNote) | `plugins/registry.toml` | ✅ `planned_adapter` |
| Citation sync engine | `src/citation_sync.rs` (1,116 lines) | ✅ **Mature implementation** |
| CLI binary | `src/bin/citation-sync.rs` (87 lines) | ✅ `--preview`/`--apply` CLI |
| JSON Schema (sync report) | `schemas/sourceright.citation-sync.schema.json` | ✅ Draft 2020-12, 131 lines |
| JSON Schema (sync manifest) | `schemas/sourceright.sync-manifest.schema.json` | ✅ Draft 2020-12, 51 lines |
| Integration tests | `tests/citation_sync_schema_contract.rs` (151 lines) | ✅ Schema contract tests |
| Unit tests (citation_sync) | `src/citation_sync.rs` inline | ✅ 8 tests covering all scenarios |
| Export engine (ENW/RIS) | `src/export.rs` (442 lines) | ✅ **ENW/RIS/BibLaTeX/XML/YAML** |
| Export tests | `src/export.rs` inline | ✅ 3 tests |
| Profile YAML (Zotero) | `examples/citation-manager-profiles/zotero.yaml` | ✅ `api_and_file`, `dry_run` |
| Profile YAML (EndNote) | `examples/citation-manager-profiles/endnote.yaml` | ✅ `file` family, `enw/ris/xml` |
| Sync manifest dry-run example | `examples/citation-manager-profiles/sync-manifest.dry-run.json` | ✅ RIS export dry run |
| Documentation | `docs/src/citation-manager-integrations.md` | ✅ Preview/apply contract |
| Provider fixtures | `provider-fixtures/` (7 subdirs) | ❌ **No Zotero/EndNote dirs** |
| Zotero fixture: exact match | `fixtures/providers/zotero/preview-exact-match.json` | ✅ **NEW - 2025-05-14** |
| Zotero fixture: title update | `fixtures/providers/zotero/preview-title-update.json` | ✅ **NEW - 2025-05-14** |
| Zotero fixture: apply preview | `fixtures/providers/zotero/apply-success-preview.json` | ✅ **NEW - 2025-05-14** |

### Plugin Manifests

Both Zotero and EndNote have registered plugin manifests in `plugins/manifests/` with entries in `plugins/registry.toml`. Both use status `planned_adapter`, which accurately reflects current maturity — adapter contracts are defined but not yet proven with fixture-backed tests.

### Source Code Analysis

**`src/citation_sync.rs` (1,116 lines)** — the Zotero sync engine:
- `CitationSyncConfig` — preview/apply/audit/remote config with Zotero-specific fields
- `CitationSyncReport` — schema_version `sourceright.citation_sync.v1`, 6 counters
- `CitationSyncAction` — Create, Update, Skip, Conflict (tagged enum)
- `CitationSyncSuggestionKind` — SafeUpdate, NoOp, LowConfidence, Suppressed, ReviewRequired, Conflict
- Core pipeline: `run_citation_sync()` with DOI/title exact match, narrow fit, conflict detection
- Apply writes audit log (JSONL) and remote fixture snapshot
- **8 unit tests** covering all major scenarios

**`src/export.rs` (442 lines)** — ENW/RIS export for EndNote:
- `ExportFormat` enum with `export_ris()` and `export_enw()` generators
- CSL→RIS/ENW type mappings (article-journal→JOUR/%0 Journal Article)
- Internal metadata excluded from exports
- **3 unit tests** verify filenames, clean exports, structural output

**`src/bin/citation-sync.rs` (87 lines)** — CLI with `--preview`/`--apply` flags

**`tests/citation_sync_schema_contract.rs` (151 lines)** — verifies serialized report, suggestion kinds, and action shapes match JSON Schema

### Gap Analysis

| Requirement | Zotero | EndNote |
|-------------|--------|---------|
| Plugin manifest | ✅ `planned_adapter` | ✅ `planned_adapter` |
| Registry entry | ✅ `planned_adapter` | ✅ `planned_adapter` |
| Profile YAML | ✅ `api_and_file` | ✅ `file` family |
| Source code | ✅ `citation_sync.rs` (1,116 lines) | ✅ `export.rs` (ENW/RIS) |
| Preview/apply/audit | ✅ Config + CLI + audit log | ❌ N/A (file export) |
| ENW/RIS export | ❌ Not in citation_sync.rs | ✅ `export_ris()` + `export_enw()` |
| JSON Schema | ✅ citation-sync + sync-manifest | ✅ sync-manifest |
| Integration tests | ✅ Schema contract tests | ❌ No fixture tests |
| Unit tests | ✅ 8 tests | ✅ 3 tests |
| Fixture dir | ❌ Not created | ❌ Not created |
| Disposable smoke | ❌ Not implemented | ❌ N/A |

### Key Findings

1. **Zotero sync engine is substantially mature.** The 1,116-line module implements preview/apply/audit with Zotero API config, narrow-fit matching, 6 suggestion classes, and structured audit logs. 8 unit tests and 3 schema contract tests pass.

2. **EndNote ENW/RIS export is fully implemented.** `export.rs` has dedicated `export_ris()` and `export_enw()` generators with correct tag mappings and type conversions.

3. **No fixture directories exist** for either manager. The `provider-fixtures/` directory has 7 subdirectories for live providers but zero for citation managers.

4. **Profile YAMLs exist for 8 managers** — only Zotero and EndNote have plugin manifests.

5. **A sync-manifest dry-run example** demonstrates the intended file handoff flow.

## Recommendations

1. **Create `provider-fixtures/zotero/`** with remote fixture JSON for preview comparison, apply audit scenarios, and match-test cases.

2. **Create `provider-fixtures/endnote/`** with sample ENW/RIS files and reparse-expected CSL.

3. **Add ENW/RIS reparse integration test** to verify round-trip integrity.

4. **Update Zotero manifest status** to `fixture_tested` once fixture tests exist.

5. **Defer disposable-library smoke** as a future enhancement.

## Status

- **Previous status**: planned
- **Current status**: in_progress (mature engine exists, schema contracts tested, fixtures needed to advance manifest status)
