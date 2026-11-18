# CSL Canonical Model Spec

## Goal

Use CSL JSON as the canonical academic-reference representation.

## Scope

- Define Rust models or validated serde-compatible structures for CSL JSON.
- Preserve style-neutral reference data.
- Keep verification metadata out of CSL records.

## Outputs

- `references.csl.json`.
- Validation errors with stable machine-readable codes.

## Serde dependency assessment — 18 November 2026 (bounded follow-up)

The crate already declares `serde` derive, `serde_json`, and the maintained Serde-compatible `serde-saphyr` for YAML task/benchmark manifests. `src/csl.rs` models CSL JSON; `src/bench.rs` deserializes `BenchmarkManifest` with `serde_saphyr::from_str`. `roxmltree` and HTTP clients retain separate semantics. **No migration to Serde is needed; Serde is the existing abstraction.** Do not replace the XML tree parser, YAML format adapter, canonical CSL or verification sidecars with traits.

- [ ] In existing track 04/verification work, freeze CSL and YAML parser fixtures including unknown fields, YAML duplicate keys, aliases, numeric precision, Unicode, malformed text, citation mapping and errors.
- [ ] Compare any stable CSL DTO boundary to actual Searchright/CiteLattice cross-repo exchanges; preserve style-neutral bibliographic fields and keep evidence/rights out of CSL itself.
- [ ] Audit for *actual* repeated handcrafted JSON; prefer typed Serde DTOs when the schema is stable but keep forward-compatible external extensions lossless. Reject a replacement that breaks cited historical outputs or introduces an unneeded codec.
- [ ] Only after independent exact-head tests/security/consumer acceptance should a bounded change be proposed; this addendum is a no-go against broad Serde migration and no change to prior completion claims.
