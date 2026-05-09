# Verification Sidecar Test Matrix

| Scenario | Expected result |
| --- | --- |
| Provider match | Candidate, confidence, and source are recorded. |
| Conflicting providers | Conflict is preserved with enough data for review. |
| Manual decision | Accepted/rejected/merged state is recorded. |
| Provider candidate invariant | Provider candidates without provider identity, timestamp, bounded confidence, or data are flagged. |
| Conflict invariant | Conflict entries without field, severity, and provider/source are flagged. |
| Review transition | Review decisions move records through explicit allowed status transitions and reject invalid jumps. |
| Review queue JSONL | Queue output is derived from sidecar records needing review, sorted deterministically by CSL id, and excludes resolved/not-required records. |
| Export | Clean reference exports do not include sidecar internals. |
| Sidecar parse/format | Parses verification sidecar JSON and writes deterministic, newline-terminated pretty JSON. |
| Sidecar version validation | Unsupported schema versions produce stable sidecar diagnostics. |
