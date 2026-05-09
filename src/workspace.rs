use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::csl::{CslDocument, validate_csl_json};
use crate::report::ReferenceReport;
use crate::sidecar::VerificationSidecar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcerightWorkspace {
    pub root: PathBuf,
    pub references_csl_json: PathBuf,
    pub verification_sidecar_json: PathBuf,
    pub review_queue_jsonl: PathBuf,
    pub exports_dir: PathBuf,
}

impl SourcerightWorkspace {
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

    pub fn init(&self) -> Result<(), WorkspaceError> {
        fs::create_dir_all(&self.exports_dir)?;

        write_json_if_missing(&self.references_csl_json, &CslDocument::empty())?;
        write_json_if_missing(
            &self.verification_sidecar_json,
            &VerificationSidecar::empty(),
        )?;

        if !self.review_queue_jsonl.exists() {
            fs::write(&self.review_queue_jsonl, "")?;
        }

        Ok(())
    }

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

    pub fn reference_report_markdown(&self) -> Result<String, WorkspaceError> {
        let csl: CslDocument = read_json(&self.references_csl_json)?;
        let sidecar: VerificationSidecar = read_json(&self.verification_sidecar_json)?;
        Ok(ReferenceReport::from_documents(&csl, &sidecar).to_markdown())
    }
}

fn write_json_if_missing<T: serde::Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), WorkspaceError> {
    if path.exists() {
        return Ok(());
    }

    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, WorkspaceError> {
    let input = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}

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
}
