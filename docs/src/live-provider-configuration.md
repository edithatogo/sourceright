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
3. Enable the global live flags only for the session that needs them.
4. Capture the provider output as evidence, not as a silent overwrite of CSL.

## Safety boundary

Do not treat live provider configuration as a promise that every planned
adapter is already available. The repo uses the registry to distinguish
`core_normalizer`, `core_exporter`, `planned_public_api`, `planned_byo_key`, and
`planned_adapter` surfaces.
