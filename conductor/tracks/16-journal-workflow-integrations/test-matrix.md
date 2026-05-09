# Journal Workflow Integrations Test Matrix

| Scenario | Expected result |
| --- | --- |
| Generic screening contract | Manuscript text or CSL JSON produces JSON and Markdown citation-integrity reports with stable severity summary. |
| OJS submission file | OJS adapter maps a submitted manuscript or extracted text into Sourceright intake and attaches/stores report artifacts. |
| OJS author feedback setting | Author-facing checklist is generated only when configured and omits internal editorial diagnostics. |
| Generic webhook input | Webhook/batch runner accepts submission id and file/text location, then returns deterministic report artifact paths or payloads. |
| ScholarOne adapter contract | Contract describes required manuscript-file retrieval, report upload, and status signaling without requiring live API tests by default. |
| Editorial Manager adapter contract | Contract describes ingest/workflow integration and report return points without requiring live API tests by default. |
| eJournalPress adapter contract | Contract supports batch/webhook integration pending vendor API access. |
| Privacy boundary | Manuscript text is processed locally by default and report excerpts can be minimized/redacted. |
| Extraction blocked | DOCX/PDF/OCR adapter gaps produce a clear platform-facing extraction-blocked result. |
| Citation-risk report | Plausible but unverified references, missing DOI, conflicts, and missing sidecar evidence appear as risks without claims of AI authorship. |
| Live platform smoke | Live checks are opt-in, credential-gated, and skipped in normal CI. |
