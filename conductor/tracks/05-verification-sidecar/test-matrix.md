# Verification Sidecar Test Matrix

| Scenario | Expected result |
| --- | --- |
| Provider match | Candidate, confidence, and source are recorded. |
| Conflicting providers | Conflict is preserved with enough data for review. |
| Manual decision | Accepted/rejected/merged state is recorded. |
| Export | Clean reference exports do not include sidecar internals. |
