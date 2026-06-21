# Track 94 - Reproducibility, Replication, and Executable Evidence

## Goal

Assess whether a submission can be computationally reproduced, independently replicated, partially verified, or not reproduced from available materials.

## User outcome

Editors, reviewers, authors, and readers receive a clear reproducibility tier with evidence, limitations, and recommendations.

## Scope

- Detect available data, code, environment files, workflows, containers, notebooks, compute needs, licenses, and permissions.
- Rerun analyses where all required materials are available and permissions allow.
- Regenerate declared tables, figures, statistics, models, and outputs where feasible.
- Classify evidence as full rerun, partial rerun, replication feasible, conceptual replication feasible, not reproducible from supplied materials, or not applicable.
- Record blocked reproduction reasons, including missing data, missing code, missing environment, insufficient permissions, non-determinism, compute constraints, or privacy limits.
- Support GitHub, GitLab, notebooks, Colab, Binder-like environments, Code Ocean-like capsules, WorkflowHub, RO-Crate, Zenodo, OSF, Dataverse, and Software Heritage linkage.

## Out of scope

- Treating rerun success as independent validation of scientific truth.
- Requiring sensitive health data to be made open.
- Running unsafe code without sandboxing.

## Completion criteria

- Reproducibility evidence contract is documented.
- Reproduction tiers are documented.
- Rerun and replication-feasibility workflows are separated.
- Safety and sandbox boundaries are explicit.
