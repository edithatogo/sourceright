# CLI And MCP Test Matrix

| Scenario | Status | Expected result |
| --- | --- | --- |
| CLI help | Initial | Lists `init`, `validate-csl`, and `mcp` with clear descriptions. |
| Init command | Initial | Creates or confirms the local workspace layout without mutating unrelated files. |
| CSL validation success | Initial | Valid CSL JSON exits successfully and can emit deterministic diagnostics. |
| CSL validation failure | Initial | Invalid CSL JSON exits non-zero with stable machine-readable error details. |
| MCP placeholder | Initial | `sourceright mcp` exits non-zero or reports unavailable server mode clearly. |
| Extract command | Planned | Produces reference candidates from supported input formats. |
| Normalize command | Planned | Produces canonical CSL JSON from extracted or supplied records. |
| Verify command | Planned | Produces verification sidecars and provider match diagnostics. |
| Review command | Planned | Reads and updates review queues according to explicit user action. |
| Export command | Planned | Writes requested export formats and an export manifest. |
| Pipeline run | Planned | Creates canonical, sidecar, queue, report, and export artifacts deterministically. |
| MCP startup | Planned | Server initializes and exposes expected tools/resources after core contracts stabilize. |
| Adapter smoke | Planned | Thin adapter can invoke the shared binary or library. |
