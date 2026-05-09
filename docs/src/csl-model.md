# Canonical CSL Model

`references.csl.json` is the canonical academic-reference file for Sourceright. Every academic reference that has entered the pipeline should be represented here as clean, style-neutral CSL JSON before downstream verification, review, and export behavior relies on it.

## File Contract

The canonical file name is `references.csl.json`. The initial implementation should read and write this file as a single JSON array of CSL item objects:

```json
[
  {
    "id": "smith-2024-trial",
    "type": "article-journal",
    "title": "Example trial title",
    "author": [
      {
        "family": "Smith",
        "given": "Jane"
      }
    ],
    "issued": {
      "date-parts": [[2024]]
    },
    "DOI": "10.0000/example"
  }
]
```

The file must be deterministic: item order, object key ordering, and formatting should be stable for the same logical data so that diffs remain reviewable.

## Clean Boundary

CSL JSON contains only bibliographic reference data. It must not contain Sourceright workflow state, provider diagnostics, confidence scores, review assignments, conflict records, extraction spans, source-file paths, or agent notes.

This boundary is strict because `references.csl.json` is the source for clean exports such as RIS, ENW, BibLaTeX, XML, and YAML. Internal verification state belongs in `references.verification.json`; unresolved work items belong in `review-queue.jsonl`.

Allowed CSL content includes:

- Stable `id` values used to join with sidecar records.
- CSL item `type`.
- Bibliographic fields such as title, author/editor, issued date, container title, volume, issue, pages, publisher, DOI, URL, ISBN, ISSN, PMID, and PMCID where they are valid CSL item data.
- Style-neutral normalized values produced by standardisation and cleaning.

Not allowed in canonical CSL records:

- `confidence`, `score`, `provider`, `verified`, `review_status`, or similar Sourceright-specific workflow fields.
- Provider candidate payloads.
- Field-level conflict explanations.
- Extraction provenance, source offsets, or document spans.
- Export-specific formatting choices.

If an input source includes mixed bibliographic and workflow metadata, the implementation should split it: write clean CSL fields to `references.csl.json` and write verification metadata to the sidecar.

## Identifiers

Every CSL item must have a non-empty `id`. The `id` is Sourceright's durable join key across:

- `references.csl.json`.
- `references.verification.json`.
- `review-queue.jsonl`.
- In-text citation reconciliation outputs.
- Export manifests and diagnostics.

Initial implementation should preserve incoming IDs when they are already unique and stable. If an ID must be generated, it should be deterministic from normalized bibliographic data, not from wall-clock time or array position. Duplicate IDs are validation errors.

## Initial Type Coverage

The initial implementation should support the common academic reference types first:

- `article-journal`.
- `book`.
- `chapter`.
- `paper-conference`.
- `report`.
- `thesis`.
- `webpage`.

Other CSL item types may pass through if they validate structurally, but unsupported type-specific transformations should produce diagnostics rather than silent lossy conversion.

## Validation Expectations

Validation should run before writing `references.csl.json` and before using it as export input. Diagnostics should be deterministic and machine-readable, with stable error codes suitable for CLI, MCP, and tests.

Minimum validation expectations:

- The top-level JSON value is an array.
- Each item is an object with non-empty `id` and `type`.
- IDs are unique within the file.
- Item `type` is a valid CSL type or an explicitly diagnosed unsupported type.
- Names use CSL name arrays where applicable.
- Dates use CSL date structures such as `date-parts`, not display-formatted date strings when structured data is available.
- DOI values are normalized enough for matching while preserving the identifier value.
- Sourceright verification fields are rejected from CSL records or moved to the sidecar by an explicit migration path.

Validation failures should not be hidden by export generation. Exports should only be produced from a clean canonical CSL file, unless the user explicitly requests a diagnostic or partial output mode.

## Relationship To Cleaning

Standardisation and cleaning may update CSL fields when the result is more accurate and style-neutral. Examples include normalizing DOI casing, moving journal names into `container-title`, converting display dates into CSL date parts, and separating author names into CSL name objects.

Cleaning decisions that need an audit trail should also record sidecar evidence. The CSL file should show the current best canonical reference; the sidecar should explain why Sourceright trusts it, where it came from, and what conflicts or manual decisions remain.
