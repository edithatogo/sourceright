# Verification Sidecar

`references.verification.json` stores Sourceright verification state for records in `references.csl.json`. It exists so the canonical CSL file stays clean while the system remains auditable, deterministic, and safe to hand to humans or agents for review.

## File Contract

The canonical sidecar file name is `references.verification.json`. The initial implementation should read and write one JSON object with a schema version and records keyed by CSL `id`:

```json
{
  "schema_version": "sourceright.verification.v1",
  "references": {
    "smith-2024-trial": {
      "extraction": {
        "source": "input.docx",
        "original_text": "Smith J. Example trial title. Journal. 2024.",
        "span": "paragraph:12"
      },
      "provider_candidates": [
        {
          "provider": "crossref",
          "confidence": 0.86,
          "retrieved_at": "2026-05-09T00:00:00Z",
          "data": {
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
      "review_status": "queued",
      "review_decisions": []
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

The Rust model exposes the stable workflow states used by the sidecar:

- `not_required`: no review queue entry is required.
- `queued`: manual or agent review is waiting.
- `in_progress`: a reviewer has claimed or started the record.
- `resolved`: review is complete and the retained evidence supports the current canonical CSL record or a deliberate merge.
- `unresolved`: review completed without enough evidence to treat the record as clean.

Review helpers enforce explicit transitions. A resolved record must be reopened to `queued` before being moved back to active review, and conflicted records with terminal review states should retain at least one review decision. Queue builders should use the model helper rather than hand-maintaining a separate status list.

Provider candidates are considered usable only when they name a provider, include a retrieval timestamp, carry non-null provider data, and use a finite confidence value from `0.0` to `1.0`. Conflict entries should identify the field, severity, and either a provider or source so reports and future providers can explain why review is needed.

## Review Queue

`review-queue.jsonl` is the work queue for unresolved records. Each line should be one JSON object keyed to a CSL `id`. It should contain enough context for human or agent review without requiring mutation of the canonical CSL file:

```json
{"id":"smith-2024-trial","extraction":{"source":"input.docx","original_text":"Smith J. Example trial title. Journal. 2024.","span":"paragraph:12"},"provider_candidates":[{"provider":"crossref","confidence":0.86,"retrieved_at":"2026-05-09T00:00:00Z","data":{"DOI":"10.0000/example","title":"Example trial title","type":"article-journal"}}],"conflicts":[{"candidate":{"date-parts":[[2023]]},"canonical":{"date-parts":[[2024]]},"field":"issued","provider":"crossref","severity":"review"}],"review_status":"queued"}
```

The queue is derived from the sidecar. `references.verification.json` is the durable verification record; `review-queue.jsonl` is the operational list of records that still need attention.

The Rust model exposes `VerificationSidecar::review_queue_entries()` and `VerificationSidecar::to_review_queue_jsonl()` for this derived output. These helpers include only records whose review status is `queued`, `in_progress`, or `unresolved`, and because records are stored by CSL id in a sorted map, emitted JSONL lines are deterministic by reference id. Resolved and `not_required` records stay in `references.verification.json` but are omitted from the operational queue.

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
- Review records include decision evidence when terminal conflicted records are resolved or left unresolved.
- `review-queue.jsonl` entries are derived from records whose sidecar status is `queued`, `in_progress`, or `unresolved`.

Validation should also protect the clean boundary. If sidecar-only fields are found in `references.csl.json`, the canonical model validator should reject or migrate them. If bibliographic corrections are made during review, the canonical CSL item should be updated and the sidecar should record the decision.

## Compatibility

The first sidecar schema version is `sourceright.verification.v1`. Future incompatible changes should increment `schema_version` and provide migration diagnostics. Compatible additions may add optional fields, but existing readers should continue to treat unknown fields as non-authoritative unless the schema version requires them.

The Rust core exposes the completed sidecar contract through:

- `parse_verification_sidecar_json` for canonical loading.
- `format_verification_sidecar_json` for newline-terminated deterministic pretty JSON.
- `VerificationSidecar::validate` for stable sidecar diagnostics, including schema-version checks and provider/conflict/review invariant checks.
- `VerificationSidecar::review_queue_entries` and `VerificationSidecar::to_review_queue_jsonl` for derived queue output.
