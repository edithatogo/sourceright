# Future Scholarly Communication Contracts

## Contract principles

All future publishing workflows should be contract-first. A contract is a machine-readable agreement about inputs, outputs, provenance, validation status, human approval, and claim boundaries. Contracts make it possible for humans and agents to act on the same research record without depending on hidden process state.

## Contract family overview

| Contract | Purpose | Primary consumers |
| --- | --- | --- |
| `sourceright.research_package.v1` | Complete submission package and graph root | Platform, agents, editors, indexers |
| `sourceright.journal_article_requirements.v1` | Journal, article type, and submission requirements | Authors, editors, requirement agents |
| `sourceright.requirements_assessment.v1` | Assessment against journal and article requirements | Editors, authors, automation gates |
| `sourceright.reporting_checklist_contract.v1` | Deduplicated reporting checklist bundle | Authors, reviewers, checklist agents |
| `sourceright.reporting_assessment.v1` | Checklist compliance and MoSCoW gaps | Editors, authors, reviewers |
| `sourceright.agent_workflow.v1` | Agent workflow policy, task graph, and audit requirements | Workflow engine, editors, auditors |
| `sourceright.peer_review_contribution.v1` | Human or agent review record | Editors, reviewers, indexers |
| `sourceright.reproducibility_evidence.v1` | Code, data, environment, rerun and replication status | Reviewers, editors, readers |
| `sourceright.presentation_surface.v1` | Blog, preprint, slides, audio, video, monograph, chat view | Readers, dissemination agents |
| `sourceright.contribution_ledger.v1` | Credited author, reviewer, editor, verification, data, software, and platform work | Contributors, funders, institutions |
| `sourceright.project_knowledge_graph.v1` | Evolving graph of questions, claims, evidence, versions, outputs, and decisions | Indexers, evidence synthesis, readers |

## Research package contract

Minimum fields:

```yaml
schema_version: sourceright.research_package.v1
package_id: string
project_title: string
project_scope: string
status: draft | verified_private | preprint_ready | preprint_posted | under_review | reviewed_preprint | accepted | published | updated | retracted | superseded
created_at: datetime
updated_at: datetime
version: string
license: string
embargo: null | object
contributors: []
manuscript:
  canonical_format: markdown | myst | jats_xml | docx_text | latex | other
  human_readable_uri: string
  machine_readable_uri: string
references:
  csl_json_uri: string
  biblatex_uri: string | null
  ris_uri: string | null
data: []
code: []
protocols: []
ethics_and_governance: []
journal_targets: []
reporting_checklists: []
reviews: []
dissemination_outputs: []
knowledge_graph_uri: string
```

## Journal-article requirements contract

This contract exists so a journal-specific submission workflow can be sourced, reviewed, applied, and repeated.

```yaml
schema_version: sourceright.journal_article_requirements.v1
contract_id: string
journal:
  name: string
  publisher: string
  issn: string | null
  url: string
article_type: string
source_evidence:
  - label: string
    url: string
    retrieved_at: datetime
    source_type: author_instructions | editorial_policy | data_policy | ethics_policy | ai_policy | reporting_policy | submission_system_policy
    freshness_status: current | stale | unknown
requirements:
  - id: string
    text: string
    category: scope | format | structure | word_limit | references | data | code | ethics | ai_use | reporting | figures | supplementary | license | fees | peer_review | preprint | conflict_of_interest
    moscow_default: must | should | could
    source_ids: []
    machine_checkable: boolean
    assessment_method: deterministic | heuristic | agentic | human
human_review:
  reviewer: string | null
  reviewed_at: datetime | null
  status: unreviewed | approved | needs_update
```

## Requirements assessment contract

```yaml
schema_version: sourceright.requirements_assessment.v1
assessment_id: string
package_id: string
contract_id: string
assessed_at: datetime
findings:
  - requirement_id: string
    status: pass | fail | partial | not_applicable | unknown
    moscow: must | should | could | wont
    evidence: string
    recommendation: string
    implementable: boolean
    implementation_action: none | author_task | automated_dry_run | human_editor_task
summary:
  must_failures: integer
  should_failures: integer
  could_opportunities: integer
  ready_for_submission: boolean
```

