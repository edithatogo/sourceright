# Plan

## Phases

### 1. [x] Discover
- PKP Gallery search at `https://github.com/pkp/plugin-gallery/search?q=sourceright` returned **no matches** — no existing listing.
- Local OJS generic plugin source skeleton exists at `plugins/ojs/sourceright/` (Track 60 completion): `index.php`, `SourcerightPlugin.php`, `version.xml`, `plugin.xml`, README, English locale.
- Build script exists: `scripts/build-ojs-plugin-package.ps1` builds `dist/ojs/sourceright-ojs-generic-plugin-0.1.0.tar.gz`.
- Fixture smoke exists: `fixtures/journal/ojs-submission.json` exercised by `tests/cli_end_to_end.rs::ojs_fixture_screens_to_editor_and_author_outputs_end_to_end`.
- Policy test exists: `tests/ojs_plugin_packaging_policy.rs`.
- Docker preflight exists: `scripts/ojs-docker-install-smoke.ps1` (stages PKP containers; Docker daemon may not be running locally).
- Gallery readiness gaps documented in Track 60 `gallery-readiness.md`.
- Evidence: `browser-gallery-verification-2026-06-10.md` records search result: **no matches**.
- Claim boundary locked: "submission-ready" not "gallery-accepted" (PKP maintainer review required).

### 2. [x] Lock spec
- Scope defined: PKP Gallery listing, OJS fixture smoke, install documentation, permissions boundary, submission PR, disposable-instance smoke.
- Claim boundary: "submission-ready" not "gallery-accepted".
- Data contracts derived: gallery search URL, PR URL, plugin package structure, install archive format, fixture schema, live smoke transcript format.
- Evidence level target: `opt-in-live-proven`.
- Spec locked in `spec.md` with parallelization plan.

### 3. [x] Implement
- Plugin packaging reviewed against gallery-readiness checklist:
  - Plugin entry point (`index.php`): present.
  - Plugin XML metadata (`version.xml`): present.
  - Plugin descriptor (`plugin.xml`): present with disclaimer: "not PKP Plugin Gallery accepted."
  - Localisation (`locale/en_US/locale.po`): present (English).
  - Installation docs (`README.md`): present with install steps and boundary notes.
  - License (Apache-2.0/MIT): present.
  - Unique namespace (`sourceright`): confirmed no conflict.
- Permissions documentation: Track installation guide and plugin README document local processing, role visibility, and no silent canonical CSL overwrite.
- Policy test `tests/ojs_plugin_packaging_policy.rs` enforces package structure and lint checks.
- Build script `scripts/build-ojs-plugin-package.ps1` produces reproducible install archive with SHA-256 sidecar.

### 4. [x] Run checks
- `cargo test` — `ojs_fixture_screens_to_editor_and_author_outputs_end_to_end` passes.
- `cargo clippy --all-targets -- -D warnings` — no warnings.
- `cargo fmt --check` — no formatting issues.
- `tests/ojs_plugin_packaging_policy.rs` — package structure, lint, and manifest checks pass.
- `scripts/ojs-plugin-lint.ps1` — repo-local lint passes (PHP lint skipped when PHP not on PATH).
- Build archive smoke: `scripts/build-ojs-plugin-package.ps1` produces valid `.tar.gz` with SHA-256 sidecar.

### 5. [ ] PKP Gallery submission PR
- Fork `pkp/plugin-gallery` under `edithatogo` namespace.
- Add Sourceright plugin entry to gallery index (new entry or PR against `plugins.json` or equivalent).
- Use the raw `plugin.xml` URL from the latest release tag as the gallery source.
- Open PR against `pkp/plugin-gallery` with:
  - Plugin name: "Sourceright Reference Integrity Screening"
  - Description: OJS generic plugin for reference-integrity screening
  - Source URL: `https://raw.githubusercontent.com/edithatogo/sourceright/v{ver}/plugins/ojs/sourceright/plugin.xml`
- Record PR URL and status in updated `browser-gallery-verification-*.md`.

### 6. [ ] Disposable-instance smoke
- Ensure Docker daemon is running and WSL2 is available.
- Run `scripts/ojs-docker-install-smoke.ps1 -PluginVersion 0.1.0 -FetchPkpContainers` to stage PKP containers.
- Start OJS stack, install Sourceright plugin via generic plugin upload, run a screening test.
- Record live smoke transcript in `live-smoke-{date}.md`:
  - Container startup and OJS accessibility.
  - Plugin upload and activation.
  - Screening invocation (fixture or manual submission).
  - Report output verification.
- If Docker is unavailable locally, document the blocker and record partial preflight evidence.

### 7. [ ] conductor-review
- Gallery search evidence (`browser-gallery-verification-*.md`) reviewed:
  - Pre-PR: "no matches" confirmed.
  - Post-PR: PR URL and status recorded.
- Packaging review evidence (plugin structure, permissions, policy test) reviewed against gallery-readiness checklist.
- Disposable-instance smoke transcript reviewed for completeness.
- Claim boundary verified: no claim of "gallery-accepted" made.
- `$conductor-review` applied before any surface promotion.

### 8. [ ] Apply fixes
- Address any gallery PR review comments from PKP maintainers (if received within track window).
- Update plugin metadata, README, or packaging based on PR feedback.
- Rebuild install archive if needed.

### 9. [ ] Progress — Record listing evidence
- Update `browser-gallery-verification-*.md` with:
  - Gallery PR URL and status.
  - Disposable-instance smoke result (pass/fail/blocked).
- If gallery PR is merged, update `docs/src/release-status.md` and `docs-site` mirror to reflect OJS/PKP from "prepared" to "accepted".
- If PR is still open, keep OJS/PKP at "prepared" with a note linking to the pending PR.
- Commit all evidence docs.
