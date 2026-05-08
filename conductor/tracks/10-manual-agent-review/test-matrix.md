# Manual Agent Review Test Matrix

| Scenario | Expected result |
| --- | --- |
| Low-confidence match | Queue item includes original text and candidates. |
| Conflicting providers | Queue item includes field-level diffs. |
| Subagent partition | No record is assigned to multiple active reviewers unless explicitly allowed. |
| Decision import | Sidecar metadata records the decision and reviewer provenance. |
