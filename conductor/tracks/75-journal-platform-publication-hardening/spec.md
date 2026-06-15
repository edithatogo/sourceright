# Journal-Platform Publication Hardening Spec

## Goal

Mature the OJS/PKP plugin package from a local source skeleton and fixture smoke into a gallery-ready submission with verified packaging, permissions documentation, fixture smoke, and optional live-instance proof. The end state is a confirmed PKP Plugin Gallery PR with a recorded listing evidence trace, not necessarily gallery acceptance.

## User outcome

Journal operators can find Sourceright in the PKP Plugin Gallery search results, verify the plugin metadata, and install the generic plugin package from a tagged GitHub release with documented permissions and fixture-backed screening evidence. Contributors can point to a recorded gallery submission PR and a disposable-instance smoke transcript as evidence of publication readiness.

## Scope

- **PKP Gallery search verification**: Browser probe of `https://github.com/pkp/plugin-gallery/search?q=sourceright` recorded as evidence.
- **OJS fixture smoke**: Existing `fixtures/journal/ojs-submission.json` exercised by `cli_end_to_end` OJS fixture test continues to pass.
- **Plugin packaging hardening**: `plugins/ojs/sourceright/` source skeleton reviewed for gallery-readiness criteria (entry point, XML metadata, localisation, install docs, license, namespace uniqueness).
- **Permissions documentation**: Plugin permissions and data boundaries documented in track installation guide and plugin README — local processing, role visibility, no silent canonical CSL overwrite.
- **Gallery submission PR**: Fork `pkp/plugin-gallery`, add Sourceright plugin entry, open PR, and record PR URL.
- **Disposable-instance smoke**: Run `scripts/ojs-docker-install-smoke.ps1` against a disposable OJS instance and record the live smoke transcript.
- **Listing evidence recording**: Gallery probe result, PR URL, and live smoke transcript committed as evidence docs.

## Out of scope

- PKP Plugin Gallery acceptance (maintainer review and merge).
- OJS compatibility matrix execution (3.3–3.5).
- Live OJS handler/settings-form/workflow-template wiring.
- REST/HTTP endpoint packaging for OJS.
- i18n beyond English locale.
- PHP unit tests for the plugin skeleton.
- Uninstall logic.
- Release tag creation (handled by release process, not this track).

## Data contracts

| Contract | Source | Format |
|---|---|---|
| PKP Plugin Gallery search | `https://github.com/pkp/plugin-gallery/search?q=sourceright` | Search result page (match count, snippet) |
| PKP Plugin Gallery PR | `https://github.com/pkp/plugin-gallery/pull/{id}` | PR URL, status (open/merged/closed) |
| OJS generic plugin package | `plugins/ojs/sourceright/` | PHP source skeleton: `index.php`, `SourcerightPlugin.php`, `version.xml`, `plugin.xml`, locale, README |
| Install archive | `scripts/build-ojs-plugin-package.ps1` → `dist/ojs/sourceright-ojs-generic-plugin-{version}.tar.gz` | `.tar.gz` with SHA-256 sidecar |
| OJS submission fixture | `fixtures/journal/ojs-submission.json` | JSON — 5 references with mixed integrity states |
| Live smoke transcript | `conductor/tracks/75-journal-platform-publication-hardening/live-smoke-{date}.md` | Markdown — transcript of disposable OJS instance install and screening run |
| Browser gallery verification | `conductor/tracks/75-journal-platform-publication-hardening/browser-gallery-verification-{date}.md` | Markdown — gallery search probe results |

## Claim boundary

**"Submission-ready" not "gallery-accepted".** This track may claim that the plugin package meets PKP Gallery submission requirements and that a PR has been opened, but it must not claim gallery acceptance until a PKP maintainer merges the PR and the plugin appears in the live PKP Plugin Gallery listing. All evidence docs must include the disclaimer: "This plugin is not PKP Plugin Gallery accepted. Gallery acceptance requires maintainer review and merge."

## Evidence level target

`opt-in-live-proven` — PKP Gallery search must be verified by a human browser probe at the live public URL. Disposable-instance smoke must be recorded as a live transcript for opt-in verification.

## Parallelization plan

- **Subagent A**: PKP Gallery search verification and browser evidence recording.
- **Subagent B**: Plugin packaging review (gallery-readiness checklist, permissions documentation, policy test audit).
- **Subagent C**: Gallery submission PR (fork, add plugin entry, open PR).
- **Subagent D**: Disposable-instance smoke (run OJS Docker smoke script, record transcript).

Subagents A and B can run in parallel. Subagent C depends on B (packaging must be reviewed before PR is opened). Subagent D can run independently.
