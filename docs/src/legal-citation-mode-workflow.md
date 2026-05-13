# Legal Citation Mode Workflow

Legal citations are a separate mode, not a variation of academic CSL. The
purpose is to extract and report legal citation evidence without forcing it into
the academic reference model.

## Recommended sequence

1. Prepare a plain text input file with the legal material you want to screen.

2. Run the legal extractor.

   ```text
   sourceright legal <legal-text.txt>
   ```

3. Review the report output and the issues it surfaces.

4. Keep the legal records separate from `references.csl.json`.

## What the mode covers

- Neutral citations.
- Case and legislation mentions.
- Jurisdiction and court hints.
- Public-provider candidates.
- Review issues for uncertain matches.

## What it does not do

- It does not rewrite academic CSL into a legal model.
- It does not assert that a cited proposition is true.
- It does not collapse legal records into the same clean bibliography surface
  used for journal articles and books.

## Provider note

The legal provider registry currently treats CourtListener as the first public
provider slot. That is a contract and roadmap signal, not a claim that all live
legal lookups are already complete.
