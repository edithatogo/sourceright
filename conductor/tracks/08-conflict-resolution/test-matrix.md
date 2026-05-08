# Conflict Resolution Test Matrix

| Scenario | Expected result |
| --- | --- |
| Exact DOI match | High-confidence merge is allowed. |
| Title/year mismatch | Conflict is recorded and review is required. |
| Multiple candidates | Ranked candidate list is preserved. |
| Low confidence | No destructive canonical update occurs. |
