# Submission Contracts

Sourceright uses a shared submission contract for every external registry,
marketplace, plugin host, and upstream repository submission surface. This
public mirror stays aligned with `conductor/submission-contracts.md` and
`conductor/submission-requirements.json`.

External submissions require explicit approval. A local build, draft package,
or CI success is not enough to submit anything externally.

## Surfaces

| Surface | Contract shape | Evidence gate |
| --- | --- | --- |
| Official MCP Registry | Registry listing metadata and release artifact binding | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Smithery | MCPB or Streamable HTTP package with listing metadata | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Glama | Directory listing metadata and install proof | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Zotero | Plugin package and library-scoped smoke proof | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| EndNote | Reference-checking handoff package or export contract | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| OJS/PKP | Installable plugin package with permissions and smoke proof | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| arXiv submit-ce | Upstream issue or pull request URL plus package evidence | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| arXiv submission-core | Upstream issue or pull request URL plus package evidence | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Claude Cowork | Host-package contract or explicit deferral | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Codex app | Host-package contract or explicit deferral | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| GitHub Copilot | Host-package contract or explicit deferral | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Gemini CLI extensions | Host-package contract or explicit deferral | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| Qwen CLI extensions | Host-package contract or explicit deferral | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |
| VS Code / Open VSX | VSIX or marketplace package and install proof | Requirements searched -> Contracted -> Hardened local package -> Submission-ready -> Submitted -> Publicly accepted |

## Approval Rule

External submissions require explicit approval before any issue, pull request,
listing, or registry submission is created.

## Gate Ladder

1. Requirements searched
1. Contracted
1. Hardened local package
1. Submission-ready
1. Submitted
1. Publicly accepted

## Boundary

Sourceright does not submit or mutate external systems automatically. Any
external issue or pull request URL must be paired with the exact surface and
the approval record that authorized it.

## arXiv Boundary

- arXiv/submit-ce
- arXiv/arxiv-submission-core
- maturity, stability, and testing gates
- Sourceright does not submit papers or mutate arXiv state
