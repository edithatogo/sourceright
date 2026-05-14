# Track 58 — Mature Zotero Plugin: Completion Checklist

## Status: In Progress

> Generated: 2026-05-14
> Source: Review of `docs/src/zotero-plugin-install.md`, `fixtures/providers/zotero/README.md`, source code evidence

---

## 1. Requirements Checklist

| # | Requirement | Criterion | Status | Evidence |
|---|-------------|-----------|--------|----------|
| 1 | **Preview (dry-run)** | Differences shown without writes | ✅ Done | `src/citation_sync.rs` — `run_citation_sync()` with preview mode; `--preview` CLI flag |
| 2 | **Apply (explicit)** | Explicit apply writes audit log and changed records only | ✅ Done | `--apply` CLI flag; audit log writes JSONL; `CitationSyncConfig::apply` mode |
| 3 | **Disposable-library smoke** | Optional test library proves round trip or skips cleanly | ❌ Missing | No env-guarded live Zotero test exists |
| 4 | **Packaging** | `.xpi` or chosen package validates and installs locally | ❌ Deferred | Architecture decision documents CLI/Web API over `.xpi` plugin |
| 5 | **Distribution** | Docs separate shareable package from official acceptance | ❌ Missing | No Zotero Forum or Plugin Gallery listing docs exist |
| 6 | **Install documentation** | Complete install notes with API key setup, CLI usage, troubleshooting | ✅ Done | `docs/src/zotero-plugin-install.md` |
| 7 | **Fixture definitions** | README documents planned fixture scenarios and format | ✅ Done | `fixtures/providers/zotero/README.md` |
| 8 | **Fixture files** | Actual fixture JSON files exist for key scenarios | ✅ Done | `preview-exact-match.json`, `preview-title-update.json`, `apply-success-preview.json` |
| 9 | **Fixture-backed tests** | Rust tests load fixtures and verify report output | ❌ Missing | Existing 8 unit tests use inline data; no fixture-driven tests |
| 10 | **Schema contract** | Report JSON matches published schema | ✅ Done | `tests/citation_sync_schema_contract.rs` (151 lines, 3 tests) |
| 11 | **Unit tests** | Core engine has adequate test coverage | ✅ Done | 8 inline tests covering preview/apply/match/conflict/suppress |
| 12 | **Plugin manifest** | Manifest declared in registry with accurate status | ✅ Done | `plugins/manifests/citation-manager.zotero.toml` — `planned_adapter` |

---

## 2. Artifact Inventory

| Artifact | Path | Status |
|----------|------|--------|
| Plugin manifest | `plugins/manifests/citation-manager.zotero.toml` | ✅ Present |
| Registry entry | `plugins/registry.toml` | ✅ Present |
| Profile YAML | `examples/citation-manager-profiles/zotero.yaml` | ✅ Present |
| Sync engine | `src/citation_sync.rs` (1,116 lines) | ✅ Mature |
| CLI binary | `src/bin/citation-sync.rs` (87 lines) | ✅ Present |
| JSON Schema (report) | `schemas/sourceright.citation-sync.schema.json` | ✅ Present |
| JSON Schema (manifest) | `schemas/sourceright.sync-manifest.schema.json` | ✅ Present |
| Unit tests | `src/citation_sync.rs` inline | ✅ 8 tests |
| Schema integration tests | `tests/citation_sync_schema_contract.rs` | ✅ 3 tests |
| Integration docs | `docs/src/citation-manager-integrations.md` | ✅ Present |
| Install docs | `docs/src/zotero-plugin-install.md` | ✅ **NEW** |
| Fixture README | `fixtures/providers/zotero/README.md` | ✅ **NEW** |
| Fixture: exact match | `fixtures/providers/zotero/preview-exact-match.json` | ✅ **NEW** |
| Fixture: title update | `fixtures/providers/zotero/preview-title-update.json` | ✅ **NEW** |
| Fixture: apply preview | `fixtures/providers/zotero/apply-success-preview.json` | ✅ **NEW** |
| Disposable-library smoke | N/A | ❌ Missing |
| `.xpi` packaging | N/A | ❌ Deferred |
| Distribution docs | N/A | ❌ Missing |
| Fixture-backed Rust tests | N/A | ❌ Missing |

---

## 3. Gaps Remaining

| Gap | Priority | Effort | Notes |
|-----|----------|--------|-------|
| **Fixture-backed Rust tests** | High | Medium | Write integration tests in `tests/` that load fixtures from `fixtures/providers/zotero/` via `CitationSyncConfig::remote_fixture_path`, then assert report fields match expected values. Three fixtures now exist as input data. |
| **Distribution notes** | Medium | Small | Document whether Zotero Forum listing is planned, or document that CLI/Web API model does not use Zotero plugin distribution channels. |
| **Disposable-library smoke** | Low | Medium | Create a gated test (env var `SOURCERIGHT_ZOTERO_TEST_LIBRARY_ID`) that creates temporary items, runs preview/apply, and cleans up. Guard with `#[ignore]` or feature flag. |
| **Update manifest status to `fixture_tested`** | Medium | Small | Once fixture-backed tests exist, update manifest from `planned_adapter` to `fixture_tested`. |
| **`.xpi` packaging** | Low | Large | Deferred by architecture decision. Revisit if a browser-plugin use case emerges. |

---

## 4. Completion Gate Criteria

**To mark this track complete, all of the following must be true:**

- [ ] Fixture-backed Rust integration tests exist and pass in CI
- [ ] Plugin manifest status updated to `fixture_tested`
- [ ] Install docs are verified consistent with actual CLI behavior
- [ ] Distribution approach documented (or explicitly deferred)
- [ ] All 12 requirements in section 1 are ✅ Done or explicitly deferred with documented rationale

**Current progress:** 8 of 12 requirements ✅ Done (requirements 3, 4, 5, 9 remain open)
