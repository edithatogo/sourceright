# submission-core Migration Mapping Check

Date: 2026-06-09

## Pinned contract

`submission-core-contract-snapshot.json` records the expected legacy integration
surface for `arXiv/arxiv-submission-core` as implemented in Sourceright.

## Command

```bash
cargo test arxiv_submission_core_migration_mapping_matches_pinned_snapshot -- --nocapture
cargo test track_80_arxiv_submission_core_maturity_hardening_is_completed -- --nocapture
```

## Checks performed

| Check | Result |
| --- | --- |
| Primary fixture `upstream_target.repository` matches snapshot | Pass |
| Event variant suite covers accepted/held/rejected/malformed/unknown-event | Pass |
| Unknown events map to `screened_with_warnings` | Pass |
| Malformed events map to `screened_with_errors` | Pass |
| Fixture `$schema` resolves to arXiv submission fixture schema | Pass |
| Journal screening output schema version unchanged | Pass |
| CLI/MCP platform aliases match snapshot labels | Pass |
| Security boundaries documented | Pass |

## Boundary

Migration mapping checks guard the local Sourceright contract against fixture
and schema regressions. They do not poll the live `arXiv/arxiv-submission-core`
repository or claim upstream domain-model parity until maintainer pinning is
recorded.
