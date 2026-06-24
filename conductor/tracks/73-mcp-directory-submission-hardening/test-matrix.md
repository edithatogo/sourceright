| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Registry metadata | `server.json`, `glama.json`, and MCPB metadata match target release. | Existing MCP distribution tests plus release-surface verifier | Default-CI |
| Smithery package | `.mcpb` bundle builds from release binary and installs locally. | `scripts/build-smithery-mcpb.ps1` output and smoke log | Opt-in-live |
| Glama listing | Listing or API evidence records URL, version/date, and install metadata. | Track evidence row | Opt-in-live |
| No overclaim | Smithery/Glama remain prepared until accepted evidence exists. | `tests/marketplace_submission_evidence_policy.rs` | Default-CI |
