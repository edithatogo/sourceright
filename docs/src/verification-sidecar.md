# Verification Sidecar

`references.verification.json` stores Sourceright verification state for records in `references.csl.json`. It exists so the canonical CSL file stays clean while the system remains auditable, deterministic, and safe to hand to humans or agents for review.

## File Contract

The canonical sidecar file name is `references.verification.json`. The initial implementation should read and write one JSON object with a schema version and records keyed by CSL `id`:

```json
{
  "schema_version": 1,
  "records": {
    "smith-2024-trial": {
      "status": "needs_review",
      "confidence": 0.72,
      "sources": [
        {
          "kind": "document",
          "label": "input.docx",
          "extracted_text": "Smith J. Example trial title. Journal. 2024."
        }
      ],
      "candidates": [
        {
          "provider": "crossref",
          "provider_id": "10.0000/example",
          "confidence": 0.86,
          "matched_fields": ["DOI", "title"],
          "candidate_csl": {
            "type": "article-journal",
            "title": "Example trial title",
            "DOI": "10.0000/example"
          }
        }
      ],
      "conflicts": [
        {
          "field": "issued",
          "canonical": {"date-parts": [[2024]]},
          "candidate": {"date-parts": [[2023]]},
          "provider": "crossref",
          "severity": "review"
        }
      ],
      "review": {
        "state": "queued",
        "reason": "Provider year conflicts with extracted reference."
      }
    }
  }
}
```

The sidecar should be deterministic: record keys, arrays, and emitted object keys should have stable ordering where that does not change meaning.

## Schema Intent

The sidecar is not a second canonical reference model. It is an audit and workflow record keyed to CSL items. Its job is to answer:

- What source text or file produced this reference?
- Which providers were checked?
- What candidate records did providers return?
- Which fields matched or conflicted?
- How confident is Sourceright in the canonical CSL record?
- Does this record need manual review?
- What decision did a reviewer or agent make?

The sidecar may contain candidate CSL fragments from providers, but those fragments are evidence only. The authoritative academic-reference data remains the matching item in `references.csl.json`.

## Status Values

Initial implementation should use a small status set:

- `unverified`: canonical CSL exists but no provider or manual verification has completed.
- `verified`: automated or manual verification supports the canonical CSL record.
- `needs_review`: deterministic checks found ambiguity, low confidence, missing evidence, or conflicts.
- `rejected`: the record should not be treated as a valid reference without further user action.
- `merged`: a manual or deterministic process resolved this record into another canonical CSL ID.

Status transitions should be explicit in sidecar data. When a record moves from `needs_review` to `verified`, the sidecar should retain the decision evidence rather than only the final status.

## Review Queue

`review-queue.jsonl` is the work queue for unresolved records. Each line should be one JSON object keyed to a CSL `id`. It should contain enough context for human or agent review without requiring mutation of the canonical CSL file:

```json
{"id":"smith-2024-trial","reason":"year_conflict","priority":"normal","assigned_to":null}
```

The queue is derived from the sidecar. `references.verification.json` is the durable verification record; `review-queue.jsonl` is the operational list of records that still need attention.

The queue should be partitionable for parallel review. A reviewer should be able to claim or complete one record without changing unrelated queue entries.

## Boundary With CSL

The sidecar owns all Sourceright-specific metadata:

- Provider names, IDs, request timestamps, response diagnostics, and cached candidate summaries.
- Confidence scores and matching explanations.
- Field-level conflicts and selected resolutions.
- Source provenance, extraction text, document spans, and imported fixture paths.
- Manual review state, reviewer decisions, and agent notes.
- Migration and compatibility metadata.

The sidecar must not be required to render a clean bibliography export. Exporters may consult the sidecar for gating or diagnostics, but output formats should be generated from canonical CSL data.

## Validation Expectations

Sidecar validation should run whenever a sidecar is loaded or written. Diagnostics should use stable machine-readable codes.

Minimum validation expectations:

- `schema_version` is present and supported.
- Every sidecar record key matches a CSL `id` in `references.csl.json`, unless the record is explicitly marked as an orphan diagnostic during migration.
- No CSL item is silently missing sidecar state after a full verification run.
- Status values are from the supported set.
- Confidence values are numeric and bounded from `0.0` to `1.0`.
- Provider candidate entries name the provider and preserve enough provider identity to re-check or explain the match.
- Conflict entries identify the field, competing values, source/provider, and severity.
- Review records include a state and reason when `status` is `needs_review`.
- `review-queue.jsonl` entries refer to records whose sidecar status is `needs_review`.

Validation should also protect the clean boundary. If sidecar-only fields are found in `references.csl.json`, the canonical model validator should reject or migrate them. If bibliographic corrections are made during review, the canonical CSL item should be updated and the sidecar should record the decision.

## Compatibility

The first sidecar schema version is `1`. Future incompatible changes should increment `schema_version` and provide migration diagnostics. Compatible additions may add optional fields, but existing readers should continue to treat unknown fields as non-authoritative unless the schema version requires them.
