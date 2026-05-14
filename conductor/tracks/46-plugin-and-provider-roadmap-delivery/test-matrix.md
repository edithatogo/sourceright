# Plugin And Provider Roadmap Delivery Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Registry inventory | Every `plugins/registry.toml` entry appears in the plugin roadmap table. |
| Status promotion | A plugin cannot move from planned to implemented without fixtures and docs. |
| Public API provider | Fixture-backed success, no-match, ambiguous, and rate-limit/error cases exist. |
| BYO-key provider | Credentials are opt-in; default tests skip without secrets. |
| Adapter | Preview/apply/audit or read-only behavior is tested against the Rust core. |
| Docs parity | Source docs and docs-site mirror plugin statuses. |
| Review loop | `$conductor-review` runs for each plugin family slice. |
