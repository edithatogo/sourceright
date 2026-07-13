# CiteWeft and optional GROBID extraction

Sourceright contains a provisional, internal `citeweft` boundary for scholarly
document extraction. It is not a published package or cleared product name.
The neutral model does not depend on Sourceright CSL, sidecar, CLI, MCP, or
provider types. A separate adapter turns neutral references into intake
candidates and review evidence.

The initial backend supports GROBID's `/api/processReferences` endpoint only.
It is disabled by default, uses `http://127.0.0.1:8070` by default, rejects
remote endpoints without explicit opt-in, and requires HTTPS for opted-in
remote hosts. Requests are bounded by document size, response size, timeout,
and retry count. They send `includeRawCitations=1` and
`consolidateCitations=0`; provider consolidation remains Sourceright's job.

The user-facing command is:

```text
sourceright extract-references [options] document.pdf
sourceright extract-references --health [options]
```

Use `--json` for candidates and review-sidecar evidence. `--health` probes
GROBID health/version without uploading a document. The command never writes
canonical CSL or sidecar files.

The implemented TEI subset preserves raw citations, TEI identifiers, authors,
title, container title, date, volume, issue, pages, and identifiers. Missing or
unsupported content remains absent and may produce diagnostics; fields are not
fabricated. HTTP 204, persistent 503 overload, unsafe endpoints, oversized
documents/responses, transport failures, and malformed TEI have typed errors.

All adapted records are queued for review. Extraction provenance belongs in
`references.verification.json`; the backend does not write or overwrite
`references.csl.json`. Default tests decode self-authored TEI and require no
network, container, credentials, model download, or private document.

GROBID-NER is not a runtime dependency. Entity recognition has an independent
neutral trait, and entity output must remain outside CSL. Any future data,
weights, taxonomy, or compatibility bridge requires file-by-file provenance,
license, version, and redistribution review.

Full-document extraction, coordinates, citation callouts, native PDF/layout,
native learned reference parsing, NER models, hybrid routing, production
hardening, and extraction to a separate repository are follow-on tracks.
