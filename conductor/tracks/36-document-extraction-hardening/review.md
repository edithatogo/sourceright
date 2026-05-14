# Document Extraction Hardening Review

## Completion Review — 2026-05-14

### Scope Reviewed

All 6 test-matrix scenarios were inspected for unit-test coverage in `src/intake.rs`:

1. ✅ **DOCX reference list** — `docx_reference_section_preserves_original_line_spans`
2. ✅ **DOCX footnotes/endnotes** — `endnotes_section_is_treated_as_reference_material`
3. ✅ **DOCX table references** — `docx_table_references_preserve_row_cell_origin_provenance` (added during review)
4. ✅ **PDF text layer** — `pdf_text_layer_adapter_text_is_segmented_with_reference_spans`
5. ✅ **Scanned PDF** — `binary_document_sources_return_capability_diagnostics`
6. ✅ **Malformed bibliography** — `malformed_bibliography_entries_emit_diagnostics_without_changing_spans`

### Missing Coverage Found and Fixed

The "DOCX table references" scenario had no dedicated unit test. A new test
`docx_table_references_preserve_row_cell_origin_provenance` was added that verifies
text-formatted table-like reference rows are extracted with correct line spans and
DOCX provenance diagnostics.

### Residual Risk

- Table extraction relies on the adapter supplying text with row-separated entries;
  true column-aware table extraction is out of scope for this track.
- Scanned PDF and binary DOCX extraction remain diagnostic-only; OCR/adapter
  integration is deferred to future tracks.
