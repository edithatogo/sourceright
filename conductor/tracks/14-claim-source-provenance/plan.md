# Claim Source Provenance Plan

1. Define claim and evidence graph concepts after reference verification stabilizes. Implemented through `EvidenceGraph`, `ClaimNode`, `SourceNode`, and links.
2. Add claim-to-citation linking from reconciled citations. Implemented by linking detected citation occurrences to claim spans.
3. Add quote and paraphrase verification workflows. Implemented as quote-without-citation provenance issues; semantic paraphrase checks remain future provider work.
4. Add source-quality and provenance reports. Implemented as JSON and Markdown provenance reports with unsupported-claim diagnostics.
5. Expose the layer through CLI/MCP once reliable fixtures exist. Implemented through `sourceright provenance` and MCP status resource declarations.
