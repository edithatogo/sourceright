# Mature OJS Plugin Test Matrix

| Scenario | Acceptance | Evidence |
| --- | --- | --- |
| Package | OJS generic plugin source skeleton exists and has install instructions; Plugin Gallery acceptance is not claimed. | `plugins/ojs/sourceright/` contains `index.php`, `SourcerightPlugin.php`, `classes/SourcerightCliRunner.php`, `version.xml`, locale, and README. `tests/ojs_plugin_packaging_policy.rs` enforces required files and guardrails. |
| Screening | Fixture submission produces editor and author outputs. | `fixtures/journal/ojs-submission.json` is exercised by `tests/cli_end_to_end.rs::ojs_fixture_screens_to_editor_and_author_outputs_end_to_end`. |
| Permissions | Plugin permissions and data boundaries are documented. | Track installation guide and plugin README document local processing, role visibility, and no silent canonical CSL overwrite. |
| Live smoke | Optional OJS test instance smoke passes or skips cleanly. | Live OJS instance smoke remains opt-in and deferred; the default CI path is fixture-backed only. |
| Gallery | PKP/OJS gallery readiness is tracked separately from acceptance. | `gallery-readiness.md` marks source skeleton files present while keeping handler/settings/template wiring, compatibility testing, Gallery PR, and acceptance open. |
| Review loop | `$conductor-review` runs and local fixes are applied. | Review findings were integrated into source-package boundary docs and policy tests. |
