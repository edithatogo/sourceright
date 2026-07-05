# Test Matrix

## Fixture smoke — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| OJS submission fixture produces editor and author outputs | `tests/cli_end_to_end.rs::ojs_fixture_screens_to_editor_and_author_outputs_end_to_end` passes | Fixture `fixtures/journal/ojs-submission.json` exercised; `JournalScreeningReport` contains `editorial_summary` and `author_action_checklist` | default-CI |
| Policy test enforces OJS plugin package structure | `tests/ojs_plugin_packaging_policy.rs` passes: `plugins/ojs/sourceright/` contains required files (`index.php`, `SourcerightPlugin.php`, `version.xml`, `plugin.xml`, locale, README) | `tests/ojs_plugin_packaging_policy.rs` (`#[test] fn package_structure_has_required_files`) | default-CI |
| Build script produces install archive | `scripts/build-ojs-plugin-package.ps1` exits 0 and creates `dist/ojs/sourceright-ojs-generic-plugin-0.1.0.tar.gz` with SHA-256 sidecar | Build script output and `dist/ojs/` contents | default-CI |
| Repo-local lint passes | `scripts/ojs-plugin-lint.ps1` exits 0 (PHP lint skipped when PHP not on PATH) | Lint script output | default-CI |

## Plugin packaging review — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Plugin entry point exists | `plugins/ojs/sourceright/index.php` returns `new SourcerightPlugin()` | Source file present; lint check passes | default-CI |
| Plugin XML metadata present | `version.xml` declares OJS 3.x plugin type, release 0.1.0 | Source file present | default-CI |
| Gallery descriptor present | `plugin.xml` has category, product, name, summary, description, release, version, author, homepage, license | Source file present; disclaimer: "not PKP Plugin Gallery accepted" | default-CI |
| Localisation present | `locale/en_US/locale.po` contains English strings | Source file present | default-CI |
| Installation docs present | `README.md` documents install steps, dependencies, and boundary | Source file present | default-CI |
| Permissions and data boundaries documented | Track installation guide and plugin README document local processing, role visibility, and no silent canonical CSL overwrite | Reviewed in `conductor/tracks/75-journal-platform-publication-hardening/` docs | default-CI |

## PKP Gallery listing probes (opt-in live)

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Pre-submission gallery search | `https://github.com/pkp/plugin-gallery/search?q=sourceright` returns **no matches** (or pre-existing results) | `browser-gallery-verification-*.md` row with probe URL and result | opt-in-live |
| Gallery submission PR opened | Fork of `pkp/plugin-gallery` has open PR adding Sourceright plugin entry | `browser-gallery-verification-*.md` row with PR URL, status (open), date | opt-in-live |
| Gallery PR status check (follow-up) | PR is open/merged/closed; review comments recorded if any | `browser-gallery-verification-*.md` updated with PR status check date | opt-in-live |
| Post-acceptance gallery search (if merged) | `https://github.com/pkp/plugin-gallery/search?q=sourceright` returns match | `browser-gallery-verification-*.md` row with match result | opt-in-live |

## Disposable-instance smoke (opt-in live)

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Docker preflight | Docker daemon running, WSL2 available, `docker compose` functional | Preflight output recorded in `live-smoke-*.md` | opt-in-live |
| OJS container stack starts | `scripts/ojs-docker-install-smoke.ps1 -FetchPkpContainers` pulls and starts PKP containers; OJS accessible via HTTP | Live smoke transcript: container startup log, OJS URL accessibility check | opt-in-live |
| Plugin install via generic upload | Sourceright plugin archive uploads and activates in OJS Plugin Gallery | Live smoke transcript: upload confirmation, activation status | opt-in-live |
| Screening invocation | Plugin calls Sourceright CLI on a test submission; returns screening report | Live smoke transcript: CLI invocation output, report JSON/Markdown | opt-in-live |
| Screening report verified | Report contains `editorial_summary` and `author_action_checklist` fields | Live smoke transcript: report content verification | opt-in-live |
| Plugin uninstall (if tested) | Plugin deactivates and removes without errors | Live smoke transcript: uninstall confirmation | opt-in-live |

## Evidence recording

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Browser gallery evidence committed | `browser-gallery-verification-*.md` updated with PR URL, status, and any post-PR search results | File exists in track directory | opt-in-live |
| Live smoke evidence committed | `live-smoke-{date}.md` committed with full transcript (or blocker documentation) | File exists in track directory | opt-in-live |
| Release-status docs updated (on gallery acceptance) | `docs/src/release-status.md` and docs-site mirror reflect OJS/PKP from "prepared" to "accepted" (if merged) | Docs build and review | opt-in-live |
| Claim boundary verified | No track doc claims "gallery-accepted" without PKP maintainer merge evidence | Review of all evidence docs | opt-in-live |
