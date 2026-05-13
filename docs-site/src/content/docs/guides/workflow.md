---
title: Workflow
description: The reference intake, verification, review, export, and publish flow.
---

The default Sourceright workflow is deliberately staged:

1. Intake references from documents, pasted text, or workspace fixtures.
2. Normalize CSL and record provider evidence in the verification sidecar.
3. Resolve conflicts without silently overwriting canonical CSL.
4. Queue uncertain records for manual review.
5. Export clean outputs only after the workspace passes validation.

The current workflow is appropriate for structured reference triage, not as a
final examiner-grade verifier. Scanned-PDF and OCR sources still surface as
explicit capability diagnostics until the extraction adapters land.

Adapter-supplied DOCX/PDF text keeps line spans, wrapped bibliography entries,
and malformed-reference diagnostics. Citation reconciliation reports
mixed-style and title-fallback diagnostics as review signals, not final
correctness claims.

See the [limitations page](limitations) for the current gap list and the
hardening tracks that close it.
