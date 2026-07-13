# Track 80 — arXiv submission-core Maturity Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory id | Prior track |
| --- | --- | --- |
| arXiv submission-core | `arxiv-submission-core` | Track 78 requirements matrix |

## Hardening gates

| Gate | Evidence |
| --- | --- |
| Legacy fixture breadth | `arxiv-submission-core-variants.json` (accepted, held, rejected, malformed, unknown-event) |
| Migration-safe mapping | `submission-core-contract-snapshot.json`, `migration-mapping-check-2026-06-09.md` |
| Compatibility tests | `tests/arxiv_platform_adapter_policy.rs`, `tests/cli_end_to_end.rs` |
| Security / no-writeback | `security-boundaries.md`, journal screening unit tests |
| Maintainer packet | `evidence-packet.md` |

## Claim boundary

Upstream-ready local hardening only. No upstream GitHub issue, pull request, or
legacy module acceptance is claimed.
