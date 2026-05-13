# Document Extraction Hardening Spec

## Goal

Move Sourceright from adapter-supplied text intake toward examiner-grade DOCX
and PDF reference extraction that can be trusted on messy manuscripts.

## Scope

- Extract reference sections from real DOCX manuscripts, including paragraphs,
  numbered lists, tables, footnotes, and endnotes where the adapter supports
  them.
- Extract references from PDF text layers without treating scanned images as
  successful extraction.
- Preserve source spans and original extracted text for every reference and
  citation candidate.
- Add scanned-PDF/OCR diagnostics and adapter hooks without making OCR a silent
  dependency.
- Build a fixture corpus covering ordinary papers, poor reference formatting,
  tables, footnotes, endnotes, and malformed inputs.

## Outputs

- DOCX and PDF extraction adapters or adapter contracts.
- Fixture-backed regression tests for common examiner workflows.
- Extraction provenance in `references.verification.json`.
- Updated CLI and workflow documentation that describes the supported document
  classes and explicit gaps.

## Boundaries

This track must not pretend OCR or binary extraction succeeded when only a
capability diagnostic was produced. It should preserve uncertainty for review
rather than manufacturing canonical CSL records from weak extraction evidence.
