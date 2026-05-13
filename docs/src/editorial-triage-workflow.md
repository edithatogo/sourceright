# Editorial Triage Workflow

This workflow is for editors, screening staff, and journal operators who need a
conservative first pass over a submission. Sourceright is not a rejection
engine. It is a citation-integrity and screening aid that can surface risks,
missing evidence, and review items for a human decision.

## Typical inputs

- Submission manuscript text or a text-layer export.
- Optional canonical CSL JSON or extracted reference text.
- Optional repository or provider evidence in the verification sidecar.

## Recommended sequence

1. Run the journal screening report.

   ```text
   sourceright journal-screen <submission.txt>
   ```

2. Inspect the reference-integrity report when you want the broader citation
   view.

   ```text
   sourceright report --json [.sourceright-directory]
   ```

3. Review unresolved items.

   ```text
   sourceright review queue [.sourceright-directory]
   ```

4. Send only the needed items back for author correction or internal review.

5. Re-run the screening report after updates so the editorial state stays
   deterministic.

## What to look for

- Missing or unstable identifiers.
- Provider conflicts that need human judgment.
- Unresolved review queue entries.
- Sidecar data that suggests extraction uncertainty.
- Style, recency, or integrity policy issues that should be escalated rather
  than auto-applied.

## Output surfaces

- Human-readable screening summary for editors.
- `sourceright.journal_screening.v1` JSON for workflow automation.
- Reference reports for follow-up action.

## Platform note

OJS is the first intended journal target in the plugin registry. Other journal
systems should be treated as adapter work until their own contract surface is
implemented.
