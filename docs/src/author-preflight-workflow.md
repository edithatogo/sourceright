# Author Preflight Workflow

This workflow is for an author who wants to check a manuscript before sending
it to a journal, repository, supervisor, or collaborator. The goal is to
separate clean CSL data from verification state, then resolve obvious citation
issues before the document leaves the local workspace.

## Inputs

- Manuscript text, a text export from the manuscript editor, or a `.docx`
  manuscript.
- `references.csl.json` as the clean canonical reference file.
- `references.verification.json` as the provider and review sidecar.
- `review-queue.jsonl` for unresolved items.

## Recommended sequence

1. Confirm the workspace layout.

   ```text
   sourceright init [.sourceright-directory]
   ```

2. Validate the canonical reference file.

   ```text
   sourceright validate-csl references.csl.json
   ```

3. Check the current integrity report.

   ```text
   sourceright report [.sourceright-directory]
   ```

4. Reconcile in-text citations against the reference list when you have a text
   manuscript or a `.docx` manuscript.

   ```text
   sourceright citations manuscript.txt
   ```

   ```text
   sourceright citations manuscript.docx
   ```

5. Inspect and resolve the review queue.

   ```text
   sourceright review queue [.sourceright-directory]
   ```

6. Export clean bibliographic files only after the canonical file and sidecar
   are in a state you are willing to share.

   ```text
   sourceright export --all [.sourceright-directory]
   ```

## Practical checks

- Keep provider evidence in `references.verification.json`, not in the CSL
  source file.
- Treat unresolved queue items as a signal to stop and inspect the source
  record, not as a reason to force export.
- Use preview or JSON output when you need to inspect results in a script.
- If your manuscript is in Word format, Sourceright extracts the manuscript
  text directly from the `.docx` package and normalizes superscript numeric
  markers before reconciliation.

## Good stopping point

The workspace is usually ready to hand off when:

- `validate-csl` passes;
- the report has no unresolved high-severity issues that matter to the handoff;
- the review queue is empty or explicitly accepted;
- the exported files are clean and format-specific.
