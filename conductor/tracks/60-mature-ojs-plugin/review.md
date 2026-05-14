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
| Plugin packaging | ❌ Not found | No OJS plugin directory, build script, or XML |
| Install docs | ❌ Not found | No OJS installation instructions |
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

2. **No OJS plugin packaging exists.** There is no PHP plugin directory, no `version.xml`, no `index.php` — the integration is CLI/MCP-based, not a traditional OJS PHP plugin.

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

## Status

- **Previous status**: planned
- **New status**: in_progress (screening engine is mature; OJS packaging, fixtures, and install docs needed)
