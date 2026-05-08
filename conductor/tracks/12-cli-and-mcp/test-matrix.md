# CLI And MCP Test Matrix

| Scenario | Expected result |
| --- | --- |
| CLI help | Lists supported commands and exits successfully. |
| JSON output | Same input produces stable output. |
| Pipeline run | Creates canonical, sidecar, queue, report, and exports. |
| MCP startup | Server initializes and exposes expected tools/resources. |
| Adapter smoke | Thin adapter can invoke the shared binary or library. |
