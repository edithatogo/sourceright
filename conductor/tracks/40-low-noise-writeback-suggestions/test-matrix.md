# Low-Noise Writeback Suggestions Test Matrix

| Scenario | Expected coverage |
| --- | --- |
| Missing DOI with strong provider evidence | Suggests CSL patch without auto-apply. |
| Conflicting title evidence | Routes to review-required suggestion. |
| Broken URL with archive candidate | Suggests archive/update action with evidence. |
| Weak provider match | Suppresses or labels as low-confidence. |
| Citation-manager preview | Includes suggestions without remote writes. |
| Citation-sync schema contract | Documents emitted counters, suggestions, and explanations. |
| Schema inventory | Ensures public schemas are valid JSON, packaged, and documented. |
| Explicit apply | Writes audit log and preserves review traceability. |
