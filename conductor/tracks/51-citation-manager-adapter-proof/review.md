# Track 51 — Citation Manager Adapter Proof: Completion Review

| Scenario | Acceptance | Evidence | Status |
|----------|------------|----------|--------|
| Zotero preview | Proposed changes visible without writes | `src/citation_sync.rs` (1,116 lines): preview mode, 6 suggestion classes, report JSON. Fixtures: `preview-exact-match.json`, `preview-title-update.json`. 8 unit tests + 3 schema contract tests. | ✅ PASS |
| Zotero apply | Apply requires explicit opt-in and writes audit log | `src/bin/citation-sync.rs`: `--apply` flag. `apply-success-preview.json` shows `applied=true` with `audit_log_path`. Apply mode writes audit JSONL. | ✅ PASS |
| EndNote handoff | ENW/RIS files generated and structurally checked | `src/export.rs` (442 lines): `export_ris()`, `export_enw()`. 3 inline unit tests verify structural output. | ✅ PASS |
| Boundary | EndNote export does not count as Zotero sync proof | `docs/src/citation-manager-integrations.md`: separate Zotero (preview/apply) and EndNote (file export) sections. `conductor/requirements.md` line 59: "Do not claim Zotero live sync from EndNote/RIS export proof." `spec.md`: "Separate Zotero sync proof from EndNote handoff proof." | ✅ PASS |
| Review loop | `$conductor-review` runs and local fixes applied | Plugin manifests updated, registry updated, metadata promoted, boundary claim verified. | ✅ PASS |

## Evidence Summary

### Zotero Sync Engine (1,116 lines)
- `CitationSyncConfig` — preview/apply/audit/remote config with Zotero-specific fields
- `CitationSyncReport` — schema_version `sourceright.citation_sync.v1`, 6 counters
- `CitationSyncAction` — Create, Update, Skip, Conflict (tagged enum)
- `CitationSyncSuggestionKind` — SafeUpdate, NoOp, LowConfidence, Suppressed, ReviewRequired, Conflict
- Core pipeline: `run_citation_sync()` with DOI/title exact match, narrow fit, conflict detection
- Apply writes audit log (JSONL) and remote fixture snapshot

### EndNote ENW/RIS Export (442 lines)
- `ExportFormat` enum with `export_ris()` and `export_enw()` generators
- CSL→RIS/ENW type mappings (article-journal→JOUR/%0 Journal Article)
- Internal metadata excluded from exports

### Fixtures (3 files at `fixtures/providers/zotero/`)
- `preview-exact-match.json` — DOI and title exact match → Skip/NoOp
- `preview-title-update.json` — Same DOI, different title → Update/SafeUpdate
- `apply-success-preview.json` — Full apply run with audit log: 1 skip, 1 update, 1 create

### Documentation
- `docs/src/citation-manager-integrations.md` — Full preview/apply contract + fixture documentation
- `conductor/tracks/51-citation-manager-adapter-proof/spec.md` — Clear boundary: Zotero proof ≠ EndNote proof
- `conductor/requirements.md` — Overclaim guard: "Do not claim Zotero live sync from EndNote/RIS export proof."

### Plugin Registry Status Changes

| Plugin | Previous Status | New Status |
|--------|----------------|------------|
| `citation-manager.zotero` | `planned_adapter` | **`fixture_tested`** |
| `citation-manager.endnote` | `planned_adapter` | **`fixture_tested`** |

## Boundary Claim Verification

The `docs/src/citation-manager-integrations.md` correctly maintains the boundary:

1. Sections are **separate** — "Zotero Preview And Apply" (§33-61) is distinct from EndNote file-handoff documentation.
2. `conductor/requirements.md` explicitly states the overclaim guard: *"Do not claim Zotero live sync from EndNote/RIS export proof."*
3. `spec.md` states: *"Separate Zotero sync proof from EndNote handoff proof."*

**Verdict**: Boundary claim is correct and properly documented.

## Status

- **Previous status**: in_progress
- **Current status**: completed
- **Evidence level**: fixture_tested (deterministic local fixtures prove behavior)
- **Plugin manifests**: both promoted from `planned_adapter` → `fixture_tested`
- **Review date**: 2025-05-15

