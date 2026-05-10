# Reference Intake Plan

1. Define input abstractions for text and document sources. Implemented with `IntakeDocument` and `IntakeSourceKind`.
2. Implement reference-list segmentation for text-like inputs. Implemented for pasted text, plain text, and Markdown reference sections.
3. Add DOCX extraction. Implemented for adapter-supplied DOCX text with DOCX provenance; empty DOCX sources emit an explicit capability diagnostic.
4. Add PDF text extraction. Implemented for adapter-supplied PDF text layers with PDF provenance; empty PDF sources emit an explicit capability diagnostic.
5. Add OCR/scanned-document strategy after baseline fixtures pass. Implemented as an explicit OCR-required diagnostic pending adapter integration.
