# Test matrix

| Check | Expected |
| --- | --- |
| Track metadata exists | `metadata.json` present |
| Track docs exist | `spec.md`, `plan.md`, `test-matrix.md` present |
| Dependency gate | Depends on track 72 and journal workflow integrations |
| Shared dependency contract | Track 72 submission requirements and the journal workflow integration boundary are the shared contract |
| Default CI | Policy tests validate the fixture-backed adapter contract without live credentials |
| Downstream compatibility | Short alias `71-arxiv-submission-platform-adapters` is referenced by tracks 78-81 |
