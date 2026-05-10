# Live Provider Adapters Test Matrix

| Scenario | Expected result |
| --- | --- |
| Default test run | Provider tests use fixtures and require no network access. |
| Missing credentials | Live smoke tests are skipped with clear reasons. |
| Unpaywall adapter | Live or recorded response produces sidecar evidence and provenance. |
| OpenCitations adapter | Citation evidence is captured without changing canonical CSL. |
| arXiv adapter | Preprint metadata is captured as provider evidence. |
| Europe PMC adapter | Biomedical metadata is captured as provider evidence. |
| Repository records | Repository metadata is captured with source and confidence fields. |
| Licensed provider | Bring-your-own-key configuration is required before live calls. |
| Conflict handling | Provider/canonical differences become conflicts or review items, not silent overwrites. |
