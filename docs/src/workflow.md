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

The current Rust core implements deterministic text-like intake for pasted text, plain text, Markdown, direct DOCX manuscript parsing, and adapter-supplied PDF text layers. The DOCX path preserves paragraph order and normalizes superscript numeric citations into bracketed inline markers for reconciliation. Scanned-PDF/OCR sources are represented as explicit capability diagnostics until OCR adapters are wired in, so binary sources do not fail silently or pretend extraction happened.

That means the current workflow is appropriate for structured reference triage,
not as a final examiner-grade verifier. Tracks 36-40 harden the gaps: real
DOCX/PDF extraction, opt-in live Crossref/DataCite/OpenAlex/PubMed evidence,
smarter institutional-author and same-author citation handling, URL/archive
integrity checks, and explicit low-noise writeback suggestions.

Automated verification should do everything deterministic first. Ambiguous records then move to a manual review queue with original extracted text, provider candidates, confidence, and diffs.

The implemented cleaning surface normalizes supported CSL fields, records transformations, groups duplicate candidates, and queues risky changes for review. Conflict resolution fills only safe missing canonical fields and preserves disagreements in the sidecar. Citation reconciliation reports missing, uncited, duplicate, ambiguous, numeric-order, mixed-style, and title-fallback issues. The export surface writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from canonical CSL JSON.

DOCX intake now parses the Word XML directly for manuscript text and citation
markers, while PDF support remains adapter-text based in this technical
preview. The intake layer keeps original line spans after reference-section
trimming, keeps wrapped bibliography entries together, and emits
`intake.references.malformed_entry` diagnostics for explicit reference-list
entries that do not satisfy the current deterministic reference heuristics.
Scanned PDFs still return `intake.ocr.required` until an OCR adapter is wired in.
