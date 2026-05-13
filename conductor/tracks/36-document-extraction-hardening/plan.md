# Document Extraction Hardening Plan

1. Inventory current text-like intake behavior and fixture coverage.
2. Add representative DOCX and PDF fixtures with expected extraction spans.
3. Implement real DOCX extraction for paragraphs, lists, tables, footnotes, and
   endnotes where feasible.
4. Implement PDF text-layer extraction and explicit scanned-PDF/OCR diagnostics.
5. Preserve original text, source path, and document span in the verification
   sidecar.
6. Update CLI/help/docs to distinguish supported extraction from diagnostic
   placeholders.

## Completion Signal

Messy DOCX and PDF fixtures produce stable references, citations, provenance,
and review diagnostics without silent extraction failures.

## Progress Notes

- 2026-05-12: First deterministic hardening slice landed. DOCX/PDF adapter text
  now detects common reference-section headings such as `works cited`,
  `footnotes`, and `endnotes`, and reference spans preserve original line
  numbers after section trimming.
- 2026-05-12: Second deterministic segmentation slice landed. Extracted text
  now treats numbered, bracketed, and bulleted entries as reference starts and
  keeps wrapped continuation lines attached to the same candidate.
- 2026-05-12: Third diagnostic slice landed. Explicit malformed bibliography
  entries found under a detected references section now emit
  `intake.references.malformed_entry` diagnostics without changing accepted
  reference spans.
- 2026-05-13: Consolidation pass updated operator docs to describe adapter-text
  DOCX/PDF span handling, malformed-reference diagnostics, and the remaining OCR
  boundary.
