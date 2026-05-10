# Plugin Authoring

Sourceright plugin manifests are contract documents first. They should explain
what a plugin can do without implying that the current Rust binary can load it.

## Manifest Rules

Every manifest should include:

- a stable `id` using a category prefix, for example `provider.crossref`;
- `plugin_api = "sourceright.plugin.v1"`;
- a `category` from the registry category list;
- `status`, using conservative values such as `core_normalizer`,
  `planned`, `planned_byo_key`, or `planned_adapter`;
- authentication and network requirements;
- cache, licence, and data-retention expectations;
- output contracts that match the schemas in `schemas/`.

Provider plugins must not overwrite canonical CSL fields silently. They should
emit candidates, conflicts, provenance, confidence inputs, or review issues that
can be stored in the verification sidecar or derived reports.

Citation-manager and export plugins should start with dry-run manifests and file
contracts before any direct sync implementation. Journal plugins should consume
or produce `sourceright.journal_screening.v1` reports without making claims
about author intent.

## Test Expectations

Default plugin tests should be fixture-backed and deterministic. Live API checks
can exist later, but they must be opt-in and excluded from normal local and CI
test paths.
