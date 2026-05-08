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

The first implementation starts with text-like sources and DOCX, then expands to PDF text extraction and OCR/scanned documents.

Automated verification should do everything deterministic first. Ambiguous records then move to a manual review queue with original extracted text, provider candidates, confidence, and diffs.
