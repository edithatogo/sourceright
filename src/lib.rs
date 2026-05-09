pub mod csl;
pub mod report;
pub mod sidecar;
pub mod workspace;

pub use csl::{CslDocument, CslItem, ValidationDiagnostic, validate_csl_json};
pub use report::{ReferenceReport, ReferenceReportIssue, ReferenceReportSeverity};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision, ReviewStatus,
    VerificationSidecar,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
