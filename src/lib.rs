pub mod csl;
pub mod report;
pub mod sidecar;
pub mod workspace;

pub use csl::{
    CslDocument, CslItem, ValidationDiagnostic, normalize_doi, normalize_identifier,
    normalize_item_type, normalize_title, validate_csl_json,
};
pub use report::{
    ReferenceReport, ReferenceReportCategory, ReferenceReportIssue, ReferenceReportSeverity,
};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision, ReviewStatus,
    ReviewStatusTransitionError, SidecarInvariantIssue, VerificationSidecar,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
