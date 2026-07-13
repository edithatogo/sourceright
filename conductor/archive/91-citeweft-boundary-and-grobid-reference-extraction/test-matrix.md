# Test Matrix

| Area | Evidence |
| --- | --- |
| Safe defaults | Disabled by default; local/private endpoint accepted; remote HTTP rejected; remote HTTPS requires opt-in. |
| Request contract | `processReferences`, raw citations on, consolidation off, no coordinate options. |
| User-facing selection | CLI/configuration selects GROBID explicitly; default invocation remains disabled and machine-readable diagnostics are stable. |
| Health/provenance | Health/version calls do not upload documents; backend/version, endpoint class, config fingerprint, and input hash are captured. |
| Bounds/errors | Document/response limits, timeout, 204, bounded 503 retry, HTTP, transport, and malformed TEI are typed. |
| HTTP harness | Deterministic mock coverage verifies multipart fields, retry exhaustion, timeout, response limit, malformed response, and endpoint rejection. |
| Transport isolation | Tests use an injectable transport seam; no fixed live port or external service is required. |
| SSRF resistance | DNS/IP validation, HTTPS allowlist, redirect policy, URL credentials, alternate IP forms, and scheme rejection are tested. |
| XML safety | External entities, entity expansion, deep nesting, and oversized response cases are rejected or bounded. |
| Lifecycle budgets | Batch concurrency, cancellation, deadline, upload, memory, and retry budgets are explicit and tested. |
| TEI | Self-authored fixture covers ID, raw text, author, title, journal, date, volume, issue, pages, and DOI. |
| Fixture governance | Manifest-backed fixtures record source/license attribution and golden neutral/adapter outputs. |
| Stable provenance | `xml:id` is preserved; internal XML node numbers are not durable identifiers. |
| Schema compatibility | Neutral schema major/minor compatibility, unknown fields, and golden serialization are tested. |
| Partial input | Missing raw text emits a diagnostic; malformed XML fails explicitly. |
| Adapter | TEI to neutral record to intake candidate and queued extraction provenance. |
| CSL boundary | Adapter exposes no canonical CSL output or write path. |
| Network isolation | Default tests require no service, container, credential, model, or network. |
| Supply chain and release | `cargo deny`, coverage, package, publish dry-run, docs, and security checks pass or have recorded blockers. |
| Third-party evidence | Runtime/image/license/notice/fixture/model evidence is machine-readable and distribution claims are gated. |
| Optional live smoke | A pinned local GROBID image/tag or digest and exact command are recorded when available. |
