# External Proof Suites Test Matrix

| Proof family | Acceptance |
| --- | --- |
| Installed CLI | Binary install/run smoke produces expected JSON or help output. |
| MCP stdio | Transcript fixture proves initialize/list/read paths. |
| OJS | Fixture adapter produces editor and author screening outputs; live smoke is opt-in. |
| Zotero/EndNote | Preview/apply/audit semantics are fixture-backed; live library smoke is opt-in. |
| Live providers | Provider smoke respects timeout, retry, min-interval, and cache controls. |
| Registries | Accepted listings have install/listing checks; prepared surfaces do not overclaim. |
| Review loop | `$conductor-review` runs after each proof family slice. |
