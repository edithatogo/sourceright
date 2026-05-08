# Public Repo Infrastructure Test Matrix

| Scenario | Expected result |
| --- | --- |
| Local Rust check | `cargo check` succeeds. |
| GitHub Actions CI | Format, clippy, tests, and docs jobs pass on pull requests and `main`. |
| Pages build | Documentation deploys from the Pages workflow. |
| Tagged release dry-run | Release workflow packages artifacts without publishing unless tag conditions are met. |
| Security workflows | CodeQL, dependency review, audit, and Scorecard jobs run with minimum permissions. |
