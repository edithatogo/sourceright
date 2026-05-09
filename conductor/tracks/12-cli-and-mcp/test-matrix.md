# CLI And MCP Test Matrix

| Scenario | Status | Expected result |
| --- | --- | --- |
| CLI help | Initial | Lists `init`, `validate-csl`, `report`, and `mcp` with clear descriptions. |
| Command help | Initial | Implemented commands expose command-specific usage and reject extra help arguments. |
| Init command | Initial | Creates or confirms the local workspace layout without mutating unrelated files. |
| CSL validation success | Initial | Valid CSL JSON exits successfully and can emit deterministic diagnostics. |
| CSL validation failure | Initial | Invalid CSL JSON exits non-zero with stable machine-readable error details. |
| MCP placeholder | Initial | `sourceright mcp` prints unavailable server status and exits non-zero. |
| MCP status | Initial | `sourceright mcp status` reports unavailable server status without pretending tools exist. |
| Extract command | Planned | Produces reference candidates from supported input formats. |
| Normalize command | Planned | Produces canonical CSL JSON from extracted or supplied records. |
| Verify command | Planned | Produces verification sidecars and provider match diagnostics. |
| Review command | Planned | Reads and updates review queues according to explicit user action. |
| Export command | Planned | Writes requested export formats and an export manifest. |
| Pipeline run | Planned | Creates canonical, sidecar, queue, report, and export artifacts deterministically. |
| MCP startup | Planned | Server initializes and exposes expected tools/resources after core contracts stabilize. |
| Adapter smoke | Planned | Thin adapter can invoke the shared binary or library. |
