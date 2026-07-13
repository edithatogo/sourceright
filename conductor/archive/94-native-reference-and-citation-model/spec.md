# Native Reference and Citation Model Spec

## Goal

Provide a deterministic native baseline for reference-section segmentation,
source-grounded bibliographic fields, numeric callout detection, and linking
behind the neutral CiteWeft boundary.

## User outcome

Consumers can inspect reference candidates and callout links with source spans,
confidence, and review states without mutating canonical CSL or silently
asserting bibliographic truth.

## Scope

- Versioned task labels and model-manifest metadata.
- Deterministic reference-section detection and entry segmentation.
- Conservative extraction of authors, title, year, and DOI evidence.
- Numeric callout detection and reference linking with explicit abstention.
- Byte spans and raw text preserved for review and future model comparison.

## Out of scope

- Constructing or writing canonical CSL.
- General NER, generative extraction, ONNX inference, or model training.
- Universal citation-style support or claims of superiority over GROBID.
- Treating extracted metadata as verified bibliographic truth.

## Data contracts

Every candidate and callout has a source span. Low-confidence candidates use
`review` status; `extracted` means only that the deterministic extractor
emitted a candidate, not that it was verified. Identifiers are evidence strings only. The model manifest
records the deterministic backend, label schema, tokenizer/configuration,
license, and runtime requirements.

## Claim boundary

This track proves a fixture-backed deterministic baseline. Native accuracy,
calibration thresholds, learned artifacts, and GROBID comparisons require
independent Track 92 cohorts and later model evidence.

## Evidence level target

Fixture-backed native baseline with stable JSON, source grounding, deterministic
decoding, and explicit abstention; no live network or model download.

## Parallelization plan

- Lane A: labels, fixtures, and manifest.
- Lane B: segmentation and field baseline.
- Lane C: callout extraction and linking.
- Lane D: learned runtime/model artifacts only after baseline and licensing gates.
