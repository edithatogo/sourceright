# Plugin and provider status plan

## Goal

Make the registry market-safe by clearly labelling what is implemented, fixture-backed, planned, or BYO-key.

## Status labels

Use a controlled vocabulary:

```text
core_normalizer
core_exporter
fixture_backed
live_smoke_optional
planned_public_api
planned_byo_key
planned_adapter
planned
deprecated
unknown
```

## Provider matrix fields

For each provider/plugin:

```text
id
name
category
status
auth_required
license_profile
live_network_default
writes_canonical_csl
sidecar_only
fixture_path
manifest_path
docs_path
known_limitations
```

## Categories

```text
provider
citation_manager
journal
repository
legal
matcher
recency
relevance
extraction
export
demo
mcp
```

## Market-safe rule

Any planned provider must be explicitly described as planned, not implemented.

Paid/licensed providers such as Dimensions, Scopus, and Web of Science must be labelled:
- optional;
- BYO-key;
- license-constrained;
- sidecar-only unless a user explicitly exports allowed metadata.
