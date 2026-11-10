# Quantitative Table and Claim Provenance Test Matrix

All scenarios are future implementation acceptance checks. Their presence is not execution evidence. Fixtures must be synthetic and network-free.

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Exact cell and 95% CI match | Sidecar records `verified_exact` with matching source SHA-256, header path, point estimate, and interval bounds. | Synthetic statistical table fixture. | Default-CI, pending |
| Declared rounding tolerance match | Source cell `14.24%` cited as `14.2%` under declared 1-decimal rounding passes as `verified_rounded`; undeclared discrepancy fails. | Rounding-policy fixture. | Default-CI, pending |
| Unpropagated high-RSE or `<LOQ` footnote | Point value matches, but source cell carries `*` (`RSE > 25%`) or `<LOQ` omitted by consumer; check emits `footnote_caution_unpropagated`. | Footnote-hazard fixture. | Default-CI, pending |
| Transposed row/column or wrong subgroup header | Rejects citation when numeric value appears in an adjacent subgroup column (`Māori` vs `Total`) rather than the claimed header path. | Header-lineage negative fixture. | Default-CI, pending |
| Source hash drift or modified table bytes | Invalidates cached sidecar verification when source SHA-256 or scoped view digest changes. | Hash-invalidation regression. | Default-CI, pending |
