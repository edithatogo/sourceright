# Track 58 â€” Mature Zotero Plugin: Review

## Current State

### Documentation Review

**Spec** requirements:
- Dry-run preview compares Zotero records with Sourceright CSL/sidecar state
- Apply requires explicit user action and writes an audit log
- Disposable-library smoke test proves write semantics
- `.xpi` packaging, install notes, versioning, permissions, public distribution notes
- Distribution notes cover Zotero Forums/plugin listing state

**Test Matrix**:
| Scenario | Acceptance |
|----------|-----------|
| Preview | Differences are shown without writes |
| Apply | Explicit apply writes audit log and changed records only |
| Disposable smoke | Optional test library proves round trip or skips cleanly |
| Packaging | `.xpi` or chosen package validates and installs locally |
| Distribution | Docs separate shareable package from official acceptance |

### Codebase Evidence

| Component | Status | Location |
|-----------|--------|----------|
| Plugin manifest | âœ… `planned_adapter` | `plugins/manifests/citation-manager.zotero.toml` |
| Registry entry | âœ… `planned_adapter` | `plugins/registry.toml` |
| Profile YAML | âœ… `api_and_file`, `dry_run` default | `examples/citation-manager-profiles/zotero.yaml` |
| Citation sync engine | âœ… **Mature (1,116 lines)** | `src/citation_sync.rs` |
| CLI binary | âœ… `--preview`/`--apply` flags, Zotero env vars | `src/bin/citation-sync.rs` |
| JSON Schema (report) | âœ… Draft 2020-12 | `schemas/sourceright.citation-sync.schema.json` |
| JSON Schema (manifest) | âœ… Draft 2020-12 | `schemas/sourceright.sync-manifest.schema.json` |
| Unit tests | âœ… 8 tests in citation_sync.rs | preview, exact match, duplicate, apply, conflict, narrow fit, suppressed, title-update |
| Schema integration tests | âœ… 3 tests in tests/ | `tests/citation_sync_schema_contract.rs` |
| Documentation | âœ… Docs exist | `docs/src/citation-manager-integrations.md` |
| Dry-run manifest example | âœ… | `examples/citation-manager-profiles/sync-manifest.dry-run.json` |
| Fixture-backed tests | âŒ Not found | No `provider-fixtures/zotero/` directory |
| Disposable library smoke | âŒ Not found | No env-guarded live Zotero test |
| `.xpi` packaging | âŒ Not found | No packaging scripts or build |
| Zotero plugin install docs | [OK] Created | `docs/src/zotero-plugin-install.md` |
| Zotero fixture README | [OK] Created | `fixtures/providers/zotero/README.md` |
| Distribution notes | [??] Not found | No Zotero Forum/Plugin Gallery listing docs |

### Maturity Assessment

**What exists (substantial):**
- The `citation_sync.rs` engine implements the full preview/apply/audit contract with Zotero Web API integration (config fields: `zotero_api_url`, `zotero_api_key`, `zotero_library_id`, `zotero_library_type`)
- `CitationSyncConfig` supports remote fixture loading for deterministic testing without live Zotero
- `plan_sync_actions()` implements multiple match strategies: DOI exact, title exact, narrow fit (shared tokens, common prefix), DOI prefix conflict
- 6 `CitationSyncSuggestionKind` values (SafeUpdate, NoOp, LowConfidence, Suppressed, ReviewRequired, Conflict) provide nuanced suggestion taxonomy
- Apply mode writes JSONL audit logs and optional remote fixture snapshots
- CLI binary reads `SOURCERIGHT_ZOTERO_*` environment variables
- Sync-manifest JSON Schema supports `dry_run`/`write_files`/`api_sync` modes
- Zotero profile YAML defines the adapter as `api_and_file` family with `dry_run` default

**What is missing:**
1. **Provider fixtures** â€” No `provider-fixtures/zotero/` directory with deterministic remote record JSON files for preview comparison and apply audit testing
2. **Fixture-backed Rust tests** â€” The existing 8 unit tests use inline data; adding fixture-based tests would strengthen the proof
3. **Disposable-library smoke** â€” A gated integration test that creates a temporary Zotero test library, runs preview/apply, and verifies results (guarded on e.g. `SOURCERIGHT_ZOTERO_TEST_LIBRARY_ID` env var)
4. **`.xpi` packaging** â€” No browser plugin packaging; the integration is CLI/Rust-based via the Zotero Web API. This is appropriate for server-side sync but differs from the traditional Zotero plugin model
5. **Install documentation** -- Created `docs/src/zotero-plugin-install.md` with API key setup, CLI instructions, and compatibility matrix
6. **Fixture directory README** -- Created `fixtures/providers/zotero/README.md` documenting planned fixture scenarios and format

### Architecture Decision: API Mode

The current integration targets the **Zotero Web API** (v3) via `reqwest` blocking HTTP client. This is the right choice for sourceright's server-side/CLI architecture:
- No browser plugin dependency
- Works in CI/CD pipelines
- Compatible with MCP server runtime
- Can be used by Zotero groups/shared libraries
- A `.xpi` browser plugin would be a separate product, not required for the core sync contract

### Key Findings

1. **The preview/apply/audit contract is substantially implemented.** The engine handles create, update, skip, conflict, suppress, and review-required actions with structured explanations.

2. **No fixture-backed tests exist.** This is the highest-priority gap â€” deterministic fixture data enables reliable regression testing.

3. **No .xpi packaging.** The CLI/MCP integration pattern is architecturally appropriate.
4. **Install documentation now exists.** `docs/src/zotero-plugin-install.md` covers API key setup, CLI usage, compatibility, and architecture rationale.
5. **Fixture directory README created.** `fixtures/providers/zotero/README.md` documents planned fixture scenarios.

## Recommendations

1. **Create `provider-fixtures/zotero/`** with remote fixture JSON files covering:
   - Preview scenarios (single match, exact match, narrow fit, conflict)
   - Apply scenarios (create, update, skip)
   - Edge cases (suppressed, review-required)

2. **Add fixture-driven integration tests** that load fixtures and verify report output against expected values.

3. **[DONE] `docs/src/zotero-plugin-install.md` created** -- Covers API key setup, CLI examples, compatibility matrix (Zotero 5.x/6.x, API v3), architecture rationale.
4. **[DONE] `fixtures/providers/zotero/README.md` created** -- Documents fixture format, planned scenarios, and creation instructions.
5. **Defer .xpi packaging.** Document in the track why CLI/Web API is the chosen integration model.
6. **Defer disposable-library smoke** but note it as a future enhancement gated on env vars.
7. **Update plugin manifest status** from planned_adapter to fixture_tested once fixtures and tests are in place.
## Status

- **Current status**: in_progress (mature engine exists, install docs created, fixture README created, fixtures+integration tests needed, .xpi deferred)