## Reporting checklist contract

```yaml
schema_version: sourceright.reporting_checklist_contract.v1
contract_id: string
package_id: string
selection_rationale: string
source_checklists:
  - name: string
    version: string
    url: string
    retrieved_at: datetime
    authority: official | journal_policy | community | unknown
items:
  - id: string
    canonical_text: string
    source_items:
      - checklist_name: string
        item_id: string
        text: string
    category: title | abstract | introduction | methods | results | discussion | funding | ethics | data | code | ai | patient_public_involvement | equity | reproducibility
    moscow_default: must | should | could
    machine_checkable: boolean
    assessment_method: deterministic | agentic | human
```

## Agent workflow contract

```yaml
schema_version: sourceright.agent_workflow.v1
workflow_id: string
policy_profile: human_only | agent_assisted | agent_comparative | agent_screening_only | fully_automated_non_decision
allowed_tasks:
  - intake_check
  - requirement_sourcing
  - checklist_sourcing
  - metadata_extraction
  - reviewer_matching
  - technical_validation
  - reproducibility_rerun
  - review_brief
  - agent_review
  - review_synthesis
  - presentation_generation
  - post_publication_monitoring
forbidden_tasks:
  - final_accept_reject_without_human_editor
  - confidential_external_upload_without_permission
audit:
  log_inputs: true
  log_outputs: true
  log_model_versions: true
  log_tool_versions: true
  require_human_approval_for_write: true
```

## Peer review contribution contract

```yaml
schema_version: sourceright.peer_review_contribution.v1
review_id: string
package_id: string
reviewer:
  identity_type: human | agent | group | anonymous_public
  orcid: string | null
  model_or_tool: string | null
  verified_profile_ids: []
policy:
  visibility: private_editorial | public_anonymous | public_signed | public_agent_labeled
  confidentiality_level: private | public_preprint | public_after_decision
review_type: scope | methods | statistics | data | software | ethics | clinical | policy | reproducibility | general
content_uri: string
summary: string
competing_interests: string
credit:
  citable: boolean
  contribution_roles: []
```

## Reproducibility evidence contract

```yaml
schema_version: sourceright.reproducibility_evidence.v1
evidence_id: string
package_id: string
tier: full_rerun | partial_rerun | replication_feasible | conceptual_replication_feasible | not_reproducible_from_materials | not_applicable
inputs:
  data_available: boolean
  code_available: boolean
  environment_available: boolean
  permissions_sufficient: boolean
  compute_sufficient: boolean
results:
  tables_regenerated: []
  figures_regenerated: []
  tests_passed: []
  failures: []
limitations: []
recommendations: []
```

## Contribution ledger contract

The contribution ledger extends author credit to all measurable inputs into the publishing process.

Minimum contribution types:

- Conceptualization
- Data creation
- Data curation
- Software
- Workflow engineering
- Formal analysis
- Reproducibility verification
- Statistical review
- Peer review
- Editorial review
- Copyediting
- Translation
- Human verification of AI output
- Visualization
- Dissemination
- Platform plugin or workflow development
- Funding or commissioning
- Governance review

## Knowledge graph contract

The graph should preserve temporal and provenance-aware relationships:

- `asks_question`
- `uses_protocol`
- `uses_dataset`
- `uses_code`
- `produces_claim`
- `supports_claim`
- `challenges_claim`
- `supersedes_claim`
- `updates_interpretation`
- `has_review`
- `has_editorial_decision`
- `has_dissemination_output`
- `has_contribution`
- `has_reproducibility_evidence`
- `has_external_deposit`
- `has_status_label`

## Claim boundary

Contracts make evidence auditable. They do not make the evidence correct. Every contract must retain source provenance, uncertainty, and human review state so downstream systems do not mistake structured metadata for verified truth.
