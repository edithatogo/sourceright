# Live Core Provider Verification Test Matrix

| Scenario | Expected coverage |
| --- | --- |
| Crossref DOI match | Adds sidecar evidence without changing CSL. |
| DataCite dataset match | Preserves dataset-specific metadata as provider evidence. |
| OpenAlex title/DOI match | Records confidence inputs and candidate identifiers. |
| PubMed/NCBI record | Captures PMID/biomedical metadata for review. |
| DOI resolver reachable | Records reachability evidence. |
| Provider no-match/outage | Emits deterministic diagnostics and leaves canonical data unchanged. |
