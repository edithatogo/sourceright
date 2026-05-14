# Contract Evidence And Overclaim Gates Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Conductor requirements | `conductor/requirements.md` contains MoSCoW, track ownership, evidence levels, and overclaim guards. |
| Conductor design | `conductor/design.md` contains Mermaid diagrams for boundaries, dependencies, subagents, security, external proof, plugins, and overclaim gating. |
| Public mirror parity | Public docs point to or mirror the Conductor contract without drifting. |
| Forbidden claims | Tests fail if blocked production, final-verifier, SOTA, AI-detector, or legal-compliance claims appear unsupported. |
| Status promotion | Track completion requires tests, docs, evidence, and review notes. |
| Review loop | `$conductor-review` runs and fixes are applied before progression. |
