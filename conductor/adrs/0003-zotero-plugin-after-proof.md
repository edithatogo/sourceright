# ADR 0003: Zotero Plugin After Preview/Apply Proof

## Status

Accepted.

## Decision

Sourceright will not claim mature Zotero plugin support until preview/apply/audit
semantics and disposable-library proof exist. A `.xpi` package is optional until
native Zotero UX is needed.

## Rationale

Zotero library writes are user-data writes. The adapter must prove dry-run and
audit behavior before public plugin claims.
