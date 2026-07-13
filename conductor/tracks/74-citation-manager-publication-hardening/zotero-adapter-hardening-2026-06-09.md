# Zotero Adapter Hardening — Local Evidence

Date: 2026-06-09  
Version: 0.1.20

## Package decision

Zotero support is the **Sourceright CLI/Web API adapter**, not a Zotero `.xpi`.
Source: `publication-decision-2026-05-18.md`,
`conductor/tracks/58-mature-zotero-plugin/packaging-decision.md`.

## Manifest and docs

| Artifact | Role |
| --- | --- |
| `plugins/manifests/citation-manager.zotero.toml` | Plugin manifest (`fixture_tested`, dry-run default) |
| `docs/src/zotero-plugin-install.md` | Install and claim boundary |
| `fixtures/providers/zotero/` | Fixture-backed preview/apply/audit proof |

## Default-CI checks (2026-06-09)

```text
cargo test zotero --lib
  citation_sync::tests::zotero_api_shaped_fixture_is_parsed_for_preview ... ok
  citation_sync::tests::zotero_disposable_library_live_smoke_skips_without_credentials ... ignored

cargo test --test zotero_ci_policy
  zotero_live_smoke_workflow_is_manual_and_secret_gated ... ok
  zotero_desktop_smoke_workflow_is_manual_and_isolated ... ok
  zotero_docs_do_not_claim_xpi_submission_or_desktop_test ... ok

cargo test --test examples_policy citation_manager
  citation_manager_examples_default_to_dry_run_without_stored_credentials ... ok
```

Target dir: `C:\tmp\sourceright-target-track74` (GNU toolchain, locked deps).

## Opt-in live smoke (not run in this slice)

- `.github/workflows/zotero-live-smoke.yml` — disposable library API smoke
  (`SOURCERIGHT_ZOTERO_LIVE_SMOKE=1`, API key + library id).
- `.github/workflows/zotero-desktop-smoke.yml` — isolated desktop Local API probe.

Live smoke remains manual and credential-gated. No live run is required to claim
**hardened local adapter package** evidence.

## Claim boundary

This evidence supports a prepared/hardened local adapter package. It does **not**
claim Zotero Plugin Gallery acceptance, `.xpi` distribution, or default-CI live
library writes.
