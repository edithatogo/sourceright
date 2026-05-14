# Zotero Fixtures

This directory holds deterministic Zotero fixture files for the `citation-sync`
preview/apply/audit engine.

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

Preferred regression fixtures use Sourceright's compact remote-record shape:

```json
[
  {
    "key": "ABC123DEF",
    "item_type": "article-journal",
    "title": "Example Publication Title",
    "doi": "10.1234/example.2025.001"
  }
]
```

The loader also accepts captured Zotero Web API item responses where metadata is
nested under `data`:

```json
[
  {
    "key": "ABC123DEF",
    "version": 1234,
    "data": {
      "key": "ABC123DEF",
      "itemType": "journalArticle",
      "title": "Example Publication Title",
      "DOI": "10.1234/example.2025.001"
    }
  }
]
```

## Planned Fixture Scenarios

### Preview scenarios

| Fixture file | Scenario | Expected actions |
|---|---|---|
| `zotero-exact-match.json` | Same DOI + title | 1 Skip (NoOp) |
| `zotero-title-update.json` | Same DOI, different title | 1 Update (SafeUpdate) |
| `zotero-empty.json` | DOI not in Zotero | 1 Create |
| `preview.narrow-fit.json` | Shared tokens, no DOI | 1 LowConfidence or Suppressed |
| `preview.doi-conflict.json` | Same DOI, different titles | 1 Conflict |
| `preview.mixed-scenario.json` | Multiple items, all action types | Multiple actions |

### Apply scenarios

| Fixture file | Scenario | Expected behavior |
|---|---|---|
| `apply-success-preview.json` | CSL item absent from Zotero | Create written to audit log in apply-mode tests |

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
3. Capture a small `/items` response from the Zotero API, or write the compact
   `RemoteCitationRecord` JSON shape directly.
4. Trim the fixture to the desired scenario and remove unrelated personal data.
5. Place the fixture file in this directory and add a fixture-backed test.

## Usage in Tests

Rust integration tests can load fixtures as follows:

```rust
let fixture_path = Path::new("fixtures/providers/zotero/zotero-exact-match.json");
let config = CitationSyncConfig {
    remote_fixture_path: Some(fixture_path.to_path_buf()),
    ..Default::default()
};
let report = run_citation_sync(config)?;
assert!(report.preview);
assert_eq!(report.skip_count, 1);
```
