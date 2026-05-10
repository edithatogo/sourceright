# Reference Workflow

The target workflow is:

```text
document/text
  -> extracted references and in-text citations
  -> references.csl.json
  -> standardisation, cleaning, verification, enrichment
  -> references.verification.json and review-queue.jsonl
  -> conflict resolution, citation reconciliation, and reference integrity reports
  -> XML, ENW, RIS, BibLaTeX, and YAML exports
```

The current Rust core implements deterministic text-like intake for pasted text, plain text, Markdown, adapter-supplied DOCX text, and adapter-supplied PDF text layers. Scanned-PDF/OCR sources are represented as explicit capability diagnostics until OCR adapters are wired in, so binary sources do not fail silently or pretend extraction happened.

Automated verification should do everything deterministic first. Ambiguous records then move to a manual review queue with original extracted text, provider candidates, confidence, and diffs.

The implemented cleaning surface normalizes supported CSL fields, records transformations, groups duplicate candidates, and queues risky changes for review. Conflict resolution fills only safe missing canonical fields and preserves disagreements in the sidecar. Citation reconciliation reports missing, uncited, duplicate, ambiguous, and numeric-order issues. The export surface writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from canonical CSL JSON.
