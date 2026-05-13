# Security, Publication, and Contract Governance Review

## Phase Review

Findings from the review pass:

1. Live-provider runtime controls were initially documented but only timeout was
   wired into request execution.

Resolution:

- `src/live_providers.rs` now applies `SOURCERIGHT_PROVIDER_MAX_RETRIES`,
  `SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS`, and
  `SOURCERIGHT_PROVIDER_CACHE_DIR` in live fetch helpers.
- `provider_cache_returns_evidence_payload_without_network` verifies cache-hit
  behavior without a network call.

## Residual Risk

- GitHub notification emails are account-level and cannot be fully suppressed
  from repo files.
- Copilot Autofix and Copilot coding-agent merge behavior depend on GitHub-side
  settings and entitlement. Repo-local work keeps alerts actionable and
  dependencies auto-maintained through Renovate.
- Live OJS, Zotero, provider, and registry smoke tests remain opt-in because
  they require credentials, sample instances, or external package access.
