# CLI And MCP Test Matrix

| Scenario | Status | Expected result |
| --- | --- | --- |
| CLI help | Initial | Lists `init`, `validate-csl`, `report`, and `mcp` with clear descriptions. |
| Command help | Initial | Implemented commands expose command-specific usage and reject extra help arguments. |
| Init command | Initial | Creates or confirms the local workspace layout without mutating unrelated files. |
| CSL validation success | Implemented | Valid CSL JSON exits `0`; text mode prints `valid`; JSON mode prints `{"ok":true,"path":...,"diagnostics":[]}`. |
| CSL validation failure | Implemented | Invalid-but-readable CSL JSON exits `1`; text mode prints stable diagnostic lines; JSON mode prints stable diagnostic objects. |
| CSL validation usage/runtime errors | Implemented | Missing paths, extra arguments, unreadable files, and JSON parse errors exit `2` with stderr errors. |
| MCP placeholder | Initial | `sourceright mcp` prints unavailable server status and exits non-zero. |
| MCP status | Initial | `sourceright mcp status` reports unavailable server status without pretending tools exist. |
| Report JSON output | Implemented | `sourceright report --json` emits compact `sourceright.reference_report.v1` JSON. |
| Report MCP resource output | Implemented | `sourceright report --mcp-resource` emits the `sourceright://reports/reference-integrity` resource envelope. |
| Extract command | Planned | Produces reference candidates from supported input formats. |
| Normalize command | Planned | Produces canonical CSL JSON from extracted or supplied records. |
| Verify command | Planned | Produces verification sidecars and provider match diagnostics. |
| Review command | Planned | Reads and updates review queues according to explicit user action. |
| Export command | Planned | Writes requested export formats and an export manifest. |
| Pipeline run | Planned | Creates canonical, sidecar, queue, report, and export artifacts deterministically. |
| MCP startup | Planned | Server initializes and exposes expected tools/resources after core contracts stabilize. |
| Adapter smoke | Planned | Thin adapter can invoke the shared binary or library. |
