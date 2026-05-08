# Academic Providers Spec

## Goal

Verify and enrich academic references using reliable public citation APIs.

## Scope

- Crossref first.
- DOI resolution, DataCite, OpenAlex, PubMed/NCBI, and ORCID where useful.
- Provider clients behind traits for fixture-based tests.

## Boundaries

Provider results create candidates and provenance; they do not silently overwrite canonical references.
