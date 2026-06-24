| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Legacy fixture breadth | Accepted, held, rejected, malformed, and unknown-event cases are covered. | arXiv legacy fixture tests | Default-CI |
| Migration-safe mapping | Legacy states map into shared journal screening without data loss claims. | Mapping tests | Default-CI |
| No writeback | Adapter emits screening only and never mutates arXiv or canonical CSL. | Policy tests | Default-CI |
| Local legacy smoke | Optional local `arxiv-submission-core` smoke validates integration shape. | Smoke log | Opt-in-live |
