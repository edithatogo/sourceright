# Open Manuscript, Preprint and Review Platform Coverage

## Summary answer

Sourceright does not yet cover all open manuscript submission and management platforms, including preprint platforms. The current repo has strong coverage for OJS strategy, Janeway reconnaissance and hardening tracks, proprietary platform matrix planning, and arXiv submission-platform adapter work. It does not yet have comprehensive coverage for Kotahi, PubPub, OpenReview, HotCRP, Open Preprint Systems, Episciences, PREreview, Review Commons, OSF Preprints, Zenodo or InvenioRDM deposit flows, bioRxiv and medRxiv service integration, PubPeer, SciPost, or Qeios-like publishing and review systems.

## Coverage levels

| Level | Meaning |
| --- | --- |
| `implemented_or_in_progress` | Repo contains implementation, package, or explicit active track. |
| `contracted` | Repo should define an adapter contract and fixtures before claiming support. |
| `reconnaissance_required` | Official docs, APIs, licences, and integration surfaces need mapping. |
| `service_only` | The platform may be public but not necessarily open source or installable. |
| `out_of_scope_for_now` | Important to monitor but not a near-term implementation target. |

## Platform matrix

| Platform | Category | Openness | Current Sourceright status | Recommendation |
| --- | --- | --- | --- | --- |
| Open Journal Systems | Journal management and publishing | Open source, PKP ecosystem | Implemented or in progress through OJS tracks and plugin skeleton | Keep as first-class public journal integration target. |
| Janeway | Journal management and publishing | Open source, OLH/Birkbeck ecosystem | In progress through Tracks 83 to 85 | Continue reconnaissance, package hardening, and disposable smoke evidence. |
| Kotahi | Publishing workflow platform | Open source Coko ecosystem | Not covered | Add reconnaissance and adapter contract track. |
| PubPub | Community publishing platform | Open publishing platform, source and APIs require current review | Not covered | Add reconnaissance for machine-readable submission, review, and publication APIs. |
| OpenReview | Open peer review and conference/journal workflow | Public platform with APIs, not a standard journal CMS | Not covered | Add adapter for open reviews, rebuttals, public comments, and review metadata. |
| HotCRP | Conference submission and review management | Open source | Not covered | Add conference and review-management adapter reconnaissance. |
| OpenConf Community | Submission and peer-review management | Free community edition, proprietary/commercial boundary | Not covered | Treat as service or proprietary-style adapter target, not open-source core target. |
| PKP Open Preprint Systems | Preprint server software | PKP open source ecosystem | Not covered | Add preprint-platform track in parallel with OJS. |
| arXiv submit-ce | Preprint submission platform | Open source upstream project | Covered in arXiv tracks and PR work | Keep separate from journal-platform claims and bind to upstream evidence. |
| arXiv submission-core | Legacy preprint submission platform | Open source but legacy/inactive per existing track notes | Covered as legacy adapter track | Maintain migration-safe, no-overclaim posture. |
| bioRxiv and medRxiv | Preprint services | Public services, not installable open-source platforms | Not covered as service integration | Add service integration contract for deposit links, version matching, status labels, and review linkage. |
| OSF Preprints | Preprint service and OSF ecosystem | Public service and open science platform | Not covered | Add deposit and metadata adapter reconnaissance. |
| Zenodo | Repository and DOI deposit | Open infrastructure, Invenio-based | Not covered in this programme | Add repository deposit and article package archival contract. |
| InvenioRDM | Repository platform | Open source | Not covered | Add repository and knowledge-package deposit target. |
| DSpace | Repository platform | Open source | Not covered | Add repository deposit adapter candidate. |
| EPrints | Repository platform | Open source | Not covered | Add repository deposit adapter candidate. |
| Episciences | Overlay journal platform | Open source overlay publishing platform | Not covered | Add overlay journal and repository-first review adapter target. |
| PREreview | Preprint review platform | Public preprint review community | Not covered | Add public pre-review ingestion and contribution-credit adapter. |
| Review Commons | Journal-independent preprint review | Public review service | Not covered | Add review-transfer and reviewed-preprint metadata adapter. |
| PubPeer | Post-publication review | Public discussion platform | Not covered | Add monitoring-only integration with status and safety boundaries. |
| SciPost | Open review publishing platform | Public platform, review model target | Not covered | Add model and metadata reconnaissance. |
| Qeios | Preprint and open peer-review platform | Public service | Not covered | Monitor as service-only integration candidate. |
| F1000Research and Open Research platforms | Post-publication peer-review publishing model | Public platforms, not general open-source CMS | Not covered | Use as model reference, not near-term platform adapter. |
| ScholarOne | Journal submission system | Proprietary | Matrix track in progress | Keep as enterprise adapter contract only. |
| Editorial Manager | Journal submission system | Proprietary | Matrix track in progress | Keep as enterprise adapter contract only. |
| eJournalPress | Journal submission system | Proprietary | Matrix track in progress | Keep as enterprise adapter contract only. |
| Manuscript Manager | Journal submission system | Proprietary | Matrix track in progress | Keep as enterprise adapter contract only. |

## Missing coverage to add

1. Open preprint platform lane: PKP Open Preprint Systems, arXiv, OSF Preprints, Zenodo/InvenioRDM deposit, bioRxiv and medRxiv service linkage.
2. Open review lane: OpenReview, PREreview, Review Commons, PubPeer monitoring, SciPost, Qeios-like systems.
3. Open publishing platform lane: Kotahi, PubPub, Episciences, Janeway, OJS.
4. Conference and review-management lane: HotCRP and OpenReview, with OpenConf treated cautiously because of its proprietary boundary.
5. Repository-first lane: Zenodo, InvenioRDM, DSpace, EPrints, Dataverse, Figshare, OSF, and institutional repositories.

## Platform claim boundary

A platform is not supported until the repo has all of the following:

- Official-source requirements reconnaissance.
- License and deployment classification.
- Adapter contract.
- Synthetic fixture.
- Privacy and credential boundary.
- Smoke or dry-run proof where possible.
- Documentation that distinguishes implemented support from planned, service-only, or research-only support.

## External-source notes captured during programme design

- Janeway describes itself as open source and open access and lists customizable submissions, open or anonymous review, copyediting, JATS XML, metadata, file version control, roles, archiving, discovery, GDPR, security, and privacy as key features.
- PKP's software ecosystem includes Open Journal Systems, Open Preprint Systems, Open Monograph Press, and archived conference and harvester tools.
- Open peer-review platforms and preprint review services include OpenReview, PREreview, Review Commons, bioRxiv/medRxiv review links, and post-publication review surfaces.
- Emerging AI-era platform proposals such as AiraXiv point toward MCP-accessible, AI-augmented, preprint-first publishing infrastructures and should be monitored as design input rather than treated as production evidence.
