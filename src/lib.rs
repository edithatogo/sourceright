//! Sourceright provides canonical data structures and helpers for reference
//! verification workflows.
//!
//! The crate centers on canonical CSL JSON, verification sidecars, derived
//! reports, and workspace-level file management.
//!
//! ```
//! use sourceright::{CslDocument, ReferenceReport, VerificationSidecar};
//!
//! let report = ReferenceReport::from_documents(&CslDocument::empty(), &VerificationSidecar::empty());
//! assert_eq!(report.total_references, 0);
//! ```

#![forbid(unsafe_code)]

pub mod bench;
pub mod citation_sync;
pub use ::citeweft::citeweft;
pub mod citeweft_adapter;
pub mod cleaning;
pub mod conflict;
pub mod csl;
pub use ::citeweft::entity_model;
pub mod export;
pub mod extraction_benchmark;
pub mod grobid;
pub mod intake;
pub mod interoperability;
pub mod journal;
pub use ::citeweft::layout;
pub mod legal;
pub mod live_providers;
pub mod plugins;
pub mod policy;
pub mod provenance;
pub mod providers;
pub mod reconcile;
pub use ::citeweft::reference_model;
pub mod report;
pub mod review;
pub use ::citeweft::routing;
pub mod sidecar;
pub mod workspace;

