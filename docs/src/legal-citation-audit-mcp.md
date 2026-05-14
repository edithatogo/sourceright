# Legal Citation Audit MCP Pack

Sourceright can be used as a narrow legal citation audit and enrichment connector.
The useful boundary is deliberately smaller than a general legal research
assistant:

- extract legal citations from a draft, memo, brief section, chronology, or
  demand letter;
- identify courts, jurisdictions, dates, provider candidates, and citation
  model issues;
- enrich records with public-source provenance where an adapter exists;
- return confidence, conflicts, no-match states, and review flags;
- keep all outputs as draft audit evidence for attorney review.

It should not answer legal questions, predict outcomes, decide whether a cause
of action exists, draft filed material as final work product, or claim legal
compliance. It is a citation-integrity tool that can sit beside legal research
connectors, document systems, and practice-area workflows.

## Connector Contract

A legal MCP connector using Sourceright should stay read-heavy and
provenance-first:

| Requirement | Sourceright position |
| --- | --- |
| Read-heavy tools | `legal.analyze_citations` is read-only and operates on a local text file. |
| Provenance in results | Legal records include jurisdiction/provider candidates, confidence, and review issues. |
| No instruction-like retrieved content | Provider evidence is data; it must never be treated as instructions. |
| Explicit write gates | The legal citation surface has no write operation. Other MCP writes remain dry-run first and require `apply: true`. |
| Graceful failure | Missing provider matches and uncertain jurisdictions become review issues, not fabricated certainty. |
| Attorney review | Every output remains draft audit evidence for a reviewing lawyer. |

## Recommended Workflow

1. Export or save the legal text to a plain text file.

2. Run the CLI audit:

   ```text
   sourceright legal legal-text.txt
   ```

3. Or call the MCP tool from a local stdio client:

   ```json
   {
     "name": "legal.analyze_citations",
     "arguments": {
       "path": "legal-text.txt"
     }
   }
   ```

4. Review the returned records:

   - citation text;
   - citation type;
   - jurisdiction and court hints;
   - provider candidates;
   - confidence scores;
   - missing-jurisdiction, missing-year, missing-provider, and ambiguous-type issues.

5. Keep the legal report separate from `references.csl.json`.

6. Hand unresolved records to attorney review or to a licensed legal research
   system. Sourceright should not convert uncertainty into legal conclusions.

## Practice-Area Hooks

The pack is most useful when embedded as a checkpoint inside legal workflows:

| Workflow | How Sourceright helps |
| --- | --- |
| Brief section review | Extract citations and flag missing provider evidence before attorney review. |
| Demand letter drafting | Audit cited authorities before any send gate. |
| Litigation chronology | Separate case/statute citations from factual source references. |
| Legal clinic memo review | Show which cited authorities need verification or jurisdiction review. |
| Docket or matter intake | Preserve citation audit evidence beside the matter file without modifying it. |
| Claim/source provenance | Link legal citations to cited propositions without asserting claim truth. |

## Provider Roadmap

CourtListener is the first public legal-provider path for United States case law.
The immediate integration model is:

- fixture-backed CourtListener response parsing by default;
- optional live lookup only after cache, rate-limit, and auth behavior are
  documented;
- explicit `matched`, `no_match`, `ambiguous`, and `error` states;
- source URL, retrieved date, and citation-ready identifier in provider
  evidence;
- no mutation of academic CSL records.

Other provider lanes should remain separate:

- AustLII and Australian legislation registers for Australian material;
- Caselaw Access Project for historical United States cases;
- official legislation and regulation registers where access terms allow;
- licensed systems such as Westlaw or Practical Law only as external
  subscriber-provided research connectors.

## Compatibility Pitch

A concise connector description for a Claude legal workflow is:

> Sourceright audits and enriches legal citations. It extracts legal citation
> records, identifies jurisdiction/provider candidates, surfaces conflicts and
> no-match states, and returns provenance-rich review flags. It does not provide
> legal advice, predict outcomes, or draft final legal work product.

This keeps the value proposition specific enough to be useful to legal
workflows while preserving the product philosophy: evidence infrastructure
first, attorney judgment last.
