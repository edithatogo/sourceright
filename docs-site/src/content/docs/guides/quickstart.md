---
title: Quickstart
description: First local loop for validating a Sourceright workspace.
---

Start with a small workspace such as `examples/workspace/`, then validate CSL,
inspect the report and review queue, and preview exports before writing files.

```text
sourceright validate-csl references.csl.json
sourceright report examples/workspace
sourceright review queue examples/workspace
sourceright export --preview --all examples/workspace
```

Keep provider evidence in `references.verification.json`; do not merge it into
canonical CSL without explicit review.
