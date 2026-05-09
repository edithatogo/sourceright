# Journal Workflow Integrations Plan

1. Define the platform-neutral screening contract.
   - Inputs: manuscript file, extracted manuscript text, reference list text, CSL JSON, and optional sidecar data.
   - Outputs: JSON report, Markdown report, optional HTML/PDF report, review queue, and machine-readable severity summary.
   - Exit/status model: accepted, screened, blocked for extraction, screened with warnings, screened with errors.

2. Build OJS first.
   - Map OJS submission file events to Sourceright intake.
   - Add admin settings for enabling screening, output visibility, severity thresholds, and author-facing feedback.
   - Store reports as submission discussion files, editorial notes, or plugin-managed artifacts according to OJS conventions.
   - Keep plugin tests fixture-backed and separate from live OJS instance checks.

3. Add a generic webhook and batch runner.
   - Accept a submission id plus file/text location.
   - Run Sourceright CLI/MCP-compatible screening.
   - Return or write report artifacts without requiring a specific editorial platform.

4. Define enterprise adapter contracts.
   - ScholarOne: API or S3/event adapter when publisher/vendor access is available.
   - Editorial Manager: ingest/workflow adapter when API or partner access is available.
   - eJournalPress: batch/webhook adapter pending public or customer API documentation.
   - Manuscript Manager and similar SaaS systems: webhook/API adapter pending access.

5. Add security and privacy controls.
   - No manuscript data leaves the journal-controlled runtime by default.
   - Redact or minimize report excerpts where configured.
   - Separate editor-facing diagnostics from author-facing remediation.
   - Record provenance for generated reports and screening versions.

6. Add validation and release gates.
   - Fixture submissions covering clean references, missing references, hallucination-like plausible unverified references, DOI conflicts, missing DOI, and extraction failures.
   - Adapter contract tests without live editorial-platform access.
   - Optional live smoke tests gated by credentials and skipped by default.
