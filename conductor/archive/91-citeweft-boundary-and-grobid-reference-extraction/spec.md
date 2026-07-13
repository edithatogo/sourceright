# Specification

Implement a provisional, standalone-ready neutral Rust extraction boundary and
an optional safe-by-default GROBID `/api/processReferences` backend. Map neutral
references into existing Sourceright intake candidates and queued sidecar
evidence without constructing or overwriting canonical CSL. Preserve raw text,
stable TEI identifiers, supported parsed fields, engine/config provenance, and
diagnostics. Default tests are fixture-only. GROBID-NER remains independent;
full-document, native model, routing, and repository extraction are out of
scope and owned by Tracks 92-97.

## User-facing contract

The backend must be selectable through Sourceright's normal configuration or
CLI path, while remaining disabled unless explicitly selected. Configuration
must expose the backend, endpoint policy, timeout, request/response limits, and
bounded retry count without requiring callers to construct Rust types directly.

Each extraction run should optionally probe GROBID health/version endpoints and
record the observed engine identity, endpoint class, configuration fingerprint,
and input hash in provenance. Health probing must not upload document content.

TEI source spans must use stable `xml:id` values where present. Internal parser
node numbers must not be exposed as durable provenance identifiers.

The deterministic fixture contract requires checked-in, self-authored or
clearly licensed TEI/XML fixtures with a manifest, source/license attribution,
golden neutral output, and golden adapter output. Inline-only fixtures are not
an acceptable long-term evidence format.

HTTP behavior must be testable without a live service through a deterministic
mock server or equivalent request/response harness covering multipart fields,
204, 503 retry exhaustion, timeouts, size limits, malformed responses, and
endpoint rejection.

The track is not complete until dependency/license, coverage, package, and
security checks have been rerun after the multipart dependency change.

## Additional hardening requirements

- The HTTP client must have an injectable transport seam so request and failure
  behavior can be tested without binding tests to a live port or service.
- Remote endpoint policy must resist SSRF through non-public DNS resolution,
  redirects, alternate IP forms, and unexpected schemes. Remote use should be
  allowlist-based, not merely a boolean.
- XML fixtures must include entity-expansion, external-entity, deeply nested,
  and oversized-response cases proving safe rejection or bounded handling.
- Extraction must expose bounded concurrency and cancellation/deadline policy;
  a large batch must not create unbounded uploads, memory use, or retries.
- The neutral serialized schema needs explicit major/minor compatibility rules,
  golden compatibility tests, and unknown-field behavior before extraction to a
  separate repository is considered.
- GROBID code/image/version, `pdfalto` aggregation boundary, notices, fixture
  licenses, and any future model/data artifacts need a machine-readable
  third-party evidence record before distribution claims are made.
