# Claude For Legal Compatibility Notes

## Source Patterns Reviewed

The useful patterns from Anthropic's legal workflow materials are:

- legal connectors should be read-heavy and provenance-first;
- write or filing actions need explicit confirmation gates;
- retrieved content is data, not instructions;
- rate limits and provider failures should degrade into clean review issues;
- citations should be source-attributed and attorney-reviewed;
- CourtListener is a natural public connector for litigation, legal-clinic,
  intellectual-property, and law-student workflows;
- licensed research systems should remain external subscriber-provided
  connectors, not bundled assumptions.

## Sourceright Fit

Sourceright should not become a general legal assistant. Its appropriate role is
a legal citation audit and enrichment connector:

| Pattern | Sourceright adaptation |
| --- | --- |
| CourtListener connector | Track 53 owns fixture-backed and later opt-in-live CourtListener evidence. |
| Citation verification | `sourceright legal` and `legal.analyze_citations` return citation records and review issues. |
| Attorney-review gate | Docs and MCP descriptions state that output is draft audit evidence only. |
| Provenance | Provider candidates must include source, identifier, confidence, and review state. |
| No legal advice | Legal citation reports must not answer merits, outcome, compliance, or filing questions. |
| Licensed research | Westlaw/Practical Law-style tools remain external connectors; Sourceright can audit citations before or after their reports. |

## Immediate Incorporation

This track now treats the Claude-for-Legal pattern as a connector/workflow design reference, not as a code dependency. The immediate additions are:

- `docs/src/legal-citation-audit-mcp.md` and docs-site mirror;
- tighter `legal.analyze_citations` MCP descriptions;
- legal roadmap language that frames Sourceright as evidence infrastructure;
- policy tests that preserve the no-advice, attorney-review, and
  provenance-first boundaries.

## Future Work

Before Track 53 can be completed, add:

1. A CourtListener result model with explicit `matched`, `no_match`,
   `ambiguous`, and `error` outcomes.
2. Retrieved-date, source URL, and citation-ready identifier fields in provider
   evidence.
3. Opt-in live lookup with cache, rate-limit, and clean-error behavior.
4. Example legal workflow transcripts for brief audit, demand letter audit, and
   legal clinic memo review.
5. A connector submission note that describes Sourceright as a legal citation
   audit/enrichment MCP server, not a legal research answerer.
