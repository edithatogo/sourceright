# submit-ce Schema/API Drift Check

Date: 2026-06-09

## Pinned contract

`submit-ce-contract-snapshot.json` records the expected integration surface for
the current arXiv `submit-ce` platform as implemented in Sourceright.

## Command

```bash
cargo test arxiv_submit_ce_contract_drift_matches_pinned_snapshot -- --nocapture
cargo test track_79_arxiv_submit_ce_maturity_hardening_is_completed -- --nocapture
```

## Checks performed

| Check | Result |
| --- | --- |
| Primary fixture `upstream_target.repository` matches snapshot | Pass |
| Variant suite covers complete/warning/rejected/malformed cases | Pass |
| Fixture `$schema` resolves to arXiv submission fixture schema | Pass |
| `submission.platform` enum is `arxiv_submit_ce` | Pass |
| Journal screening output schema version unchanged | Pass |
| CLI/MCP platform aliases match snapshot labels | Pass |
| Security boundaries documented | Pass |

## Boundary

Drift detection guards the local Sourceright contract against fixture and
schema regressions. It does not poll the live `arXiv/submit-ce` repository or
claim upstream API parity until maintainer pinning is recorded.
