---
title: Citation manager integrations
description: Citation manager sync and profile support.
---

Citation manager integrations focus on previewable, auditable sync paths.

- Keep profile mappings explicit.
- Require preview before apply for write paths.
- Treat `suppressed` and `review_required` suggestions as review controls, not
  automatic writes.
- Surface `suppressed_count`, `review_required_count`, suggestion class, and
  explanation text in preview/apply JSON.

## Zotero Fixture-Backed Testing

Three fixture files have been created at `fixtures/providers/zotero/` to enable
deterministic, reproducible testing of the citation-sync engine without
requiring a live Zotero instance.

### Fixture Overview

| Fixture | File | Scenario | Key Test Assertions |
|---------|------|----------|---------------------|
| Preview Exact Match | `preview-exact-match.json` | CSL reference has same DOI and same title as a Zotero item | `skip_count=1`; action is `Skip` with `NoOp` suggestion; confidence approximately 0.99 |
| Preview Title Update | `preview-title-update.json` | CSL reference has same DOI but different title from Zotero item | `update_count=1`; action is `Update` with `SafeUpdate` suggestion; field diff is captured |
| Apply Success Preview | `apply-success-preview.json` | Full apply run: 1 exact match skipped, 1 title updated, 1 new item created | `applied=true`; `skip_count=1`, `update_count=1`, `create_count=1`; audit log written |

### Fixture Format

Each fixture file is a JSON array of Zotero item objects as returned by the
Zotero Web API `/items` endpoint. Items include `key`, `version`, `data`
containing title, creators, DOI, date, publication title, and related metadata.

The `apply-success-preview.json` fixture is a full
`sourceright.citation_sync.v1` report object, not a raw API response. It
includes the complete report with `actions`, `config`, and `audit_log_path`.

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

- DOI-based exact matching.
- Title-based narrow-fit matching.
- Action classification: Skip, Update, Create, and Conflict.
- Suggestion confidence scoring.
- Apply-mode audit log generation.
- Suppressed and review-required edge cases through future fixtures.

### Environment Gating

Live Zotero API calls are controlled by environment variables. Fixture-based
tests run without any environment variables and work in CI:

```text
cargo test citation_sync
```

### Future Fixture Scenarios

| Fixture | Scenario | Status |
|---------|----------|--------|
| `preview-narrow-fit.json` | Shared title tokens, no DOI match | Planned |
| `preview-doi-conflict.json` | Same DOI, different titles with author overlap | Planned |
| `preview-mixed-scenario.json` | Multiple items exercising all action types | Planned |
| `apply-create-item.json` | Pure create scenario audit | Planned |
| `edge-suppressed-near-match.json` | Weak title overlap suppressed | Planned |
| `edge-review-required.json` | Narrow lexical match flagged for review | Planned |
