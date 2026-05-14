# CourtListener Legal Provider Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Legal model | Evidence writes legal reports, not academic CSL. |
| Matching | Neutral citation, court, date, and jurisdiction are mapped deterministically. |
| Ambiguity | Multiple possible cases route to legal review. |
| Connector boundary | MCP and docs describe Sourceright as legal citation audit/enrichment, not legal advice or outcome prediction. |
| Live smoke | Optional and skip-safe. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
