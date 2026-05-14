# Citation Manager Integrations

Sourceright already exports YAML, XML, RIS, ENW, and BibLaTeX. Citation-manager
integration should build on those file surfaces first, then add dry-run API sync
contracts before any live mutation.

Profiles under `examples/citation-manager-profiles/` describe expected adapter
behavior for:

- Zotero
- Mendeley
- EndNote
- Papers/ReadCube
- JabRef
- RevMan
- Rayyan
- Covidence

## Boundaries

Canonical CSL remains in `references.csl.json`. Provider evidence, review
decisions, sync decisions, and conflicts stay outside CSL in sidecars or sync
manifests.

Direct API adapters should default to dry-run, declare auth requirements, avoid
credential storage, and cache only caller-approved metadata. File-format
adapters should record exactly which files they would write or import.

The initial sync manifest schema is `sourceright.sync_manifest.v1`.
The current Zotero preview/apply report schema is
`sourceright.citation_sync.v1`.

## Zotero Preview And Apply

The first live sync target is Zotero. Sourceright models Zotero sync as a
preview-first contract:

- preview plans create, update, skip, and conflict actions without writes;
- preview actions include a `suggestion` class and reviewer-facing
  `explanation`;
- weak near-matches can be `suppressed`, and narrow conflicts can be
  `review_required`;
- `suppressed_count` and `review_required_count` are first-class counters in
  the report JSON;
- explicit apply is required before any remote mutation;
- applied runs append an audit log;
- ambiguous updates are reported as conflicts rather than silently overwritten.

Apply mode writes only low-confidence creates and safe updates. Suppressed
near-matches and review-required suggestions remain visible in the report and
audit trail, but they are not converted into remote writes.

The CLI surface is:

```text
sourceright citation-sync [--preview|--apply] [--remote-fixture <remote.json>] [--audit-log <audit.jsonl>] [.sourceright-directory]
```

By default the command reads `.sourceright`, runs in preview mode, and prints
`sourceright.citation_sync.v1` JSON. Live Zotero sync is disabled unless the
caller supplies the relevant `SOURCERIGHT_ZOTERO_*` environment variables.

## Zotero Fixture-Backed Testing

Three fixture files have been created at `fixtures/providers/zotero/` to enable
deterministic, reproducible testing of the citation-sync engine without
requiring a live Zotero instance.

### Fixture Overview

| Fixture | File | Scenario | Key Test Assertions |
|---------|------|----------|---------------------|
| Preview Exact Match | `preview-exact-match.json` | CSL reference has same DOI **and** same title as a Zotero item | `skip_count=1`; action is `Skip` with `NoOp` suggestion; confidence ~0.99 |
| Preview Title Update | `preview-title-update.json` | CSL reference has same DOI but **different title** from Zotero item | `update_count=1`; action is `Update` with `SafeUpdate` suggestion; field diff is captured |
| Apply Success Preview | `apply-success-preview.json` | Full apply run: 1 exact match skipped, 1 title updated, 1 new item created | `applied=true`; `skip_count=1`, `update_count=1`, `create_count=1`; audit log written |

### Fixture Format

Each fixture file is a JSON array of Zotero item objects as returned by the
[Zotero Web API /items endpoint](https://www.zotero.org/support/dev/web_api/v3/basics).
Items include `key`, `version`, `data` (containing title, creators, DOI, date,
publication title, etc.), and `meta` metadata.

The `apply-success-preview.json` fixture is a full `sourceright.citation_sync.v1`
report object, not a raw API response. It includes the complete report with
`actions`, `config`, and `audit_log_path`.

### Using Fixtures in Tests

Fixtures are loaded via the `--remote-fixture` CLI flag:

```text
sourceright citation-sync --remote-fixture fixtures/providers/zotero/preview-exact-match.json .sourceright
```

Or programmatically via `CitationSyncConfig::remote_fixture_path`:

```rust
let config = CitationSyncConfig {
    remote_fixture_path: Some(Path::new("fixtures/providers/zotero/preview-exact-match.json").to_path_buf()),
    ..Default::default()
};
let report = run_citation_sync(&workspace, config)?;
assert!(report.preview);
assert_eq!(report.skip_count, 1);
```

### Regression Assurance

These fixtures protect against regressions in:

- DOI-based exact matching
- Title-based narrow-fit matching
- Action classification (Skip / Update / Create / Conflict)
- Suggestion confidence scoring
- Apply-mode audit log generation
- Suppressed and review-required edge cases (via future fixtures)

### Environment Gating

Live Zotero API calls are controlled by environment variables. Fixture-based
tests run without any environment variables and work in CI:

```text
# No env vars needed — uses remote fixture
cargo test citation_sync
```

### Future Fixture Scenarios

Planned additions to `fixtures/providers/zotero/`:

| Fixture | Scenario | Status |
|---------|----------|--------|
| `preview-narrow-fit.json` | Shared title tokens, no DOI match | Planned |
| `preview-doi-conflict.json` | Same DOI, different titles with author overlap | Planned |
| `preview-mixed-scenario.json` | Multiple items exercising all action types | Planned |
| `apply-create-item.json` | Pure create scenario audit | Planned |
| `edge-suppressed-near-match.json` | Weak title overlap suppressed | Planned |
| `edge-review-required.json` | Narrow lexical match flagged for review | Planned |
