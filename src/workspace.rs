//! On-disk Sourceright workspace layout and derived-report helpers.
//!
//! A workspace keeps canonical CSL data, verification metadata, review queue
//! output, and export artifacts together under a predictable root.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::conflict::ConflictResolutionReport;
use crate::csl::{CslDocument, format_csl_json, validate_csl_json};
use crate::export::{
    EXPORT_MANIFEST_SCHEMA_VERSION, EXPORT_SCHEMA_VERSION, ExportArtifact, ExportFormat,
    ExportManifest, ExportManifestArtifact, ExportManifestSource, export_document, export_suite,
};
use crate::journal::{JournalPlatform, JournalScreeningReport, JournalScreeningRequest};
use crate::reconcile::CitationReconciliationReport;
use crate::report::{ReferenceReport, ReferenceReportJsonOutput, ReferenceReportResource};
use crate::review::{ReviewDecisionImport, ReviewImportReport, ReviewPartition};
use crate::sidecar::{
    VerificationSidecar, format_verification_sidecar_json, parse_verification_sidecar_json,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcerightWorkspace {
    /// Workspace root directory.
    pub root: PathBuf,
    /// Canonical CSL JSON path.
    pub references_csl_json: PathBuf,
    /// Verification sidecar path.
    pub verification_sidecar_json: PathBuf,
    /// Derived review queue path.
    pub review_queue_jsonl: PathBuf,
    /// Export output directory.
    pub exports_dir: PathBuf,
}

impl SourcerightWorkspace {
    /// Derives the workspace location for a document or directory.
    pub fn for_document_or_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let root = if path.extension().is_some() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("document");
            path.with_file_name(format!("{name}.sourceright"))
        } else {
            path.join(".sourceright")
        };

        Self::from_root(root)
    }

    /// Constructs a workspace from an explicit root directory.
    pub fn from_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            references_csl_json: root.join("references.csl.json"),
            verification_sidecar_json: root.join("references.verification.json"),
            review_queue_jsonl: root.join("review-queue.jsonl"),
            exports_dir: root.join("exports"),
            root,
        }
    }

    /// Constructs a workspace from a root path or a parent directory that contains `.sourceright`.
    pub fn from_root_or_parent(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        if path.file_name().and_then(|name| name.to_str()) == Some(".sourceright")
            || path.join("references.csl.json").exists()
            || path.join("references.verification.json").exists()
            || path.join("review-queue.jsonl").exists()
        {
            return Self::from_root(path);
        }

        let nested = path.join(".sourceright");
        if nested.exists() {
            return Self::from_root(nested);
        }

        Self::from_root(path)
    }

    /// Creates the standard workspace files if they do not already exist.
    pub fn init(&self) -> Result<(), WorkspaceError> {
        fs::create_dir_all(&self.exports_dir)?;

        write_text_if_missing(
            &self.references_csl_json,
            &format_csl_json(&CslDocument::empty())?,
        )?;
        write_text_if_missing(
            &self.verification_sidecar_json,
            &format_verification_sidecar_json(&VerificationSidecar::empty())?,
        )?;

        if !self.review_queue_jsonl.exists() {
            fs::write(&self.review_queue_jsonl, "")?;
        }

        Ok(())
    }

    /// Validates a CSL JSON file and returns rendered diagnostics.
    pub fn validate_csl_file(path: impl AsRef<Path>) -> Result<Vec<String>, WorkspaceError> {
        let input = fs::read_to_string(path)?;
        let diagnostics = validate_csl_json(&input)?;
        Ok(diagnostics
            .into_iter()
            .map(|diagnostic| {
                format!(
                    "{} {} {}",
                    diagnostic.code, diagnostic.path, diagnostic.message
                )
            })
            .collect())
    }

    /// Returns the reference report as Markdown.
    pub fn reference_report_markdown(&self) -> Result<String, WorkspaceError> {
        Ok(self.reference_report()?.to_markdown())
    }

    /// Returns the reference report in JSON output shape.
    pub fn reference_report_json(&self) -> Result<ReferenceReportJsonOutput, WorkspaceError> {
        Ok(self.reference_report()?.to_json_output())
    }

    /// Returns the reference report as an MCP resource payload.
    pub fn reference_report_mcp_resource(&self) -> Result<ReferenceReportResource, WorkspaceError> {
        Ok(self.reference_report()?.to_mcp_resource())
    }

    /// Rebuilds the review queue JSONL file from the verification sidecar.
    pub fn refresh_review_queue(&self) -> Result<(), WorkspaceError> {
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        fs::write(&self.review_queue_jsonl, sidecar.to_review_queue_jsonl()?)?;
        Ok(())
    }

    /// Exports references in a single requested format or the full suite.
    pub fn export_references(
        &self,
        format: Option<ExportFormat>,
    ) -> Result<Vec<ExportArtifact>, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        Ok(match format {
            Some(format) => vec![export_document(&csl, format)],
            None => export_suite(&csl),
        })
    }

    /// Writes export artifacts to the workspace export directory.
    pub fn write_exports(
        &self,
        format: Option<ExportFormat>,
    ) -> Result<Vec<PathBuf>, WorkspaceError> {
        fs::create_dir_all(&self.exports_dir)?;
        let artifacts = self.export_references(format)?;
        let mut paths = Vec::new();
        for artifact in artifacts {
            let path = self.exports_dir.join(&artifact.filename);
            fs::write(&path, artifact.content)?;
            paths.push(path);
        }
        Ok(paths)
    }

    /// Returns the export manifest without writing files.
    pub fn export_manifest(
        &self,
        format: Option<ExportFormat>,
    ) -> Result<ExportManifest, WorkspaceError> {
        let artifacts = self.export_references(format)?;
        Ok(ExportManifest {
            schema_version: EXPORT_MANIFEST_SCHEMA_VERSION.to_string(),
            source: ExportManifestSource {
                references_csl_json: self.references_csl_json.display().to_string(),
                verification_sidecar_json: self.verification_sidecar_json.display().to_string(),
            },
            artifacts: artifacts
                .into_iter()
                .map(|artifact| {
                    let path = self.exports_dir.join(&artifact.filename);
                    ExportManifestArtifact {
                        format: artifact.format,
                        filename: path.display().to_string(),
                        content_type: artifact.format.content_type().to_string(),
                        schema_version: EXPORT_SCHEMA_VERSION.to_string(),
                    }
                })
                .collect(),
        })
    }

    /// Returns the cross-file reference integrity report.
    pub fn reference_report(&self) -> Result<ReferenceReport, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        Ok(ReferenceReport::from_documents(&csl, &sidecar))
    }

    /// Returns a conflict-resolution report derived from the workspace files.
    pub fn conflict_resolution_report(&self) -> Result<ConflictResolutionReport, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        Ok(crate::conflict::resolve_conflicts(&csl, &sidecar))
    }

    /// Returns a citation-reconciliation report for manuscript text.
    pub fn citation_reconciliation_report(
        &self,
        manuscript_text: &str,
    ) -> Result<CitationReconciliationReport, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        Ok(crate::reconcile::reconcile_citations(manuscript_text, &csl))
    }

    /// Splits the review queue into bounded partitions.
    pub fn review_queue_partitions(
        &self,
        max_entries: usize,
    ) -> Result<Vec<ReviewPartition>, WorkspaceError> {
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        Ok(crate::review::partition_review_queue(&sidecar, max_entries))
    }

    /// Applies review decisions and persists the updated sidecar and queue.
    pub fn import_review_decisions(
        &self,
        decisions: &[ReviewDecisionImport],
    ) -> Result<ReviewImportReport, WorkspaceError> {
        let input = fs::read_to_string(&self.verification_sidecar_json)?;
        let mut sidecar = parse_verification_sidecar_json(&input)?;
        let report = crate::review::apply_review_decisions(&mut sidecar, decisions);
        fs::write(
            &self.verification_sidecar_json,
            format_verification_sidecar_json(&sidecar)?,
        )?;
        fs::write(&self.review_queue_jsonl, sidecar.to_review_queue_jsonl()?)?;
        Ok(report)
    }

    /// Returns a journal screening report for the current workspace state.
    pub fn journal_screening_report(
        &self,
        submission_id: String,
        platform: JournalPlatform,
        manuscript_label: String,
    ) -> Result<JournalScreeningReport, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        Ok(crate::journal::screen_journal_submission(
            JournalScreeningRequest {
                submission_id,
                platform,
                manuscript_label,
            },
            &csl,
            &sidecar,
        ))
    }
}

