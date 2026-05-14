---
title: Legal Citation Mode Workflow
description: Separate handling for legal citation extraction and review.
---

Legal citation review stays separate from academic CSL. Use the legal report to
identify candidate citations, jurisdictions, providers, and issues without
mixing those records into bibliographic exports.

This mode is not a legal filing compliance system.

## Recommended Sequence

1. Prepare a plain text input file with the legal material you want to screen.

2. Run the legal extractor.

   ```text
   sourceright legal legal-text.txt
   ```

3. Review the report output and the issues it surfaces.

4. Keep the legal records separate from `references.csl.json`.

5. Route unresolved records to attorney review or to a licensed legal research
   system. Do not turn missing provider evidence into a legal conclusion.

## What The Mode Covers

- Neutral citations.
- Case and legislation mentions.
- Jurisdiction and court hints.
- Public-provider candidates.
- Review issues for uncertain matches.
- Draft audit evidence that can be consumed by CLI, CI, or MCP workflows.

## What It Does Not Do

- It does not rewrite academic CSL into a legal model.
- It does not assert that a cited proposition is true.
- It does not collapse legal records into the same clean bibliography surface
  used for journal articles and books.
- It does not provide legal advice, predict outcomes, or decide whether a
  filing is compliant.
- It does not replace attorney review.

## Provider Note

The legal provider registry currently treats CourtListener as the first public
provider slot. That is a contract and roadmap signal, not a claim that all live
legal lookups are already complete.

For a connector-oriented view, see
[Legal Citation Audit MCP Pack](legal-citation-audit-mcp).
