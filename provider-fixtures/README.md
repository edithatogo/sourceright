# Provider Fixture Expansion

This directory records offline fixture expectations for future provider
adapters. It is intentionally not wired to live network calls.

Each provider should eventually include:

- `success.json`
- `no-match.json`
- `ambiguous-match.json`
- `rate-limit.json`
- `outage.json`
- `malformed-response.json`
- `conflicting-metadata.json`

Fixture payloads should be synthetic or licence-compatible. Paid providers such
as Dimensions, Scopus, and Web of Science should use shape-only synthetic
fixtures unless the licence explicitly permits redistribution.
