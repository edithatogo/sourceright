pub mod csl;
pub mod report;
pub mod sidecar;
pub mod workspace;

pub use csl::{
    CslDocument, CslItem, CslMigrationChange, CslMigrationReport, ValidationDiagnostic,
    format_csl_json, migrate_csl_document, migrate_csl_json, normalize_doi, normalize_identifier,
    normalize_item_type, normalize_title, parse_csl_json, validate_csl_json,
};
pub use report::{
    REFERENCE_REPORT_SCHEMA_VERSION, ReferenceReport, ReferenceReportCategory,
    ReferenceReportIssue, ReferenceReportJsonOutput, ReferenceReportResource,
    ReferenceReportSeverity, ReferenceReportSummary,
};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision,
    ReviewQueueEntry, ReviewStatus, ReviewStatusTransitionError, SIDECAR_SCHEMA_VERSION,
    SidecarDiagnostic, SidecarInvariantIssue, VerificationSidecar,
    format_verification_sidecar_json, parse_verification_sidecar_json,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
