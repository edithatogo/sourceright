# Quantitative Table and Claim Provenance Plan

Status: **planned**, 10 November 2026. All implementation work below is pending.
Follow the [workflow](../../workflow.md), [requirements](../../requirements.md), and [test matrix](test-matrix.md).

## Discover and lock

- [ ] Align the additive `QuantitativeCellProvenance` sidecar schema with `citeweft` (`03-tabular-numeric-span-grounding`), `archive-govt-nz` (`tobac_merkle_lineage_provenance_20261110`), and `tobac` (`F01` in `conductor/tracks/nz_au_hardening_20261106/`).
- [ ] Lock synthetic fixtures covering HTML, CSV/XLSX, and PDF statistical tables with 95% confidence intervals, high-RSE caution asterisks, `<LOQ` censoring markers, and series-break footnotes.

## Implement and check

- [ ] Add deterministic table-cell numeric, interval, and footnote-marker verification to the sidecar provenance checker without altering canonical CSL records.
- [ ] Emit explicit `footnote_caution_unpropagated` diagnostics when a downstream claim or parameter table cites a flagged cell without carrying its caution/suppression metadata.
- [ ] Run offline fixture tests verifying exact match, tolerance-bounded rounding match, transposed-cell mismatch rejection, and missing-footnote detection.

## Review, migration and progression

- [ ] Review the bounded diff against the Track 14 and Track 110 provenance boundaries.
- [ ] Record producer/consumer fixture parity receipts with `citeweft` and `tobac` before promoting the track status beyond `planned`.
