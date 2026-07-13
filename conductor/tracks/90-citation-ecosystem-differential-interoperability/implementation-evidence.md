# Track 90 implementation evidence

## Differential harness slice — 2026-07-12

Implemented:

- `src/interoperability.rs` with deterministic CSL semantic comparison.
- `interoperability-diff` CLI binary for JSON output and optional Markdown
  reports.
- `schemas/sourceright.interoperability-report.schema.json`.
- `optional-runners.toml` defining pinned-revision, lockfile, license, secret,
  temporary-output, and resource-limit gates for future external runners.
- Policy tests for report schema and runner deferral.

The comparator classifies differences as information loss, unsupported
construct, parser discrepancy, or review required. It never writes canonical
CSL or verification sidecars.

Static validation passed: `cargo fmt --all --check`, `git diff --check`,
workflow harness, `actionlint`, and release parity checks.

The former local build blocker is now resolved when using the installed GNU
toolchain and `C:\tmp\sourceright-target`. Citation.js 0.8.1 and
`@retorquere/bibtex-parser` 10.0.0 are lockfile-pinned, npm-audited, and the
protected-casing Citation.js conformance smoke passes. LGPL biblatex-csl and
CPAL/AGPL citeproc-js remain isolated pending legal review.

The complete local validation command also passes on 2026-07-12: all locked
Rust tests, clippy with `-D warnings`, locked check, schema inventory,
workflow harness, release parity, cargo-audit, cargo-deny, zizmor, and npm
audit. The LGPL/CPAL/AGPL items are explicit adoption gates rather than
failed checks.

The fixture corpus now includes a self-authored adversarial BibTeX case for
macros, nested/protected casing, month parsing, multiple authors, and
publisher fields. Citation.js produces an equivalent CSL observation with no
semantic differences; the fixture remains an oracle observation and cannot
write canonical CSL or verification state.

The optional `interoperability.yml` workflow and
`scripts/run-interoperability-matrix.ps1` now execute the basic BibTeX,
adversarial BibTeX, and RIS matrix with retained JSON/Markdown artifacts.
Workflow syntax, harness invariants, and Rust formatting pass after adding
the lane.
