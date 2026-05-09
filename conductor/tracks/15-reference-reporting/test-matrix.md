# Reference Reporting Test Matrix

| Scenario | Expected result |
| --- | --- |
| Empty workspace | Report renders zero-reference summary. |
| Missing DOI | Warning issue is emitted and marked as an AI-risk signal. |
| No provider candidate | Warning issue is emitted and marked as an AI-risk signal. |
| Provider conflict | Warning issue is emitted and conflict count increases. |
| Manual review queued | Informational issue is emitted without claiming AI risk. |
| CSL boundary violation | Error issue is emitted and marked as an AI-risk signal. |
