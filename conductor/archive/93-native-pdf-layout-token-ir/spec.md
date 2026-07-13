# Native PDF Layout and Token IR Spec

## Goal

Define a bounded, backend-neutral Rust layout contract that preserves pages,
glyph/text boxes, reading order, and scan/OCR diagnostics before any PDF parser
or native model is selected.

## User outcome

CiteWeft consumers can receive auditable layout tokens with page-relative boxes
and explicit uncertainty without coupling layout to CSL, provider evidence, or
reference semantics.

## Scope

- Stable serializable layout/page/token/block types behind a Rust trait.
- Byte/page/resource limits and explicit malformed/scan/no-text diagnostics.
- Deterministic block reading order with ambiguity reporting.
- Fixture-backed contract tests using a self-authored text-layout adapter.
- Candidate-selection evidence kept separate from the stable IR.

## Out of scope

- Reference semantics, CSL mapping, OCR/VLM inference, or mandatory rendering.
- Claiming a PDF parser or universal PDF compatibility before candidate evidence.
- Adding an unreviewed parser dependency based on README performance claims.

## Data contracts

`LayoutDocument` contains pages, blocks, tokens, source boxes, reading order,
backend provenance, and diagnostics. Coordinates are page-relative. Tokens
retain source IDs and style hints. Scanned/no-text inputs are explicit and do
not produce synthetic text. Backend-specific data stays behind the adapter.

## Claim boundary

This slice proves the stable layout IR and fixture adapter only. It does not
claim native PDF extraction, OCR, or backend selection. Those claims require
the Track 92 benchmark plus license, security, malformed-input, and
cross-platform evidence.

## Evidence level target

Deterministic default-CI contract and fixture evidence, with backend selection
remaining an evidence-gated follow-on inside this track’s recorded plan.

## Parallelization plan

- Lane A: candidate license/API/security evidence.
- Lane B: stable IR and limits.
- Lane C: reading order and scan diagnostics.
- Lane D: selected backend integration only after A-C and Track 92 evidence.
