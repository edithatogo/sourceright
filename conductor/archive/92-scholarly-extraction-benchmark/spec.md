# Scholarly Extraction Benchmark Spec

## Goal

Create a licensed, leakage-controlled, stage-wise benchmark for future GROBID
and native CiteWeft extraction backends without turning third-party training
material or self-reported library benchmarks into acceptance evidence.

## User outcome

Maintainers can compare extraction changes using reproducible stage metrics,
backend provenance, cohort metadata, and explicit missing-data semantics.

## Scope

- Machine-readable manifest with source, license, access, immutable input hash,
  split, language, layout, scan status, and domain cohort.
- Offline, self-authored default-CI fixture with gold and prediction snapshots.
- Reference segmentation, field extraction, and callout-linking metrics.
- Backend version, model, configuration fingerprint, and hardware metadata.
- Hash verification and split-boundary leakage checks.
- Deterministic JSON CLI output and an explicit opt-in path for restricted data.

## Out of scope

- Declaring any backend universally best.
- Redistributing restricted corpora or downloading models in default CI.
- A combined citation-and-NER headline score.
- Replacing the existing generic benchmark runner or canonical CSL model.
- Product accuracy claims beyond the represented fixture cohort.

## Data contracts

Each document has an immutable content hash and license/access declaration.
Gold and prediction snapshots expose stage counts and backend provenance.
Unavailable coordinates are represented as `unavailable`, not zero. Future
restricted corpora must remain outside Git and provide retrieval instructions,
license evidence, and document/source-aware splits.

## Claim boundary

This track proves a deterministic benchmark contract and fixture evidence only.
Any public accuracy claim must name the corpus, split, cohort, backend
configuration, metric, and measurement date.

## Evidence level target

The default suite is a small, hash-verified, self-authored regression harness.
An independent opt-in corpus remains a follow-on evidence requirement before
model or backend selection claims.

## Parallelization plan

- Lane A: manifest, licensing, and split/leakage policy.
- Lane B: stage metrics and missing-data semantics.
- Lane C: snapshot runner and deterministic CLI report.
- Lane D: independent corpus acquisition remains separate and opt-in.
