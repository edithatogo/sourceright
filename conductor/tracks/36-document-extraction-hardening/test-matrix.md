# Document Extraction Hardening Test Matrix

| Scenario | Expected coverage |
| --- | --- |
| DOCX reference list | Extracts references with source spans and original text. |
| DOCX footnotes/endnotes | Detects citation/reference material without losing provenance. |
| DOCX table references | Preserves row/cell-origin evidence for review. |
| PDF text layer | Extracts references from readable text with page spans. |
| Scanned PDF | Emits OCR-required diagnostics without fake references. |
| Malformed bibliography | Queues uncertain entries for manual review. |
