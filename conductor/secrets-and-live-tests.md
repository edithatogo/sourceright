# Secrets And Live-Test Policy

Default CI must not require live credentials or external service writes.

## Rules

- Live tests are opt-in via explicit environment variables.
- Missing credentials must produce a clear skip, not a failure, unless the job
  is explicitly a live-validation job.
- Secrets must never appear in logs, cache keys, snapshots, reports, or audit
  artifacts.
- Caches must avoid storing private payloads unless the provider terms and
  project policy allow it.
- Live writes to Zotero, OJS, Streamlit, registries, or package managers need
  explicit user approval.

## Environment Naming

Use `SOURCERIGHT_<SURFACE>_<SETTING>` names for repo-level opt-ins, for example:

- `SOURCERIGHT_PROVIDER_LIVE=1`
- `SOURCERIGHT_ZOTERO_LIVE=1`
- `SOURCERIGHT_OJS_LIVE=1`
- `SOURCERIGHT_REGISTRY_LIVE=1`
- `SOURCERIGHT_STREAMLIT_SMOKE=1`

Provider-specific keys should use the provider name:

- `SOURCERIGHT_DIMENSIONS_API_KEY`
- `SOURCERIGHT_SCOPUS_API_KEY`
- `SOURCERIGHT_WEB_OF_SCIENCE_API_KEY`

## Redaction

Any diagnostic writer must redact values from variables ending in `_KEY`,
`_TOKEN`, `_SECRET`, `_PASSWORD`, or `_COOKIE`.
