# Rust dependency rationalisation — sourceright (2026-11-13)

**Status:** proposal and review contract; no code adoption, runtime verification or benchmark result is claimed here.

## Repository-specific decisions

- Reconcile existing citation verification, extraction/provenance and marketplace/release tracks; do not merge scientific evidence with source retrieval acceptance.
- **Verified:** uses Serde, Thiserror, Citeweft, and blocking Reqwest with native TLS. Blocking API may be intentional for synchronous CLI.
- **Evaluate:** switching to a Tokio/async Reqwest client only if concurrency and cancellation requirements justify the additional runtime; measure throughput and provider pacing.
- **Extend:** Proptest for CSL normalization, matching, dedup, stable provenance and adversarial metadata; Criterion for parsing and matching, sanitised Insta snapshots for contracts.
- **Retain:** citeweft extraction, source citation identity and typed verification errors; Regex only for bounded pattern matching, not document parser replacement.
- **Security:** credential/URL redaction, provider licence/terms, SSRF and rate limits remain mandatory; do not turn raw upstream content into trace artifacts.

## Evidence and release gate

1. Discover actual source, package boundary, MSRV, lockfile, existing tests and Conductor ownership; avoid speculative dependencies.
2. For runtime behaviour, write failing contract tests before implementation, run targeted and full suites; preserve stable schemas, errors, CLI semantics and deterministic outputs.
3. Record `cargo tree -d`, feature selections, security/licensing/advisory reviews, transitive dependency cost and rollback before accepting an adoption.
4. For performance changes, benchmark identical inputs and correctness/invariance oracles; do not claim improvements without measurements.
5. This document is a planning entry, NOT proof of compilation, hosted CI or scientific validation; update status only after a revision-bound receipt.
