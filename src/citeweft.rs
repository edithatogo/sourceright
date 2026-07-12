//! Backend-neutral scholarly extraction boundary.
//!
//! CiteWeft is a provisional internal name. This module deliberately has no
//! dependency on Sourceright CSL, sidecar, CLI, MCP, or provider types.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const SCHEMA_VERSION: &str = "citeweft.scholarly-document.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScholarlyDocument {
    pub schema_version: String,
    pub references: Vec<ReferenceRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    pub provenance: EngineProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceRecord {
    pub id: String,
    pub raw_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifiers: Vec<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<TextSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub scheme: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSpan {
    pub surface: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineProvenance {
    pub backend: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    pub configuration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCapabilities {
    pub references: bool,
    pub callouts: bool,
    pub coordinates: bool,
    pub entities: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionOptions {
    pub include_references: bool,
    pub include_entities: bool,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_references: true,
            include_entities: false,
        }
    }
}

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("unsupported extraction configuration: {0}")]
    UnsupportedConfiguration(String),
    #[error("extraction backend failed: {0}")]
    Backend(String),
}

pub trait ScholarlyDocumentExtractor {
    fn capabilities(&self) -> BackendCapabilities;
    fn extract(
        &self,
        document_bytes: &[u8],
        options: &ExtractionOptions,
    ) -> Result<ScholarlyDocument, ExtractionError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityMention {
    pub class_scheme: String,
    pub class_id: String,
    pub text: String,
    pub span: TextSpan,
}

pub trait EntityRecognizer {
    fn recognize(
        &self,
        document: &ScholarlyDocument,
    ) -> Result<Vec<EntityMention>, ExtractionError>;
}
