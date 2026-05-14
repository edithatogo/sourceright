# Citation-Manager Adapter Proof Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Zotero preview | Proposed changes are visible without writes. |
| Zotero apply | Apply requires explicit opt-in and writes an audit log. |
| EndNote handoff | ENW/RIS files are generated and structurally checked. |
| Boundary | EndNote export does not count as Zotero sync proof. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
