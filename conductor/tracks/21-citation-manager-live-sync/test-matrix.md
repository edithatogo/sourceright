# Citation Manager Live Sync Test Matrix

| Scenario | Expected result |
| --- | --- |
| Preview only | Sync returns create/update/skip/conflict plan without remote writes. |
| Explicit apply | Validated plan writes to Zotero only after explicit apply. |
| Missing credentials | Live sync is skipped or refused with a clear configuration error. |
| Local new record | Preview plans a Zotero create action. |
| Remote existing record | Preview plans update or skip according to match and diff rules. |
| Conflict | Local/remote disagreement produces a reviewable conflict, not a silent overwrite. |
| Audit log | Applied sync records target, action, remote id, timestamp, and result. |
| Default CI | Fixture-backed tests run without live Zotero access. |
