pub mod bench;
pub mod citation_sync;
pub mod cleaning;
pub mod conflict;
pub mod csl;
pub mod export;
pub mod intake;
pub mod journal;
pub mod legal;
pub mod plugins;
pub mod policy;
pub mod provenance;
pub mod providers;
pub mod reconcile;
pub mod report;
pub mod review;
pub mod sidecar;
pub mod workspace;

pub use bench::{
    BENCHMARK_MANIFEST_SCHEMA_VERSION, BenchmarkDiff, BenchmarkManifest, BenchmarkMeasure,
    BenchmarkRunReport, BenchmarkTask, BenchmarkTaskResult, run_benchmark_suite,
};
pub use citation_sync::{
    CitationSyncAction, CitationSyncAuditEntry, CitationSyncConfig, CitationSyncError,
    CitationSyncReport, RemoteCitationRecord, run_citation_sync,
};
pub use cleaning::{CleaningReport, CleaningTransformation, DuplicateGroup, standardize_document};
pub use conflict::{
    ConflictResolutionAction, ConflictResolutionDecision, ConflictResolutionPolicy,
    ConflictResolutionReport, resolve_conflicts, resolve_conflicts_with_policy,
};
pub use csl::{
    CslDocument, CslItem, CslMigrationChange, CslMigrationReport, ValidationDiagnostic,
    format_csl_json, migrate_csl_document, migrate_csl_json, normalize_doi, normalize_identifier,
    normalize_item_type, normalize_title, parse_csl_json, validate_csl_json,
};
pub use export::{
    EXPORT_MANIFEST_SCHEMA_VERSION, EXPORT_SCHEMA_VERSION, ExportArtifact, ExportFormat,
    ExportManifest, ExportManifestArtifact, ExportManifestSource, export_document, export_suite,
};
pub use intake::{
    InTextCitationCandidate, IntakeDiagnostic, IntakeDocument, IntakeResult, IntakeSourceKind,
    ReferenceCandidate, extract_in_text_citations, extract_intake, extract_references_from_text,
};
pub use journal::{
    JOURNAL_SCREENING_SCHEMA_VERSION, JournalPlatform, JournalScreeningReport,
    JournalScreeningRequest, JournalScreeningStatus, screen_journal_submission,
};
pub use legal::{
    LegalCitationIssue, LegalCitationIssueType, LegalCitationRecord, LegalCitationReport,
    LegalCitationType, LegalProvider, LegalProviderCandidate, analyze_legal_citations,
    extract_legal_citations,
};
pub use plugins::{PluginRegistryReport, discover_plugins, discover_plugins_from};
pub use policy::{
    DoiPolicy, PolicyIssue, PolicyIssueSeverity, PolicyReport, RecencyPolicy, ReferenceOrderPolicy,
    SourcerightPolicy, evaluate_policy,
};
pub use provenance::{
    ClaimNode, ClaimSourceLink, ClaimSourceLinkType, EvidenceGraph, ProvenanceIssue,
    ProvenanceIssueType, ProvenanceReport, SourceNode, analyze_claim_source_provenance,
    build_evidence_graph,
};
pub use providers::{
    AcademicProvider, AcademicProviderResult, ProviderErrorEvidence, ProviderResultStatus,
    crossref_candidate_from_work, datacite_candidate_from_work, doi_resolution_evidence,
    openalex_candidate_from_work, orcid_author_candidate_from_record, provider_error,
    pubmed_candidate_from_record,
};
pub use reconcile::{
    CitationMatch, CitationMatchConfidence, CitationOccurrence, CitationReconciliationIssue,
    CitationReconciliationIssueType, CitationReconciliationReport, CitationStyle,
    extract_citation_occurrences, reconcile_citations,
};
pub use report::{
    REFERENCE_REPORT_SCHEMA_VERSION, ReferenceReport, ReferenceReportCategory,
    ReferenceReportIssue, ReferenceReportJsonOutput, ReferenceReportResource,
    ReferenceReportSeverity, ReferenceReportSummary,
};
pub use review::{
    ReviewDecisionImport, ReviewImportError, ReviewImportReport, ReviewPartition,
    apply_review_decisions, partition_review_queue,
};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision,
    ReviewQueueEntry, ReviewStatus, ReviewStatusTransitionError, SIDECAR_SCHEMA_VERSION,
    SidecarDiagnostic, SidecarInvariantIssue, VerificationSidecar,
    format_verification_sidecar_json, parse_verification_sidecar_json,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
