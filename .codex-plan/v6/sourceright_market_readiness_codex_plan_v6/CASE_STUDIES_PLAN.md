# Case studies plan

Add three fake-but-realistic case-study fixtures. Do not use copyrighted manuscripts or real confidential submissions.

## Case study 1 — Biomedical manuscript

Scenario:
- one valid DOI;
- one missing DOI;
- one retraction/expression-of-concern warning;
- one preprint that may have a version of record.

Artifacts:
```text
case-studies/biomedical-preflight/input.md
case-studies/biomedical-preflight/references.csl.json
case-studies/biomedical-preflight/references.verification.json
case-studies/biomedical-preflight/review-queue.jsonl
case-studies/biomedical-preflight/reference-report.json
case-studies/biomedical-preflight/export-manifest.json
case-studies/biomedical-preflight/README.md
```

## Case study 2 — Preprint moderation

Scenario:
- mostly valid references;
- one plausible but unverified reference;
- one source-type mismatch;
- one old source in a fast-moving area.

## Case study 3 — Legal filing prototype

Scenario:
- valid case citation;
- ambiguous short-form citation;
- pinpoint issue;
- legal citation separate from CSL.

## Case study requirements

Each README should include:
- what is being demonstrated;
- how to run commands;
- expected result;
- limitations;
- no claim of legal or editorial compliance.
