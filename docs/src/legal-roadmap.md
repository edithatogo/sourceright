# Legal Citation Model

Legal citations are modelled separately from academic CSL JSON. They should not be forced into CSL semantics where the data model does not fit.

The current legal model represents:

- Cases.
- Legislation and regulations.
- Neutral citations.
- Courts and jurisdictions.
- Pinpoint references.
- Public-provider candidates and legal review issues.

The first extractor detects neutral citations such as `[2016] HCA 1` and simple legislation/regulation mentions. Provider slots are explicit for public sources including CourtListener, Caselaw Access Project, AustLII, and legislation registers. Use `sourceright legal <legal-text.txt>` to produce the baseline legal citation report.

Subsequent treatment, reporter series, richer jurisdiction inference, and live provider adapters remain later provider-expansion work.

## Philosophy

The legal roadmap is evidence infrastructure, not legal reasoning automation.
The product should help a reviewer see which citations are present, which
jurisdiction/provider assumptions were made, which authorities need checking,
and which records have no reliable provider evidence. It should not tell the
user what legal position to take, provide legal advice, or convert audit
evidence into a legal conclusion.

Legal work should preserve these boundaries:

- legal records stay separate from academic CSL;
- public-provider evidence never silently overwrites a canonical record;
- unresolved or ambiguous results become review issues;
- practice workflows receive audit evidence, not final legal conclusions;
- licensed research systems remain external subscriber-provided connectors.

## Workflow Targets

The most useful near-term legal workflows are:

| Workflow | Sourceright role |
| --- | --- |
| Brief section audit | Extract citations and flag missing provider evidence before attorney review. |
| Demand letter audit | Check cited authorities before any send gate. |
| Legal clinic memo review | Provide a readable citation verification queue for supervising attorneys. |
| Litigation chronology | Separate case/statute citations from factual-source provenance. |
| Claim/source provenance | Link legal citations to propositions without asserting claim truth. |

## Provider Priorities

CourtListener remains the first public live-provider candidate for United
States case law. The next useful increment is an opt-in provider path that
preserves fixture-backed default tests, cache/rate-limit controls, and explicit
`matched`, `no_match`, `ambiguous`, and `error` outcomes.

Australian and non-United States sources should be separate provider lanes.
AustLII, official legislation registers, Caselaw Access Project, and licensed
research connectors should not be collapsed into one generic legal source.