pub use bench::{
    BENCHMARK_MANIFEST_SCHEMA_VERSION, BenchmarkDiff, BenchmarkManifest, BenchmarkMeasure,
    BenchmarkRunReport, BenchmarkTask, BenchmarkTaskResult, run_benchmark_suite,
};
pub use citation_sync::{
    CitationSyncAction, CitationSyncAuditEntry, CitationSyncConfig, CitationSyncError,
    CitationSyncReport, CitationSyncSuggestionKind, RemoteCitationRecord, run_citation_sync,
};
pub use citeweft::{
    BackendCapabilities, Diagnostic as ExtractionDiagnostic, DiagnosticSeverity, EngineProvenance,
    EntityMention, EntityRecognizer, ExtractionOptions, Identifier, ReferenceRecord,
    ScholarlyDocument, ScholarlyDocumentExtractor, TextSpan,
};
pub use citeweft_adapter::{SourcerightExtraction, adapt_scholarly_document};
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
pub use entity_model::{
    DEFAULT_ENTITY_MODEL_MAX_INPUT_BYTES, DeterministicEntityRecognizer,
    ENTITY_MODEL_SCHEMA_VERSION, ENTITY_VOCABULARY_VERSION, EntityClass, EntityDiagnostic,
    EntityLinkCandidate, EntityMapping, EntityMention as NativeEntityMention, EntityModelError,
    EntityModelProvenance, EntityModelReport, EntityPattern, MappingRelation,
    general_scholarly_vocabulary,
};
pub use export::{
    EXPORT_MANIFEST_SCHEMA_VERSION, EXPORT_SCHEMA_VERSION, ExportArtifact, ExportFormat,
    ExportManifest, ExportManifestArtifact, ExportManifestSource, export_document, export_suite,
};
pub use extraction_benchmark::{
    Availability as ExtractionAvailability, BackendMetadata as ExtractionBackendMetadata,
    CountMetric as ExtractionCountMetric, EXTRACTION_BENCHMARK_SCHEMA_VERSION,
    ExtractionBenchmarkError, ExtractionBenchmarkFixture, ExtractionBenchmarkManifest,
    ExtractionBenchmarkReport, ExtractionCohort, ExtractionStageOutput,
    FixtureReport as ExtractionFixtureReport, OperationMetadata as ExtractionOperationMetadata,
    Score as ExtractionScore, run_extraction_benchmark,
};
pub use grobid::{GrobidConfig, GrobidError, GrobidExtractor, GrobidHealth, decode_references_tei};
pub use intake::{
    InTextCitationCandidate, IntakeDiagnostic, IntakeDocument, IntakeResult, IntakeSourceKind,
    ManuscriptReadError, ReferenceCandidate, extract_in_text_citations, extract_intake,
    extract_references_from_text, read_manuscript_text,
};
pub use interoperability::{
    DifferenceClass, INTEROPERABILITY_REPORT_SCHEMA_VERSION, InteroperabilityReport,
    SemanticDifference, compare_csl_json,
};
pub use journal::{
    JOURNAL_SCREENING_SCHEMA_VERSION, JournalPlatform, JournalScreeningReport,
    JournalScreeningRequest, JournalScreeningStatus, screen_journal_submission,
};
pub use layout::{
    FixtureTextLayoutExtractor, LAYOUT_SCHEMA_VERSION, LayoutBlock, LayoutBox, LayoutDiagnostic,
    LayoutDiagnosticCode, LayoutDocument, LayoutError, LayoutExtractor, LayoutLimits, LayoutPage,
    LayoutProvenance, LayoutStyle, LayoutToken,
};
pub use legal::{
    LegalCitationIssue, LegalCitationIssueType, LegalCitationRecord, LegalCitationReport,
    LegalCitationType, LegalProvider, LegalProviderCandidate, analyze_legal_citations,
    courtlistener_fixture_candidate, extract_legal_citations,
};
pub use live_providers::{
    LIVE_PROVIDER_SMOKE_SCHEMA_VERSION, LiveProviderConfig, LiveProviderExecution,
    LiveProviderOutcome, LiveProviderRuntimeControls, LiveProviderSmokeState,
    live_provider_config_from_env, live_provider_smoke_report, live_provider_smoke_report_from_env,
};
pub use plugins::{PluginRegistryReport, discover_plugins, discover_plugins_from};
pub use policy::{
    DoiPolicy, PolicyIssue, PolicyIssueSeverity, PolicyReport, RecencyPolicy, ReferenceOrderPolicy,
    SourcerightPolicy, evaluate_policy, evaluate_policy_with_verification,
    provider_backed_recency_issues, provider_backed_url_archive_issues,
};
pub use provenance::{
    ClaimNode, ClaimSourceLink, ClaimSourceLinkType, EvidenceGraph, ProvenanceIssue,
    ProvenanceIssueType, ProvenanceReport, SourceNode, analyze_claim_source_provenance,
    build_evidence_graph,
};
pub use providers::{
    AcademicProvider, AcademicProviderResult, ProviderErrorEvidence, ProviderResultDiagnostic,
    ProviderResultDiagnosticKind, ProviderResultStatus, crossref_candidate_from_work,
    datacite_candidate_from_work, doi_resolution_evidence, openalex_candidate_from_work,
    orcid_author_candidate_from_record, provider_error, provider_result_diagnostic,
    pubmed_candidate_from_record,
};
pub use reconcile::{
    CitationMatch, CitationMatchConfidence, CitationOccurrence, CitationReconciliationIssue,
    CitationReconciliationIssueType, CitationReconciliationReport, CitationStyle,
    extract_citation_occurrences, reconcile_citations,
};
pub use reference_model::{
    CitationCallout, DEFAULT_REFERENCE_MODEL_MAX_INPUT_BYTES, DeterministicReferenceModel,
    ExtractionStatus as ReferenceExtractionStatus, FieldEvidence, ModelDiagnostic,
    REFERENCE_MODEL_SCHEMA_VERSION, ReferenceCandidate as ReferenceModelCandidate, ReferenceFields,
    ReferenceModelError, ReferenceModelProvenance, ReferenceModelReport, SourceSpan,
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
pub use routing::{
    AttemptOutcome, BackendAvailability, BackendKind, CalibrationScore,
    DEFAULT_ROUTING_MAX_INPUT_BYTES, ROUTING_SCHEMA_VERSION, RouteAttempt, RouteDecision,
    RouteMode, RoutePolicy, RouteRequest, RouteTrace, cache_key, redact_diagnostic, route,
};
pub use sidecar::{
    ExtractionProvenance, ProviderCandidate, ReferenceVerification, ReviewDecision,
    ReviewQueueEntry, ReviewStatus, ReviewStatusTransitionError, SIDECAR_SCHEMA_VERSION,
    SidecarDiagnostic, SidecarInvariantIssue, VerificationSidecar,
    format_verification_sidecar_json, parse_verification_sidecar_json,
};
pub use workspace::{SourcerightWorkspace, WorkspaceError};
