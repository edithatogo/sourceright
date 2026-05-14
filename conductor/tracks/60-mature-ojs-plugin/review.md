# Track 60 — Mature OJS Plugin: Review

## Current State

### Documentation Review

**Spec** requirements:
- Platform-neutral screening contract remains the core
- OJS plugin packaging, permissions, install docs, and configuration are explicit
- Fixtures prove editor-facing and author-facing outputs
- Optional test-instance smoke proves installation and screening flow
- PKP/OJS Plugin Gallery readiness tracked separately from acceptance

**Plan**:
1. Verify current OJS/PKP plugin packaging and gallery requirements
2. Define plugin architecture that calls the Sourceright CLI/MCP core
3. Add fixtures for submission metadata, manuscript text, editor report, and author checklist
4. Add package/install smoke and optional OJS test-instance smoke
5. Add Plugin Gallery submission checklist and docs

**Test Matrix**:
| Scenario | Acceptance |
|----------|-----------|
| Package | OJS plugin package builds and has install instructions |
| Screening | Fixture submission produces editor and author outputs |
| Permissions | Plugin permissions and data boundaries are documented |
| Live smoke | Optional OJS test instance smoke passes or skips cleanly |
| Gallery | PKP/OJS gallery readiness tracked separately |

### Codebase Evidence

| Component | Status | Notes |
|-----------|--------|-------|
| OJS manifest | ✅ `planned_adapter` | `plugins/manifests/journal.ojs.toml` |
| Journal screening engine | ✅ Mature | `src/journal.rs` (187 lines) — `JournalScreeningReport`, `JournalPlatform::Ojs`, `screen_journal_submission()` |
| Editor-facing output | ✅ | `JournalScreeningReport` has `editorial_summary` field |
| Author-facing output | ✅ | `JournalScreeningReport` has `author_action_checklist` field |
| OJS submission fixture | ✅ **NEW - 2026-05-14** | `fixtures/journal/ojs-submission.json` — mixed integrity states, 5 references |
| Gallery readiness doc | ✅ **NEW - 2026-05-14** | `conductor/tracks/60-mature-ojs-plugin/gallery-readiness.md` — gaps + requirements for PKP Gallery |
| Test matrix update | ✅ **NEW - 2026-05-14** | `conductor/tracks/60-mature-ojs-plugin/test-matrix-update.md` — maps fixture scenarios to acceptance criteria |
| Plugin packaging | ✅ Source skeleton | `plugins/ojs/sourceright/` contains an OJS generic plugin source skeleton with `index.php`, main class, CLI runner, `version.xml`, README, and locale file. |
| Install docs | ✅ Source skeleton install docs | `plugins/ojs/sourceright/README.md` and `installation-guide.md` document source-tree install and CLI/MCP service boundaries. |
| Test-instance smoke | ❌ Not found | No OJS test instance smoke script |

### Journal Screening Engine Review (`src/journal.rs`)

The journal screening module is well-structured and OJS-ready:

- **`JournalScreeningRequest`** — submission_id, platform (Ojs, ScholarOne, etc.), manuscript_label
- **`JournalScreeningReport`** — schema_version, editorial_summary, author_action_checklist, reference_report
- **`JournalScreeningStatus`** — Accepted, Screened, BlockedForExtraction, ScreenedWithWarnings, ScreenedWithErrors
- **`screen_journal_submission()`** — takes CSL + sidecar, produces full screening report
- **Tests**: 2 unit tests verify editorial/author outputs and blocked-for-extraction behavior

The `JournalPlatform::Ojs` variant is already defined and used in tests.

### Key Findings

1. **Journal screening engine is mature and OJS-ready.** The `journal.rs` module produces editor-facing summaries and author-facing checklists that match OJS workflow requirements.

2. **OJS source packaging now exists, but Gallery acceptance is not claimed.** The thin PHP generic-plugin skeleton shells out to the Sourceright CLI/MCP core and keeps writes previewed or explicit. Live OJS handler/settings/template wiring and PKP Plugin Gallery review remain open.

3. **OJS submission fixture created.** `fixtures/journal/ojs-submission.json` covers 5 references with mixed integrity states: verified, provider conflict, retracted, queued, and missing DOI. The fixture exercises all relevant branches of the screening pipeline.

4. **Gallery readiness doc created.** `conductor/tracks/60-mature-ojs-plugin/gallery-readiness.md` documents all PKP Plugin Gallery requirements, current gaps, integration architecture (Path A: PHP wrapper + CLI), and a submission process checklist. 12 gaps identified with effort estimates.

5. **Test matrix update created.** `conductor/tracks/60-mature-ojs-plugin/test-matrix-update.md` maps OJS fixture scenarios to specific test acceptance criteria for future automation.

6. **Plugin manifest accurately reflects status** at `planned_adapter`.

## Recommendations

1. **Document the integration architecture.** The current approach uses Sourceright CLI/MCP as a screening service that OJS calls via webhook or CLI. This should be explicitly documented.

2. **Create OJS fixture data:**
   - `fixtures/ojs/submission-metadata.json` — submission_id, title, authors, platform
   - `fixtures/ojs/editor-report.json` — expected editorial output
   - `fixtures/ojs/author-checklist.json` — expected author action items

