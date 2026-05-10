# CLI Entrypoint Integration Test Matrix

| Scenario | Expected result |
| --- | --- |
| CLI help surface | `sourceright --help` and related usage output mention the new entrypoints consistently. |
| Command routing | `bench` and `citation-sync` are reachable from the primary CLI surface. |
| Documentation alignment | CLI docs and command references describe the same command names and usage patterns. |
| Example consistency | Examples match the current runtime invocation forms. |
| No behavior drift | CLI surface changes do not alter benchmark or citation-sync semantics. |
| No live defaults | The integrated commands remain local and deterministic unless explicitly opted into live behavior. |
