# MCP Registry Release Binding Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Metadata validation | `server.json` and release metadata agree on name, version, package, and repo. |
| OCI binding | GHCR image labels and tag match the release version. |
| Registry acceptance | Official MCP Registry listing shows the submitted version before docs say accepted. |
| Glama separation | Glama prepared/accepted status is tracked independently. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
