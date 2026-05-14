# ADR 0002: Rust Core With Thin Host Adapters

## Status

Accepted.

## Decision

Sourceright keeps reference extraction, verification, reporting, export, and
policy logic in the Rust core. Host integrations should be thin adapters that
call the CLI, MCP runtime, or stable Rust-facing contracts.

## Rationale

This avoids reimplementing verification behavior in Zotero, OJS, Streamlit, or
other host-specific code and keeps evidence boundaries consistent.

## Consequences

Adapters must prove they preserve CSL, sidecar, review queue, audit log, and
claim-boundary semantics.
