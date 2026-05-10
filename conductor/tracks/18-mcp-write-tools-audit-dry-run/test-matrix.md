# MCP Write Tools Audit Dry-Run Test Matrix

| Scenario | Expected result |
| --- | --- |
| Default write request | Tool returns a dry-run change plan and does not modify files. |
| Explicit apply | Tool applies only a validated plan after explicit apply intent. |
| Canonical CSL update | Planned canonical changes are visible and are not silently overwritten by provider evidence. |
| Sidecar update | Verification evidence writes target `references.verification.json` and preserve provenance. |
| Derived queue update | Review queue changes are derived from accepted state and are auditable. |
| Schema failure | Invalid planned changes are rejected before any file mutation. |
| Audit log | Applied changes produce audit entries with affected files, request metadata, and result. |
| Partial failure | Failed apply leaves workspace state consistent and reports recovery guidance. |
