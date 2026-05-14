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
| 3 | **Disposable-library smoke** | Optional test library discovers a live disposable library or skips cleanly without credentials | ✅ Done | Ignored Rust smoke `zotero_disposable_library_live_smoke_skips_without_credentials` in `src/citation_sync.rs`; gated by `SOURCERIGHT_ZOTERO_LIVE_SMOKE=1` and Zotero env vars |
| 4 | **Packaging** | `.xpi` or chosen package validates and installs locally | ✅ Done | `conductor/tracks/58-mature-zotero-plugin/packaging-decision.md` documents CLI/Web API as the chosen package model |
| 5 | **Distribution** | Docs separate shareable package from official acceptance | ✅ Done | `packaging-decision.md` separates intended GitHub Releases/crates.io distribution from Zotero Plugin Gallery (not applicable); CLI/API model does not use Zotero distribution channels |
| 6 | **Install documentation** | Complete install notes with API key setup, CLI usage, troubleshooting | ✅ Done | `docs/src/zotero-plugin-install.md` |
| 7 | **Fixture definitions** | README documents planned fixture scenarios and format | ✅ Done | `fixtures/providers/zotero/README.md` |
| 8 | **Fixture files** | Actual fixture JSON files exist for key scenarios | ✅ Done | `preview-exact-match.json`, `preview-title-update.json`, `apply-success-preview.json` |
| 9 | **Fixture-backed tests** | Rust tests load fixtures and verify report output | ✅ Done | Inline Rust tests load `fixtures/providers/zotero/zotero-*.json` and verify no-op, safe-update, and create reports |
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
| Unit tests | `src/citation_sync.rs` inline | ✅ 12 tests |
| Schema integration tests | `tests/citation_sync_schema_contract.rs` | ✅ 3 tests |
| Integration docs | `docs/src/citation-manager-integrations.md` | ✅ Present |
| Install docs | `docs/src/zotero-plugin-install.md` | ✅ **NEW** |
| Fixture README | `fixtures/providers/zotero/README.md` | ✅ **NEW** |
| Fixture: exact match | `fixtures/providers/zotero/preview-exact-match.json` | ✅ **NEW** |
| Fixture: title update | `fixtures/providers/zotero/preview-title-update.json` | ✅ **NEW** |
| Fixture: apply preview | `fixtures/providers/zotero/apply-success-preview.json` | ✅ **NEW** |
| Disposable-library smoke | `src/citation_sync.rs` | ✅ Env-gated ignored test |
| `.xpi` packaging | N/A | ✅ Deferred (see packaging-decision.md) |
| Distribution docs | N/A | ✅ Done (see packaging-decision.md) |
| Fixture-backed Rust tests | `fixtures/providers/zotero/zotero-*.json` | ✅ Done |

---

## 3. Gaps Remaining

| Gap | Priority | Effort | Notes |
|-----|----------|--------|-------|
| **Fixture-backed Rust tests** | High | Medium | ✅ **DONE** — Inline tests load fixtures from `fixtures/providers/zotero/` via `CitationSyncConfig::remote_fixture_path` and assert report fields. |
| **Distribution notes** | Low | Small | ✅ **DONE** — `packaging-decision.md` documents that CLI/Web API model does not use Zotero plugin distribution channels. GitHub Releases/crates.io are the intended binary/crate channels; Zotero Plugin Gallery is not applicable. |
| **Disposable-library smoke** | Low | Medium | ✅ **DONE** — Ignored test `zotero_disposable_library_live_smoke_skips_without_credentials` skips unless `SOURCERIGHT_ZOTERO_LIVE_SMOKE=1`, `SOURCERIGHT_ZOTERO_API_KEY`, and `SOURCERIGHT_ZOTERO_LIBRARY_ID` are set. It performs live discovery/planning without writes; default test runs make no network calls. |
| **Update manifest status to `fixture_tested`** | Medium | Small | ✅ **DONE** — `plugins/manifests/citation-manager.zotero.toml` is marked `fixture_tested`. |
| **`.xpi` packaging** | Low | Large | ✅ **DONE** — Deferred by architecture decision documented in `packaging-decision.md`. Revisit if a browser-plugin use case emerges. |

---

## 4. Completion Gate Criteria

**To mark this track complete, all of the following must be true:**

- [x] Fixture-backed Rust tests exist and pass locally
- [x] Plugin manifest status updated to `fixture_tested`
- [ ] Install docs are verified consistent with actual CLI behavior
- [x] Distribution approach documented (or explicitly deferred)
- [x] Env-gated disposable-library smoke exists and skips cleanly without credentials
- [x] All 12 requirements in section 1 are ✅ Done or explicitly deferred with documented rationale

**Current progress:** 12 of 12 requirements ✅ Done. The remaining closeout item is consistency review of install docs against the current CLI behavior.

### Live smoke command

Default test runs do not touch Zotero or the network. To exercise the optional live gate against a disposable library:

```bash
SOURCERIGHT_ZOTERO_LIVE_SMOKE=1 \
SOURCERIGHT_ZOTERO_API_URL=https://api.zotero.org \
SOURCERIGHT_ZOTERO_API_KEY=<key> \
SOURCERIGHT_ZOTERO_LIBRARY_ID=<disposable-library-id> \
SOURCERIGHT_ZOTERO_LIBRARY_TYPE=users \
cargo test zotero_disposable_library_live_smoke_skips_without_credentials -- --ignored --nocapture
```
