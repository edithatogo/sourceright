| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Fixture breadth | Complete, warning, rejected, and malformed `submit-ce` cases are covered. | arXiv fixture tests | Default-CI |
| Schema drift | Current `submit-ce` API/schema expectations are checked. | Drift check output | Default-CI |
| No writeback | Adapter emits screening only and never mutates arXiv or canonical CSL. | Policy tests | Default-CI |
| Local platform smoke | Optional local `submit-ce` smoke validates integration shape. | Smoke log | Opt-in-live |
