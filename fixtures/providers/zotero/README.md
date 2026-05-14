# Zotero Fixtures

This directory will hold JSON fixture files representing Zotero Web API (v3) responses
for deterministic testing of the `citation-sync` preview/apply/audit engine.

> **Note:** No fixture files are checked in yet. Zotero fixture creation requires
> capturing live API responses from a running Zotero instance. See
> [How to create fixtures](#how-to-create-fixtures) below.

## Purpose

Fixture files serve as deterministic remote-record snapshots for the
`CitationSyncConfig::remote_fixture_path` field. When `--remote-fixture <file.json>`
is passed, the sync engine reads items from the fixture instead of making live HTTP
calls to the Zotero API. This enables:

- Regression tests that do not require a live Zotero instance.
- Reproducible preview/apply comparison scenarios.
- CI pipeline runs without API credentials.
- Edge case coverage for match strategies (DOI exact, title exact, narrow fit, prefix conflict).

## Fixture Format

Each fixture file is a JSON array of Zotero item objects as returned by the
[Zotero Web API /items endpoint](https://www.zotero.org/support/dev/web_api/v3/basics).
Each item has the following structure:

```json
{
  "key": "ABC123DEF",
  "version": 1234,
  "data": {
    "key": "ABC123DEF",
    "itemType": "journalArticle",
    "title": "Example Publication Title",
    "creators": [
      { "firstName": "Jane", "lastName": "Doe" }
    ],
    "DOI": "10.1234/example.2025.001",
    "date": "2025-01-15",
    "extra": "",
    "tags": [],
    "collections": [],
    "relations": {},
    "notes": [],
    "dateAdded": "2025-01-15T10:00:00Z",
    "dateModified": "2025-01-15T10:00:00Z"
  }
}
```

## Planned Fixture Scenarios

### Preview scenarios

| Fixture file | Scenario | Expected actions |
|---|---|---|
| `preview.single-exact-match.json` | Same DOI + title | 1 Skip (NoOp) |
| `preview.title-update-needed.json` | Same DOI, different title | 1 Update (SafeUpdate) |
| `preview.create-needed.json` | DOI not in Zotero | 1 Create |
| `preview.narrow-fit.json` | Shared tokens, no DOI | 1 LowConfidence or Suppressed |
| `preview.doi-conflict.json` | Same DOI, different titles | 1 Conflict |
| `preview.mixed-scenario.json` | Multiple items, all action types | Multiple actions |

### Apply scenarios

| Fixture file | Scenario | Expected behavior |
|---|---|---|
| `apply.create-item.json` | CSL item absent from Zotero | Create written to audit log |
| `apply.update-item.json` | CSL item with updated title | Update written to audit log |
| `apply.skip-noop.json` | CSL item identical to Zotero | Skip recorded |

### Edge case scenarios

| Fixture file | Scenario | Expected behavior |
|---|---|---|
| `edge.suppressed-near-match.json` | Weak title overlap, no DOI | Suppressed, no write |
| `edge.review-required.json` | Narrow lexical match | ReviewRequired |
| `edge.multiple-creators.json` | Items with 10+ creators | Correct matching |
| `edge.empty-fields.json` | Zotero item with minimal metadata | Graceful handling |

## How to Create Fixtures

1. Start Zotero desktop (local API at `http://127.0.0.1:23119`).
2. Set your API key and library environment variables.
3. Run: `sourceright citation-sync --preview --remote-fixture ./captured-items.json`
4. Trim the captured fixture to the desired scenario.
5. Place the fixture file in this directory.

## Usage in Tests

Rust integration tests can load fixtures as follows:

```rust
let fixture_path = Path::new("fixtures/providers/zotero/preview.single-exact-match.json");
let config = CitationSyncConfig {
    remote_fixture_path: Some(fixture_path.to_path_buf()),
    ..Default::default()
};
let report = run_citation_sync(config)?;
assert!(report.preview);
assert_eq!(report.skip_count, 1);
```
