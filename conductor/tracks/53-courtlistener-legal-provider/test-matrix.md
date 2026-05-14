# CourtListener Legal Provider Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Legal model | Evidence writes legal reports, not academic CSL. |
| Matching | Neutral citation, court, date, and jurisdiction are mapped deterministically. |
| Ambiguity | Multiple possible cases route to legal review. |
| Live smoke | Optional and skip-safe. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
