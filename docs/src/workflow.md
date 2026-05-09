# Reference Workflow

The target workflow is:

```text
document/text
  -> extracted references and in-text citations
  -> references.csl.json
  -> standardisation, cleaning, verification, enrichment
  -> references.verification.json and review-queue.jsonl
  -> XML, ENW, RIS, BibLaTeX, and YAML exports
```

The current Rust core implements deterministic text-like intake for pasted text, plain text, and Markdown. DOCX, PDF text-layer, and scanned-PDF/OCR sources are represented as explicit capability diagnostics until adapters are wired in, so binary sources do not fail silently or pretend extraction happened.

Automated verification should do everything deterministic first. Ambiguous records then move to a manual review queue with original extracted text, provider candidates, confidence, and diffs.

The implemented cleaning surface normalizes supported CSL fields, records transformations, groups duplicate candidates, and queues risky changes for review. The export surface writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from canonical CSL JSON.
