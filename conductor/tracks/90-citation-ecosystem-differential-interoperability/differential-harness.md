# Differential harness implementation

Implemented 2026-07-12:

- `src/interoperability.rs` provides `compare_csl_json` and the stable report
  schema `sourceright.interoperability_report.v1`.
- `interoperability-diff` is a CLI binary that reads canonical and oracle JSON,
  emits deterministic pretty JSON, and optionally writes Markdown.
- Semantic comparison normalizes whitespace and compares type, title, DOI,
  container, authors, dates, URLs, and item counts.
- Differences are classified as `information_loss`,
  `unsupported_construct`, `parser_discrepancy`, or `review_required`.
- Oracle data is observations only: the comparator has no workspace or
  sidecar-write path.
- `optional-runners.toml` defines the security and provenance requirements for
  future Citation.js, biblatex-csl-converter, and BibTeX parser runners without
  pretending that unpinned tools are enabled.

Example:

```powershell
cargo run --locked --bin interoperability-diff -- `
  fixtures/interoperability/bibtex-basic.expected-csl.json `
  fixtures/interoperability/bibtex-basic.expected-csl.json `
  self-authored-fixture `
  interoperability-report.md
```

The command exits zero only for an equivalent comparison and exits one when
classified differences exist, making it suitable for optional CI artifacts.
