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
