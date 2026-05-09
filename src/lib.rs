pub mod csl;
pub mod report;
pub mod sidecar;
pub mod workspace;

pub use csl::{
    CslDocument, CslItem, ValidationDiagnostic, format_csl_json, normalize_doi,
    normalize_identifier, normalize_item_type, normalize_title, parse_csl_json, validate_csl_json,
};
pub use report::{
    REFERENCE_REPORT_SCHEMA_VERSION, ReferenceReport, ReferenceReportCategory,
    ReferenceReportIssue, ReferenceReportJsonOutput, ReferenceReportResource,
    ReferenceReportSeverity, ReferenceReportSummary,
};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision,
    ReviewQueueEntry, ReviewStatus, ReviewStatusTransitionError, SIDECAR_SCHEMA_VERSION,
    SidecarInvariantIssue, VerificationSidecar,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
