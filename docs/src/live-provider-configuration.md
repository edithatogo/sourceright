# Live Provider Configuration Guide

Live provider access is opt-in. The default test path is fixture-backed, and the
provider registry still marks several live adapters as planned or technical
preview. Use live configuration only when you need to confirm real provider
responses.

## Global opt-in

Set both flags before running the live smoke path:

```text
SOURCERIGHT_LIVE_PROVIDERS=1
SOURCERIGHT_LIVE_PROVIDER_SMOKE=1
```

If either flag is missing, the repo should stay on the default skipped or
fixture-backed path.

## Scholarly provider settings

| Provider | Environment variable | Notes |
| --- | --- | --- |
| Unpaywall | `UNPAYWALL_EMAIL` | Polite-mailto contact for the public API path. |
| OpenCitations | `OPENCITATIONS_ACCESS_TOKEN` | Optional auth for access-controlled use. |
| Europe PMC | `EUROPE_PMC_EMAIL` | Contact email for API use. |
| Repository records | `SOURCERIGHT_REPOSITORY_PMID` | Repository-evidence seed used by the live smoke helpers. |
| Bring-your-own key | `SOURCERIGHT_BYO_KEY` | Generic credential hook for licensed-data or private provider paths. |

The live-provider registry also covers arXiv and repository-record providers,
but their manifests should still be treated as contract surfaces first.

## Runtime controls

| Setting | Environment variable | Default | Notes |
| --- | --- | --- | --- |
| Request timeout | `SOURCERIGHT_PROVIDER_TIMEOUT_SECS` | `20` | Per-request timeout for opt-in live smoke helpers. |
| Minimum interval | `SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS` | `1000` | Conservative provider politeness interval for adapter implementations and launch runbooks. |
| Retry ceiling | `SOURCERIGHT_PROVIDER_MAX_RETRIES` | `2` | Maximum retry count for transient provider failures; fixture-backed tests should not retry. |
| Cache directory | `SOURCERIGHT_PROVIDER_CACHE_DIR` | unset | Optional local response cache path. Cached responses remain provider evidence, not canonical CSL. |

The live smoke report serializes the active runtime controls as
`runtime_controls` with `timeout_secs`, `min_interval_ms`, `max_retries`, and
`cache_enabled`. Capture that block with any opt-in live provider transcript so
the proof records the exact timeout, retry, politeness interval, and cache
policy used for the run.

Adapters should treat cache hits as provenance-bearing provider evidence and
record enough request metadata to explain when and how the response was
obtained. Cache policy must not hide rate-limit, outage, or malformed-response
diagnostics.

## Legal and citation-manager settings

| Surface | Environment variable | Notes |
| --- | --- | --- |
| CourtListener legal provider | `COURTLISTENER_API_KEY` | Optional key for the legal-provider contract. |
| Zotero sync | `SOURCERIGHT_ZOTERO_API_URL` | Used by the citation-sync preview/apply contract. |
| Zotero sync | `SOURCERIGHT_ZOTERO_API_KEY` | Used by the citation-sync preview/apply contract. |
| Zotero sync | `SOURCERIGHT_ZOTERO_LIBRARY_ID` | Required for a live library target. |
| Zotero sync | `SOURCERIGHT_ZOTERO_LIBRARY_TYPE` | Optional library type hint. |

## Practical order

1. Run the fixture-backed tests first.
2. Set the minimum provider-specific environment variables.
3. Set runtime controls for the provider terms you are testing.
4. Enable the global live flags only for the session that needs them.
5. Capture the provider output as evidence, not as a silent overwrite of CSL.

## Safety boundary

Do not treat live provider configuration as a promise that every planned
adapter is already available. The repo uses the registry to distinguish
`core_normalizer`, `core_exporter`, `planned_public_api`, `planned_byo_key`, and
`planned_adapter` surfaces.
