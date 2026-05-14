# MCP Registry Release Binding Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Metadata validation | Covered by `tests/mcp_distribution_checks.rs`: `server.json` and Cargo metadata agree on name, version, package, schema, and repo. |
| OCI binding | Covered by `tests/mcp_distribution_checks.rs`: `Dockerfile` labels and release workflow tags/labels bind to the release version. |
| Registry acceptance | Release-status docs record Official MCP Registry accepted evidence for `0.1.20` and bind it to the versioned OCI package. |
| GHCR boundary | Release-status docs keep direct GHCR package visibility as `prepared` until separately verified. |
| Glama separation | Glama schema/maintainer metadata is validated separately and release-status docs keep Glama as prepared. |
| Review loop | Track checklist and review now record completed local evidence plus deferred external boundaries. |
