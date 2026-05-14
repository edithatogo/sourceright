---
title: Release Notes
description: Overclaim-aware technical-preview release notes for Sourceright.
---

These notes are for technical-preview communication. They should describe what
is implemented, what is fixture-backed, and what remains planned without making
production-readiness or externally comparable benchmark claims.

## Technical preview status

- Canonical CSL validation, verification sidecars, review queues, reporting,
  export previews, and deterministic benchmark tasks are available for local
  evaluation.
- Provider evidence remains separated from canonical CSL and must not silently
  overwrite clean bibliographic data.
- The benchmark harness is useful for regression checks, not for public
  performance ranking.
- DOCX/PDF extraction, live provider verification, citation disambiguation,
  URL/archive checking, and low-noise writeback suggestions remain launch
  hardening work.

## Release wording

Use language such as "technical preview", "pilot-ready workflow", "structured
reference triage", and "fixture-backed regression benchmark". Do not claim
language such as "production-ready institutional platform", "examiner-grade
final verifier", or "state-of-the-art benchmarked performance" unless the
repository evidence supports those claims.

## Operator checklist

Before publishing release notes, run the Rust checks, the benchmark command, and
the docs-site build. Include any skipped live-provider checks explicitly.