3. **Consider OJS plugin packaging** — if a traditional OJS plugin is desired:
   - Create `plugins/ojs/` directory with PHP entry points
   - Create `version.xml` for PKP Plugin Gallery
   - Document installation steps
   - Otherwise document CLI/webhook approach as the integration method

4. **Update `docs/src/journal-integrations.md`** with OJS integration guidance.

5. **Defer PKP Gallery submission** until packaging is built and tested.

## Progress Evidence (2026-05-14)

### Acceptance Criteria Assessment

| Scenario | Acceptance | Status | Evidence |
|----------|-----------|--------|----------|
| **Package** | OJS plugin package builds and has install instructions | 🔵 Partial | `installation-guide.md` documents the CLI/MCP service path. A PHP plugin wrapper/package is still required before claiming an installable OJS/PKP plugin. |
| **Screening** | Fixture submission produces editor and author outputs | ✅ Completed | `fixtures/journal/ojs-submission.json` exercises 5 reference states. `src/journal.rs` produces `editorial_summary` and `author_action_checklist`. Test matrix update (`test-matrix-update.md`) maps 24 test scenarios across intake, individual reference states, summary integrity, and editorial/author outputs. |
| **Permissions** | Plugin permissions and data boundaries are documented | ✅ Completed | `installation-guide.md` section 4 documents OJS role-based access (6 roles), data boundaries (5 data types with read/write/persistence), privacy and processing rules. |
| **Live smoke** | Optional OJS test instance smoke passes or skips cleanly | 🔵 Deferred | Documented as a limitation in `installation-guide.md` section 6.2 with workaround (manual testing on target instance). |
| **Gallery** | PKP/OJS gallery readiness tracked separately | ✅ Completed | `gallery-readiness.md` documents 12 gaps with effort estimates, Path A/B recommendations, and submission process. |

### Architecture Decision

The integration currently follows the **CLI/MCP service model** (not a traditional OJS PHP plugin). This is documented in `installation-guide.md` section 1 and aligns with the gallery-readiness recommendation (Path B for service, Path A deferred for Gallery submission).

### Files Created/Updated

| File | Action | Purpose |
|------|--------|---------|
| `conductor/tracks/60-mature-ojs-plugin/installation-guide.md` | **NEW** | Architecture decision, installation steps, integration methods (CLI/MCP/batch), permission boundaries, configuration reference, current status, troubleshooting, uninstallation |
| `conductor/tracks/60-mature-ojs-plugin/metadata.json` | Updated | Status remains `in_progress` until PHP packaging or an equivalent installable OJS package exists |

### Existing Assets (Verified)

- `plugins/manifests/journal.ojs.toml` — `planned_adapter` status
- `src/journal.rs` — `JournalPlatform::Ojs`, `screen_journal_submission()`, `editorial_summary`, `author_action_checklist`
- `fixtures/journal/ojs-submission.json` — 5 references with mixed integrity states
- `conductor/tracks/60-mature-ojs-plugin/gallery-readiness.md` — PKP Gallery gaps and Path A/B
- `conductor/tracks/60-mature-ojs-plugin/test-matrix-update.md` — 24 test scenarios mapped to acceptance criteria
- `docs/src/journal-integrations.md` — Platform-neutral integration contract

### Completion Update (2026-05-15)

Track 60 is complete at the repo-local evidence level: installable OJS generic
plugin source skeleton plus fixture-backed screening. The completion claim is
deliberately narrower than PKP Plugin Gallery acceptance.

| Evidence | Status |
|----------|--------|
| OJS source skeleton | Completed in `plugins/ojs/sourceright/` |
| Install-test archive | `scripts/build-ojs-plugin-package.ps1` builds `dist/ojs/sourceright-ojs-generic-plugin-0.1.0.tar.gz` with SHA-256 sidecar |
| CLI runner safety | Escaped command arguments, configurable CLI path, timeout, preview-only export |
| Editor/author separation | Editor output receives summary/full report; author output receives checklist |
| Fixture proof | `fixtures/journal/ojs-submission.json` covered by CLI end-to-end test |
| Policy proof | `tests/ojs_plugin_packaging_policy.rs` enforces package files and guardrails |
| Smoke path | `ojs-install-smoke.md` documents manual and Docker OJS test routes; local preflight detected Docker 29.4.2 and Compose v5.1.3 and staged `pkp/containers` under `C:\tmp\sourceright-ojs-smoke\containers` |
| Gallery acceptance | Not claimed; readiness remains prepared/deferred until external review |

### Remaining Work (Post-Completion)

| Item | Owner | Priority |
|------|-------|----------|
| Live OJS handler/settings/workflow-template wiring | Future OJS implementation | Medium |
| REST/HTTP endpoint for OJS | Track 60 | Medium |
| OJS test-instance smoke transcript | Track 45 / manual Docker OJS test | Low |
| PKP Gallery submission and acceptance | Track 69 / external marketplace evidence | Low |
| Localisation / i18n | Track 60 | Low |

## Status

- **Previous status**: planned
- **New status**: completed (source skeleton and fixture-backed screening are complete; OJS live wiring and PKP Gallery acceptance are not claimed)
