# Mature Zotero Plugin Test Matrix

| Scenario | Acceptance | Status | Evidence |
| --- | --- | --- | --- |
| Preview | Differences are shown without writes. | ✅ Done | `citation_sync.rs` — `run_citation_sync()` with `--preview` flag |
| Apply | Explicit apply writes audit log and changed records only. | ✅ Done | `--apply` flag, JSONL audit log, `CitationSyncConfig::apply` mode |
| Disposable smoke | Optional test library proves round trip or skips cleanly. | ❌ Deferred | Not implemented; future enhancement gated on `SOURCERIGHT_ZOTERO_TEST_LIBRARY_ID` |
| Packaging | `.xpi` or chosen package validates and installs locally. | ✅ Done | [`packaging-decision.md`](./packaging-decision.md) documents CLI/Web API binary as chosen package |
| Distribution | Docs separate shareable package from official acceptance. | ✅ Done | `packaging-decision.md` separates GitHub Releases (shareable) from Zotero Plugin Gallery (not applicable) |
| Review loop | `$conductor-review` runs and local fixes are applied. | 🔵 Pending | Run before final completion gate |
