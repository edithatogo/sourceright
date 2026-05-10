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
