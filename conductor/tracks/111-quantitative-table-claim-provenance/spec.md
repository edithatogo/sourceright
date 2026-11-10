# Quantitative Table and Claim Provenance Spec

## Goal

Extend Sourceright's verification sidecar and provenance graph ([Track 14](../14-claim-source-provenance/spec.md), [Track 110](../110-scoped-evidence-consumers/spec.md)) with cell-level HTML/XLSX/CSV/PDF quantitative estimate, confidence-interval, relative-standard-error (`RSE`), and footnote/caution-flag provenance sidecars for official statistical and regulatory releases consumed by `edithatogo/tobac` (`F01`, `nz_au_hardening_20261106`) and `edithatogo/searchright` (`Track 42`, `Track 44`).

This is a planned extension, not a claimed runtime capability or epidemiological truth evaluator.

## User outcome

A quantitative policy model or manuscript audit can verify that every cited prevalence, seizure, or wastewater figure in a table or prose claim matches the exact archived source bytes (`archive-govt-nz` SHA-256), table/sheet/cell coordinates (`citeweft` `03-tabular-numeric-span-grounding`), reported uncertainty interval (`95% CI` or `RSE`), and attached statistical caution/suppression footnotes (`*`, `np`, `<LOQ`), without mutating canonical CSL records.

## Scope

- Reuse the Track 14 provenance graph and Track 110 scoped evidence bindings while keeping canonical CSL and legal citation models unchanged.
- Define an additive versioned `QuantitativeCellProvenance` sidecar record linking:
  - immutable source artifact SHA-256 and archive receipt reference (`archive-govt-nz`);
  - table/sheet identifier, row/column header lineage, and exact UTF-8 byte/cell span (`citeweft`);
  - parsed numeric point estimate, unit/denominator string, uncertainty interval (`lower`, `upper`, `confidence_level` or `rse_pct`), and suppression/caution footnote flags;
  - deterministic match status (`verified_exact`, `verified_rounded`, `footnote_caution_unpropagated`, `mismatch`, `not_run`).
- Flag downstream manuscript claims or model parameter tables that cite a point estimate while omitting an attached high-RSE, suppression, or methodological-break footnote in the source table.

## Out of scope

Epidemiological modelling, partial-identification math, and value-of-information calculations remain in `tobac` and `voiage`. Neutral layout/table tokenisation remains in `citeweft`. Raw government release capture remains in `archive-govt-nz`.

## Data contracts

| Record | Required distinctions |
| --- | --- |
| `TableCellCoordinate` | Document/sheet ID, table index, row/column header path, raw cell text, and UTF-8 byte range or page bounding box. |
| `QuantitativeClaimSidecar` | Point value, interval/RSE fields, unit/denominator text, attached footnote markers, source SHA-256, and deterministic check status. |
| `FootnotePropagationCheck` | Distinguishes clean numeric match from unpropagated caution/suppression/break footnotes (`RSE > 25%`, `<LOQ`, series break). |

## Claim boundary

Cell/span hash verification proves that a number and its footnote markers match the cited source table at the declared coordinate; it does not establish causal validity or survey representativeness.
