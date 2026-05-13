# University Repository Workflow

This workflow is for institutional repositories, faculty deposit services, and
library-managed archive pipelines. The emphasis is on clean bibliographic data,
deterministic exports, and explicit evidence about what was verified.

## Typical use cases

- Accepted-manuscript deposits.
- Repository metadata checks.
- Citation-file export for repository ingest.
- Post-deposit verification of reference records.

## Recommended sequence

1. Confirm the workspace and validate the canonical CSL file.

   ```text
   sourceright init [.sourceright-directory]
   sourceright validate-csl references.csl.json
   ```

2. Check the integrity report and review queue.

   ```text
   sourceright report [.sourceright-directory]
   sourceright review queue [.sourceright-directory]
   ```

3. Export the file formats your repository ingests.

   ```text
   sourceright export --preview --all [.sourceright-directory]
   ```

4. Apply the export only after the preview looks correct.

   ```text
   sourceright export --all [.sourceright-directory]
   ```

## Repository boundaries

- Repository records belong in provider evidence, not in canonical CSL.
- Clean exports stay bibliographic.
- Evidence about repository landing pages, deposits, and identifiers should stay
  in the sidecar or derived reports.

## Planned integration surface

The registry already names repository-oriented plugin families, including the
repository-record provider path. Those manifests are contracts first. They
should be treated as fixture-backed guidance until the live adapter path is
explicitly wired and documented.
