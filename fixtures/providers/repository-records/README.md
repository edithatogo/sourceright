# Repository Records Fixtures

This directory contains per-provider fixture files showing the actual JSON API response
shapes from three major research data repositories used by the Repository Record Provider
adapter (`provider.repository-records`).

## Files

| File | Provider | API Version | Description |
|------|----------|-------------|-------------|
| `zenodo.example.json` | [Zenodo](https://zenodo.org/) | Zenodo REST API v2 | A dataset record with metadata wrapper, creators with ORCIDs, file listings, and statistics. The top-level contains `id`, `doi`, `doi_url`, `title`, `metadata` (nested object with full metadata), `created`/`modified` timestamps, `links`, `files`, and `stats`. |
| `osf.example.json` | [OSF](https://osf.io/) | OSF API v2 | A project node with JSON:API-compliant `data` wrapper containing `id`, `type`, and `attributes` (title, doi, dates, description, category, license, tags). Includes `relationships` and `links` sections. |
| `figshare.example.json` | [Figshare](https://figshare.com/) | Figshare API v2 | An article (dataset) record with flat JSON structure: `id`, `doi`, `title`, `authors` array, `published_date`, `defined_type`, `description`, file listings, categories, tags, references, and timeline. |

## Usage

These fixtures are used by the repository record provider adapter tests to validate
response parsing. Each file represents a realistic but synthetic API response from the
respective repository service.

## Key Fields Extracted by the Adapter

The `repository_records_fixture_result` function in `src/live_providers.rs` extracts:

- **`doi`** – from the top-level `doi` field (all three fixtures provide this at root level)
- **`title`** – from the top-level `title` field, with a fallback to `sorttitle` (all three fixtures provide this at root level)

Additional provider-specific fields available in the raw payload (stored as evidence in the verification sidecar):

- **Zenodo**: `metadata.creators`, `metadata.publication_date`, `metadata.resource_type`, `metadata.access_right`, `doi_url`
- **OSF**: `data.attributes.title`, `data.attributes.doi`, `data.attributes.date_created`, `data.attributes.description`
- **Figshare**: `authors`, `published_date`, `defined_type`, `url`
