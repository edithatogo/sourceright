# Future Scholarly Communication Design

## Architecture principle

The platform should separate the knowledge layer from presentation layers. The knowledge layer is a versioned, machine-readable research project graph. Manuscripts, preprints, blog posts, slide decks, audio, video, monographs, peer-review views, editor views, reader chats, and indexing feeds are generated or curated views over that graph.

## Layered architecture

```mermaid
flowchart TB
    A[Author or contributor inputs] --> B[Research package intake]
    B --> C[Machine-readable knowledge layer]
    C --> D[Validation and verification workflows]
    C --> E[Presentation generators]
    C --> F[Queryable public APIs]
    D --> G[Editorial and peer-review workflows]
    D --> H[Reproducibility and integrity evidence]
    E --> I[Preprint]
    E --> J[Blog post]
    E --> K[Slides and visual summaries]
    E --> L[Audio, video, short-form and long-form views]
    E --> M[Reader chat and custom summaries]
    F --> N[Indexers, funders, registries, evidence synthesis]
    G --> O[Decision, curation and status labels]
    H --> O
    O --> C
```

## Submission package architecture

```mermaid
flowchart LR
    P[Research package] --> M[Manuscript object]
    P --> R[Reference object]
    P --> D[Data object]
    P --> C[Code and workflow object]
    P --> E[Ethics and governance object]
    P --> Q[Question and claim graph]
    P --> G[Reporting checklist object]
    P --> V[Review and editorial object]
    P --> T[Contribution and credit object]
    P --> X[External dissemination object]

    M --> XML[JATS XML]
    M --> MD[Markdown or MyST]
    M --> JSON[JSON or YAML]
    R --> CSL[CSL JSON]
    D --> RO[RO-Crate]
    C --> ENV[Container or executable environment]
    V --> PR[Peer review metadata]
    T --> CR[CRediT plus extended roles]
```

## Agent-first workflow model

Agents operate on contracts, not on hidden prompts or unstructured manuscript uploads. Each agent must declare its input contract, output contract, tool version, model version where relevant, policy mode, confidence, limitations, and human approval requirement.

```mermaid
sequenceDiagram
    participant Author
    participant IntakeAgent as Intake agent
    participant RequirementsAgent as Requirements agent
    participant ChecklistAgent as Checklist agent
    participant ReproAgent as Reproducibility agent
    participant ReviewAgent as Review agent
    participant Editor
    participant PublicRecord as Public record

    Author->>IntakeAgent: Submit research package
    IntakeAgent->>RequirementsAgent: Extract journal and article target
    RequirementsAgent-->>Editor: Journal-article contract and gaps
    IntakeAgent->>ChecklistAgent: Study design and methods metadata
    ChecklistAgent-->>Editor: Checklist contract and assessment
    IntakeAgent->>ReproAgent: Code, data, environment, permissions
    ReproAgent-->>Editor: Reproducibility tier and evidence
    IntakeAgent->>ReviewAgent: Policy-constrained review brief
    ReviewAgent-->>Editor: Agent review or briefing pack
    Editor-->>PublicRecord: Approve status, preprint, review policy, and outputs
    PublicRecord-->>Author: Auditable decision and next steps
```

## Configurable peer-review modes

```mermaid
flowchart TD
    S[Submission verified for configured minimum checks] --> P[Preprint or private review state]
    P --> A{Peer-review policy}
    A --> H[Human-only review]
    A --> HA[Human review with agent briefing]
    A --> PAR[Parallel human and agent review]
    A --> COM[Community pre-publication review]
    A --> AG[Agent-only non-decision screening]
    H --> E[Editor synthesis]
    HA --> E
    PAR --> E
    COM --> E
    AG --> E
    E --> D{Decision or curation status}
    D --> R1[Reviewed preprint]
    D --> R2[Accepted version]
    D --> R3[Revision requested]
    D --> R4[Not suitable for this venue]
    D --> R5[Living update pending]
```

## Journal requirement workflow

```mermaid
flowchart LR
    A[Target journal and article type] --> B[Source official requirements]
    B --> C[Create journal-article contract]
    C --> D[Human approve contract]
    D --> E[Assess submission]
    E --> F[MoSCoW findings]
    F --> G[Dry-run implementation plan]
    G --> H{Author or editor approves?}
    H -->|Yes| I[Apply permitted changes]
    H -->|No| J[Report only]
```

## Reporting checklist workflow

```mermaid
flowchart LR
    A[Submission metadata and methods] --> B[Detect candidate checklists]
    B --> C[Source authoritative checklist items]
    C --> D[Combine and deduplicate]
    D --> E[Create checklist contract]
    E --> F[Assess manuscript]
    F --> G[MoSCoW gap report]
    G --> H[Implementation plan]
    H --> I{Approve writeback?}
    I -->|Yes| J[Apply changes or generate author tasks]
    I -->|No| K[Record assessment only]
```

## Knowledge graph evolution

The graph should support changes in question, data, analysis, interpretation, evidence status, and dissemination. A contradiction between a preprint claim and a later interpretation should be represented as a temporal state change with provenance, not as an unexplained inconsistency.

```mermaid
flowchart TB
    Q1[Initial question] --> P1[Protocol]
    P1 --> D1[Dataset]
    D1 --> A1[Analysis v1]
    A1 --> C1[Claim v1]
    C1 --> PP[Preprint]
    PP --> R[Public and invited review]
    R --> A2[Analysis v2]
    A2 --> C2[Claim v2]
    C2 --> J[Reviewed article]
    J --> U[Living update or correction]
    U --> C3[Claim v3]
    C1 -. superseded by .-> C2
    C2 -. updated by .-> C3
```

## Reader-chosen presentation layer

```mermaid
flowchart LR
    K[Knowledge package] --> RAG[RAG and retrieval policy]
    U[Reader profile: role, interest, positionality, language] --> RAG
    RAG --> S1[Clinical summary]
    RAG --> S2[Policy brief]
    RAG --> S3[Methods critique]
    RAG --> S4[Patient or community explanation]
    RAG --> S5[Technical reproducibility report]
    RAG --> S6[Audio or video script]
    RAG --> S7[Slide deck]
    RAG --> S8[Long-form article or monograph]
    RAG --> S9[Conversational chat]
```

## Governance design

- Every agentic action is logged with inputs, outputs, model or tool version, policy profile, confidence, and human approval state.
- Every write-capable workflow supports dry-run first and explicit apply.
- Every public status label is derived from explicit event records.
- Every contribution is attributed and typed, including peer review, editorial work, translation, human verification, reproducibility, code review, and data curation.
- Every external integration remains evidence-gated before support claims are made.
