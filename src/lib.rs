pub mod cleaning;
pub mod csl;
pub mod export;
pub mod intake;
pub mod providers;
pub mod report;
pub mod sidecar;
pub mod workspace;

pub use cleaning::{CleaningReport, CleaningTransformation, DuplicateGroup, standardize_document};
pub use csl::{
    CslDocument, CslItem, CslMigrationChange, CslMigrationReport, ValidationDiagnostic,
    format_csl_json, migrate_csl_document, migrate_csl_json, normalize_doi, normalize_identifier,
    normalize_item_type, normalize_title, parse_csl_json, validate_csl_json,
};
pub use export::{
    EXPORT_SCHEMA_VERSION, ExportArtifact, ExportFormat, export_document, export_suite,
};
pub use intake::{
    InTextCitationCandidate, IntakeDiagnostic, IntakeDocument, IntakeResult, IntakeSourceKind,
    ReferenceCandidate, extract_in_text_citations, extract_intake, extract_references_from_text,
};
pub use providers::{
    AcademicProvider, AcademicProviderResult, ProviderErrorEvidence, ProviderResultStatus,
    crossref_candidate_from_work, datacite_candidate_from_work, doi_resolution_evidence,
    openalex_candidate_from_work, orcid_author_candidate_from_record, provider_error,
    pubmed_candidate_from_record,
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
