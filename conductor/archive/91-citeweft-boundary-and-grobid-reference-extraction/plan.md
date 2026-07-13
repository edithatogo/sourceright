# Implementation Plan

- [x] Audit current intake, sidecar, plugin, Cargo, and roadmap contracts.
- [x] Add neutral model, extractor trait, and independent entity-recognizer trait.
- [x] Add disabled-by-default GROBID policy, multipart request, bounded response, status mapping, and TEI decoder.
- [x] Add Sourceright candidate/review adapter and deterministic vertical fixture.
- [x] Add plugin manifest, documentation, and feature-contract entry.

## Completion slice — user-facing and reproducible integration

- [x] Add normal CLI/configuration selection for `grobid` without enabling it by default.
    - [x] Expose endpoint, remote opt-in, explicit host allowlist, timeout, document/response limits, and retry settings.
    - [x] Reject unsupported combinations, including entity/coordinate options in the reference-only path.
    - [x] Emit deterministic machine-readable configuration diagnostics.
- [x] Add health/version probing without document upload and capture engine identity in provenance.
    - [x] Record endpoint class, configuration fingerprint, and caller-supplied input hash.
    - [x] Keep health/version failures actionable without leaking document content.
- [x] Replace internal XML node-number provenance with stable TEI `xml:id` provenance.
- [x] Move the inline TEI cases into a manifest-backed fixture corpus.
    - [x] Include source/license attribution, golden neutral output, and golden adapter output.
    - [x] Verify the fixture corpus is excluded from package claims unless its licenses permit redistribution.
- [x] Add deterministic local mock HTTP coverage for request fields, response limits, malformed response, and endpoint policy.
- [x] Complete Rust formatting, clippy, full tests, locked check, and benchmark validation on the stable GNU toolchain.
- [x] Run package and publish dry-run checks after the multipart and hashing dependency changes.
- [x] Run repository security/workflow and documentation policy checks after the dependency changes.
- [x] Record `cargo deny` and coverage as environment-gated CI checks; current workstation lacks both binaries and no local pass is claimed.
- [x] Record optional pinned local GROBID smoke as deferred; no Docker runtime is available in this environment.

## Hardening slice — transport, isolation, and lifecycle

- [x] Add deterministic local mock transport coverage for request/response behavior.
- [x] Replace boolean remote permission with an explicit HTTPS hostname allowlist and redirect disabling.
    - [x] DNS rebinding and resolved-address revalidation remain explicitly owned by Track 96 before remote production use.
    - [x] Reject unapproved schemes and remote hosts; credentials and alternate IP forms remain deferred with Track 96's remote deployment policy.
- [x] Add XML external-entity regression coverage; deep nesting and broader XML resource budgets remain Track 96 hardening.
- [x] Defer bounded batch concurrency and cancellation controls to Track 96; Track 91 exposes one bounded request.
- [x] Version the neutral schema and preserve unknown fields through serde-compatible models; cross-version compatibility fixtures remain Track 97 extraction criteria.
- [x] Add machine-readable third-party evidence for GROBID/pdfalto/runtime licenses, notices, image identity, fixtures, and future model/data artifacts.
- [x] Conductor review and close after the recorded environment-gated checks are accepted as CI/deferred evidence.

## Review fixes

- [x] Guard health/version probing behind the same disabled-by-default backend gate as extraction.
