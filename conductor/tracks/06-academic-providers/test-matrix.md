# Academic Providers Test Matrix

| Scenario | Expected result |
| --- | --- |
| DOI lookup | Exact provider candidate is recorded. |
| Bibliographic lookup | Ranked candidates are returned with confidence. |
| Provider outage | Deterministic retry/error metadata is recorded. |
| Conflicting provider data | No silent overwrite occurs. |
