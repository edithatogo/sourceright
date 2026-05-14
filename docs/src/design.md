# Design

This design document explains the architecture behind the feature contract. The
Conductor-owned planning authority is `conductor/design.md`; this page mirrors
the same architecture for users and contributors. It is intentionally
contract-first: public claims, tests, and Conductor tracks should stay
consistent with these boundaries.

## Scope

Sourceright is a Rust-first reference triage and verification system. The core
accepts text or document-derived references, builds canonical CSL, records
evidence in a sidecar, routes ambiguity to review, and exposes reports, exports,
CLI commands, and MCP surfaces.

## Data Boundaries

```mermaid
flowchart LR
    Input[Document or text input] --> Intake[Reference and citation intake]
    Intake --> CSL[references.csl.json]
    CSL --> Verify[Provider and policy verification]
    Verify --> Sidecar[references.verification.json]
    CSL --> Review[review-queue.jsonl]
    Sidecar --> Review
    CSL --> Report[Reports and exports]
    Sidecar --> Report
```

Canonical CSL stays clean. Provider records, provenance spans, confidence, and
conflicts live in the sidecar. Review queues and reports are derived artifacts.

```mermaid
flowchart TB
    CSL[Canonical CSL]:::canonical
    Sidecar[Evidence sidecar]:::evidence
    Queue[Review queue]:::derived
    Exports[Exports and reports]:::derived
    Providers[External providers]:::external

    Providers --> Sidecar
    CSL --> Queue
    Sidecar --> Queue
    CSL --> Exports
    Sidecar --> Exports

    classDef canonical fill:#e8f5f1,stroke:#16775c,color:#10231d;
    classDef evidence fill:#f2f0ff,stroke:#6750a4,color:#1f1833;
    classDef derived fill:#fff4df,stroke:#a15c00,color:#2d1a00;
    classDef external fill:#eef2f7,stroke:#506070,color:#101820;
```

## CLI And MCP Surfaces

```mermaid
flowchart LR
    Core[Rust core] --> CLI[sourceright CLI]
    Core --> MCP[stdio MCP server]
    CLI --> Commands[init, validate-csl, report, export, bench, citation-sync]
    MCP --> Tools[tools]
    MCP --> Resources[resources]
    MCP --> Prompts[prompts]
    Tools --> DryRun[write tools default to dry-run]
```

CLI and MCP are two adapters over the same Rust core. Public JSON outputs are
treated as contracts, and write-capable MCP operations must remain dry-run first
with explicit apply semantics.

## Providers And Plugins

```mermaid
flowchart TB
    Registry[plugins/registry.toml] --> Manifests[Plugin manifests]
    Manifests --> Validate[Manifest validation]
    Validate --> Trust[Trust and capability policy]
    Trust --> Discovery[CLI and MCP discovery report]
    Providers[Provider adapters] --> Sidecar[Sidecar-only evidence]
```

Plugins declare capabilities and network/auth requirements. Provider adapters
may enrich evidence, but they do not overwrite canonical CSL.

## Journal Integration

```mermaid
sequenceDiagram
    participant OJS as OJS or journal system
    participant Adapter as Adapter contract
    participant CLI as Sourceright CLI/MCP
    participant Editor as Editorial reviewer
    OJS->>Adapter: Submission metadata and manuscript text
    Adapter->>CLI: Run intake and screening
    CLI-->>Adapter: JSON report and review queue
    Adapter-->>Editor: Editor-facing triage summary
```

OJS is the first public journal target because it is open source and has a
plugin ecosystem. Other platforms should use the same screening contract before
any platform-specific writeback.

## Citation Manager Sync

```mermaid
sequenceDiagram
    participant User
    participant Sync as citation-sync
    participant Workspace as Sourceright workspace
    participant Manager as Zotero or fixture target
    User->>Sync: preview
    Sync->>Workspace: compare CSL and sidecar evidence
    Sync-->>User: proposed changes and conflicts
    User->>Sync: apply
    Sync->>Manager: explicit write
    Sync->>Workspace: audit log
```

Citation-manager sync defaults to preview. Apply operations must be explicit,
audited, and conflict-aware.

## Release And Registry Flow

```mermaid
flowchart LR
    Tag[v*.*.* tag] --> Checks[CI, security, coverage, release dry run]
    Checks --> Crate[crates.io package]
    Checks --> Release[GitHub Release artifacts]
    Checks --> OCI[GHCR MCP image]
    OCI --> MCPRegistry[Official MCP Registry]
    Release --> Docs[docs.rs and public docs]
    Metadata[server.json and glama.json] --> MCPRegistry
```

Publication evidence must separate accepted listings from prepared or submitted
metadata. Future package-manager channels should be added only when they have a
maintainable manifest and validation path.
