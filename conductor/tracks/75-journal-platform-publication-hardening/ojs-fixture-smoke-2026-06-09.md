# OJS Fixture and Package Smoke — Local Evidence

Date: 2026-06-09  
Version: 0.1.20 (Sourceright crate) / 0.1.0 (OJS plugin package)

## Package lint and build

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\ojs-plugin-lint.ps1 `
  -OutputDir C:\tmp\sourceright-ojs-lint-track75\packages
```

Result:

| Check | Outcome |
| --- | --- |
| Archive | `C:\tmp\sourceright-ojs-lint-track75\packages\sourceright-ojs-generic-plugin-0.1.0.tar.gz` |
| SHA-256 sidecar | Present |
| Archive entries | ok |
| XML validation | powershell-xml |
| PHP lint | skipped: php not on PATH |
| Rust policy tests | passed |

## Default-CI tests (2026-06-09)

```text
cargo test --test ojs_plugin_packaging_policy
  4 tests ... ok

cargo test --test cli_end_to_end ojs_fixture_screens
  ojs_fixture_screens_to_editor_and_author_outputs_end_to_end ... ok
```

Target dirs: `C:\tmp\sourceright-ojs-lint-target`, `C:\tmp\sourceright-target-track75`
(GNU toolchain, locked deps).

## Opt-in live smoke (not run)

- `scripts/ojs-docker-install-smoke.ps1` — disposable OJS install planning
- `conductor/tracks/60-mature-ojs-plugin/ojs-install-smoke.md` — manual transcript path

No verified live OJS install transcript is recorded in this slice.

## Claim boundary

This evidence supports **hardened local package** (source tree, install archive,
fixture screening). It does **not** claim PKP Plugin Gallery acceptance or live
OJS compatibility on a running instance.
