# Mature Zotero Plugin Test Matrix

| Scenario | Acceptance | Status | Evidence |
| --- | --- | --- | --- |
| Preview | Differences are shown without writes. | ✅ Done | `citation_sync.rs` — `run_citation_sync()` with `--preview` flag |
| Apply | Explicit apply writes audit log and changed records only. | ✅ Done | `--apply` flag, JSONL audit log, `CitationSyncConfig::apply` mode |
| Disposable smoke | Optional disposable library smoke discovers/plans through live Zotero API only when explicitly env-enabled; otherwise skips cleanly. | ✅ Done | Ignored Rust test `zotero_disposable_library_live_smoke_skips_without_credentials`; requires `SOURCERIGHT_ZOTERO_LIVE_SMOKE=1`, `SOURCERIGHT_ZOTERO_API_KEY`, and `SOURCERIGHT_ZOTERO_LIBRARY_ID`; no default network calls |
| GitHub Actions live smoke | Manual workflow can run fixture-backed and disposable-library Zotero API checks without PR-time secrets. | ✅ Done | `.github/workflows/zotero-live-smoke.yml`; protected `zotero-live-smoke` environment; fixture job always runs, live job skips unless secrets are configured |
| Zotero Desktop local API smoke | Manual workflow can launch Zotero under `xvfb` with a temporary profile and probe the local API. | 🔶 Experimental | `.github/workflows/zotero-desktop-smoke.yml`; validates local API availability only, not `.xpi` loading |
| Packaging | `.xpi` or chosen package validates and installs locally. | ✅ Done | [`packaging-decision.md`](./packaging-decision.md) documents CLI/Web API binary as chosen package |
| Distribution | Docs separate shareable package from official acceptance. | ✅ Done | `packaging-decision.md` separates GitHub Releases (shareable) from Zotero Plugin Gallery (not applicable) |
| Review loop | `$conductor-review` runs and local fixes are applied. | 🔵 Pending | Run before final completion gate |

## Live Smoke Gate

Default checks do not call Zotero. The live smoke is discoverable and ignored by default:

```bash
cargo test zotero_disposable_library_live_smoke_skips_without_credentials -- --ignored --nocapture
```

Without `SOURCERIGHT_ZOTERO_LIVE_SMOKE=1` and credentials, the test prints a skip reason and returns successfully. With those variables set, it fetches the disposable library and runs citation-sync planning with `apply=false`, so it does not write an audit log or mutate Zotero.