fn write_text_if_missing(path: &Path, value: &str) -> Result<(), WorkspaceError> {
    if path.exists() {
        return Ok(());
    }

    fs::write(path, value)?;
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, WorkspaceError> {
    let input = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}

/// Errors raised while reading or writing workspace files.
#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_workspace_files() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());

        workspace.init().expect("init workspace");

        assert!(workspace.references_csl_json.exists());
        assert!(workspace.verification_sidecar_json.exists());
        assert!(workspace.review_queue_jsonl.exists());
        assert!(workspace.exports_dir.is_dir());
    }

    #[test]
    fn report_reads_workspace_and_returns_markdown() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");

        let report = workspace
            .reference_report_markdown()
            .expect("generate reference report");

        assert!(report.contains("Sourceright Reference Report"));
        assert!(report.contains("Total references: 0"));
    }

    #[test]
    fn report_reads_workspace_and_returns_json_and_mcp_resource() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");

        let report = workspace
            .reference_report_json()
            .expect("generate report json");
        let resource = workspace
            .reference_report_mcp_resource()
            .expect("generate report resource");

        assert_eq!(report.schema_version, "sourceright.reference_report.v1");
        assert_eq!(report.summary.total_references, 0);
        assert_eq!(resource.uri, "sourceright://reports/reference-integrity");
        assert_eq!(resource.mime_type, "application/json");
    }

    #[test]
    fn refresh_review_queue_writes_derived_jsonl() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");

        fs::write(
            &workspace.verification_sidecar_json,
            r#"{"schema_version":"sourceright.verification.v1","references":{"queued":{"review_status":"queued"}}}"#,
        )
        .expect("write sidecar");

        workspace
            .refresh_review_queue()
            .expect("refresh review queue");
        let jsonl = fs::read_to_string(&workspace.review_queue_jsonl).expect("read queue");

        assert_eq!(
            jsonl,
            r#"{"id":"queued","review_status":"queued"}"#.to_string() + "\n"
        );
    }

    #[test]
    fn write_exports_generates_clean_suite_files() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Trial","DOI":"10.1000/example"}]"#,
        )
        .expect("write references");

        let paths = workspace.write_exports(None).expect("write exports");

        assert_eq!(paths.len(), 5);
        assert!(workspace.exports_dir.join("references.ris").exists());
        assert!(
            fs::read_to_string(workspace.exports_dir.join("references.ris"))
                .expect("read ris")
                .contains("DO  - 10.1000/example")
        );
    }

    #[test]
    fn export_manifest_previews_outputs_without_writing_files() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Trial"}]"#,
        )
        .expect("write references");

        let manifest = workspace
            .export_manifest(Some(ExportFormat::Ris))
            .expect("preview export manifest");

        assert_eq!(manifest.schema_version, "sourceright.export_manifest.v1");
        assert_eq!(manifest.artifacts.len(), 1);
        assert_eq!(manifest.artifacts[0].format, ExportFormat::Ris);
        assert!(manifest.artifacts[0].filename.ends_with("references.ris"));
        assert!(!workspace.exports_dir.join("references.ris").exists());
    }

    #[test]
    fn conflict_resolution_report_reads_workspace_evidence() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Trial"}]"#,
        )
        .expect("write references");
        fs::write(
            &workspace.verification_sidecar_json,
            r#"{"schema_version":"sourceright.verification.v1","references":{"smith-2024":{"provider_candidates":[{"provider":"crossref","confidence":0.98,"retrieved_at":"2026-05-10T00:00:00Z","data":{"DOI":"10.1000/example"}}],"review_status":"not_required"}}}"#,
        )
        .expect("write sidecar");

        let report = workspace
            .conflict_resolution_report()
            .expect("read conflict report");

        assert_eq!(
            report.document.items[0].doi.as_deref(),
            Some("10.1000/example")
        );
        assert_eq!(report.decisions.len(), 1);
    }

    #[test]
    fn citation_reconciliation_report_reads_workspace_references() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Trial","author":[{"family":"Smith"}]}]"#,
        )
        .expect("write references");

        let report = workspace
            .citation_reconciliation_report("Text cites (Smith, 2024).")
            .expect("reconcile citations");

        assert_eq!(report.matches[0].reference_id, "smith-2024");
    }

    #[test]
    fn review_decision_import_updates_sidecar_and_queue() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.verification_sidecar_json,
            r#"{"schema_version":"sourceright.verification.v1","references":{"queued":{"review_status":"queued"}}}"#,
        )
        .expect("write sidecar");

        let report = workspace
            .import_review_decisions(&[ReviewDecisionImport {
                reference_id: "queued".to_string(),
                decision: "accepted".to_string(),
                reviewer: "agent:test".to_string(),
                decided_at: "2026-05-10T00:00:00Z".to_string(),
                status: crate::sidecar::ReviewStatus::Resolved,
                notes: None,
            }])
            .expect("import decisions");

        assert_eq!(report.applied, 1);
        assert_eq!(
            fs::read_to_string(&workspace.review_queue_jsonl).expect("read queue"),
            ""
        );
    }

    #[test]
    fn journal_screening_report_reads_workspace() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Trial"}]"#,
        )
        .expect("write references");

        let report = workspace
            .journal_screening_report(
                "SUB-1".to_string(),
                JournalPlatform::Ojs,
                "manuscript.docx".to_string(),
            )
            .expect("screen submission");

        assert_eq!(report.submission_id, "SUB-1");
        assert_eq!(report.platform, JournalPlatform::Ojs);
    }
}
