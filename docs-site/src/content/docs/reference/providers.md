---
title: Providers
description: Provider normalization, evidence, and confidence policy.
---

Academic verification should be built as an ordered provider roadmap. Each provider adds candidates, provenance, and confidence signals; provider data must never silently overwrite the canonical reference record.

Use the registry status matrix in [Plugin Registry](../../plugin-registry/#status-matrix)
when describing readiness. In practice, `core_normalizer` and `core_exporter`
are the only statuses that should be described as technical preview; the
`planned_*` and `planned` statuses remain roadmap-only.

## Implementation Order

1. Crossref: first source for DOI metadata and bibliographic matching. Use it to establish the provider contract, normalized candidate shape, confidence scoring inputs, and provenance fields.
2. DOI resolution: validate DOI reachability and final landing-page targets separately from metadata lookup. Resolution confirms identifier usability; it does not by itself prove bibliographic correctness.
3. DataCite: add coverage for datasets, software, preprints, and non-journal scholarly outputs that Crossref may not describe well.
4. OpenAlex: add broad graph enrichment, including work identifiers, venues, authorship, concepts, and citation context. Treat OpenAlex as enrichment unless its evidence independently supports a verification decision.
5. PubMed/NCBI: add biomedical lookup and PMID/PMCID enrichment for health and life-science references. Use NCBI identifiers as additional provenance rather than replacements for DOI or canonical citation fields.
6. ORCID: add author identity enrichment after work-level matching is reliable. ORCID should link candidate authors to persistent person identifiers, not create work matches on its own.

## Fixture and Mocking Expectations

Provider work must be testable without live network access. Each provider should define fixture-backed responses for successful lookup, no-match, ambiguous-match, rate-limit, outage, malformed response, and conflicting metadata cases. Fixtures should preserve enough raw provider payload to verify parsing and provenance, while tests assert against normalized candidate output.

Provider result diagnostics are deterministic. A fixture-backed provider result
can be classified as `no_match`, `ambiguous`, `malformed_response`, or
`outage`. These diagnostics describe provider evidence quality; they do not
modify canonical CSL.

The Rust core exposes those diagnostics as provider, kind, code, and message
fields. `no_match` and `ambiguous_match` come from normalized fixture evidence;
`malformed_response` and `outage` come from explicit provider error evidence.
The raw provider candidate remains sidecar evidence and never overwrites
canonical CSL.

HTTP clients should be mockable behind the provider boundary. Unit tests should use local fixtures or explicit mock responses; live provider checks, if added later, should be opt-in and excluded from the default test path.

Live adapters must expose timeout, cache, retry, and minimum-interval controls
before they are used in pilot workflows. The default policy is conservative:
20-second request timeout, 1000 ms minimum interval, two retries for transient
failures, and no cache directory unless `SOURCERIGHT_PROVIDER_CACHE_DIR` is set.
The cache is an evidence cache; cached payloads do not become canonical CSL and
must still carry provider/provenance metadata.

## Implemented Core

The Rust core currently includes the provider result model and fixture-friendly helpers for:

- Crossref work normalization from JSON payloads into sidecar `ProviderCandidate` records.
- DOI resolution evidence that records reachability, final target, or deterministic error metadata without changing canonical CSL fields.
- Provider error evidence for malformed responses and unavailable provider states.

The remaining providers in the roadmap should reuse this result shape and keep their default tests fixture-backed.

## Expansion Fixtures

The next provider families are tracked under `provider-fixtures/` and the plugin
registry. They are not live adapters yet:

- Crossmark or Retraction Watch-style status evidence.
- Unpaywall open-access location evidence.
- OpenCitations citation graph evidence.
- arXiv preprint identity evidence.
- Europe PMC biomedical repository evidence.
- Zenodo, OSF, Figshare, and Dataverse repository references.
- Dimensions, Scopus, and Web of Science as BYO-key/licensed-data plugins.

Each future adapter should add success, no-match, ambiguous-match, rate-limit,
outage, malformed-response, and conflicting-metadata fixtures before any live
network behavior is enabled.

## Live Adapter Policy

Live provider adapters are opt-in. The Rust core now exposes live smoke helpers
for Unpaywall, OpenCitations, arXiv, Europe PMC, repository records, and a
generic bring-your-own-key hook. They default to skipped unless both
`SOURCERIGHT_LIVE_PROVIDERS=1` and `SOURCERIGHT_LIVE_PROVIDER_SMOKE=1` are set.

Required or optional credentials are read from:

- `UNPAYWALL_EMAIL`
- `OPENCITATIONS_ACCESS_TOKEN`
- `EUROPE_PMC_EMAIL`
- `SOURCERIGHT_REPOSITORY_PMID`
- `SOURCERIGHT_BYO_KEY`

Runtime policy is read from:

- `SOURCERIGHT_PROVIDER_TIMEOUT_SECS`
- `SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS`
- `SOURCERIGHT_PROVIDER_MAX_RETRIES`
- `SOURCERIGHT_PROVIDER_CACHE_DIR`

The adapters write only provider evidence and do not touch canonical CSL JSON.
Default tests stay fixture-backed and the live smoke helpers report skipped
states with explicit reasons when credentials or opt-in flags are missing.

## Merge Rule

Provider results are candidates with provenance. They must not silently overwrite canonical references. Any proposed change to canonical fields must be represented as a sidecar candidate, conflict, or explicit merge decision with source attribution.

---

## Expanded Provider Catalogue

The table below documents every normaliser provider candidate tracked by Sourceright.
Each row includes access model, current status, owning track, evidence level, fixture
coverage, default-CI behaviour, and the overclaim guard that prevents unsupported
market-readiness claims.

| Provider | Domain | Access Model | Status | Track | Evidence Level | Fixtures | Default-CI | Overclaim Guard |
|---|---|---|---|---|---|---|---|---|
| Crossref | Scholarly / DOI | Public API (polite mailto) | `core_normalizer` | 48 | fixture-backed | yes (success, no-match, ambiguous, error) | opt-in live smoke | "Provider results are candidates with provenance—Crossref evidence never overwrites canonical CSL fields." |
| DataCite | Data / software / preprint | Public API | planned | 48 | contracted | not yet | opt-in live smoke | "DataCite is enrichment-only; dataset evidence is sidecar, not canonical." |
| OpenAlex | Scholarly graph | Public API | planned | 48 | contracted | not yet | opt-in live smoke | "OpenAlex provides graph enrichment; it does not independently verify bibliographic correctness." |
| PubMed/NCBI | Biomedical | Public API | `core_normalizer` (repository.pubmed) | 48 | fixture-backed | yes (PMID lookup, no-match) | opt-in live smoke | "PubMed identifiers are provenance, not replacements for DOI or canonical fields." |
| ORCID | Author identity | Public API | planned | 48 | contracted | not yet | opt-in live smoke | "ORCID links authors to persistent identifiers; it does not create work-level matches on its own." |
| Unpaywall | Open-access location | Public API (polite mailto) | `planned_public_api` | 48 | fixture-backed | planned | opt-in live smoke | "OA location evidence is sidecar; canonical availability is never assumed from Unpaywall alone." |
| OpenCitations | Citation graph | Public API (token recommended) | `planned_public_api` | 48 | fixture-backed | planned | opt-in live smoke | "Citation evidence is graph-sidecar; no SOTA citation-count claims are made from OpenCitations data." |
| arXiv | Preprint identity | Public API | `planned_public_api` | 48 | fixture-backed | planned | opt-in live smoke | "arXiv preprint evidence is version-provenance, not peer-review status." |
| Europe PMC | Biomedical repository | Public API | `planned_public_api` | 48 | fixture-backed | planned | opt-in live smoke | "Europe PMC evidence supplements PubMed/DOI; preprint/review status is labelled as provider-reported, not system-verified." |
| Dimensions | Licensed scholarly metadata | BYO-key (licensed) | `planned_byo_key` | 49 | contracted | not yet | opt-in, skip by default | "Licensed data plugin—no Dimensions claims without active BYO-key and explicit user consent." |
| Scopus | Licensed scholarly metadata | BYO-key (licensed) | `planned_byo_key` | 49 | contracted | not yet | opt-in, skip by default | "Licensed data plugin—no Scopus claims without active BYO-key and explicit user consent." |
| Web of Science | Licensed scholarly metadata | BYO-key (licensed) | `planned_byo_key` | 49 | contracted | not yet | opt-in, skip by default | "Licensed data plugin—no WoS claims without active BYO-key and explicit user consent." |
| RePEc | Economics working papers | Public API (RePEc) | deferred | 62 | assessment-only | none | excluded | "RePEc has no unified API; individual archive scraping is not supported. Assessed and deferred—see economics-decision-log." |
| SSRN | Preprint / working papers | Public API (Elsevier-owned) | deferred | 62 | assessment-only | none | excluded | "SSRN API access requires Elsevier agreement. Assessed and deferred—see economics-decision-log." |
| NBER | Economics working papers | Public API (NBER) | deferred | 62 | assessment-only | none | excluded | "NBER provides bulk data; no per-work API suitable for normaliser lookup. Assessed and deferred—see economics-decision-log." |
| EconLit | Economics bibliography | Licensed (EBSCO) | deferred | 62 | assessment-only | none | excluded | "EconLit is EBSCO-licensed; no public API. Assessed and deferred—see economics-decision-log." |
| IDEAS | Economics / RePEc portal | Public (RePEc aggregate) | deferred | 62 | assessment-only | none | excluded | "IDEAS mirrors RePEc; inherits the same no-unified-API limitation. Assessed and deferred—see economics-decision-log." |
| Zenodo | Repository / datasets | Public API | planned | 50 | contracted | not yet | opt-in live smoke | "Zenodo evidence is repository-sidecar; schema crosswalk normalises to CSL but never overwrites curated fields." |
| OSF | Repository / project | Public API | planned | 50 | contracted | not yet | opt-in live smoke | "OSF evidence is repository-sidecar; project vs. component granularity is documented and never assumed canonical." |
| Figshare | Repository / data | Public API | planned | 50 | contracted | not yet | opt-in live smoke | "Figshare evidence is repository-sidecar; DOI/URL linking follows provider-record shape." |
| Dataverse | Repository / data | Public API | planned | 50 | contracted | not yet | opt-in live smoke | "Dataverse evidence is repository-sidecar; version and collection provenance is recorded but never canonical." |
| Institutional repositories | Repository / thesis | API varies (OAI-PMH, REST) | deferred | 62 | assessment-only | none | excluded | "No single API contract; per-repository adapters are deferred pending community patterns and track 50 prioritisation." |
| Google Scholar | Search index | Search only (no public API) | **deferred — prohibited** | 62 | assessment-only | none | excluded permanently | **ADR 0005**: No scraping or unsupported automation. Google Scholar is documented as assessment-only / deferred. No live adapter, no fixtures, no CI path. |
| bioRxiv / medRxiv | Preprint biology / medicine | Public API | planned | 58 | contracted | not yet | opt-in live smoke | "bioRxiv/medRxiv preprint evidence is version-provenance; API returns preprints, not peer-reviewed status." |
| Clinical trial registries | Clinical trials | Public API (CT.gov, EU-CTR, etc.) | deferred | 62 | assessment-only | none | excluded | "Clinical trial registry normalisation is domain-specific beyond baseline CSL; deferred to future health-sciences track." |

### Economics Decision Log

Economics providers RePEc, SSRN, NBER, EconLit, and IDEAS were assessed during
track 62. Each was found unsuitable for the current normaliser provider API
contract (per-work DOI or identifier lookup with normalised JSON-LD response)
for the following shared reasons:

- **No unified REST API**: RePEc and IDEAS rely on decentralised archive
  aggregation; there is no single endpoint for per-work metadata resolution.
- **Licensing barriers**: SSRN requires an Elsevier API agreement; EconLit is
  EBSCO-licensed and offers no public API.
- **Bulk-only data**: NBER provides bulk CSV/Stata dumps but no per-work service
  suitable for on-demand normalisation.
- **Deferral policy**: These providers may become viable if the community
  standardises a RePEc JSON API, or if a dedicated economics normalisation track
  is created. For now they are recorded as `deferred — see economics-decision-log`.

### Google Scholar Decision Log (ADR 0005)

**Decision**: Google Scholar will never be implemented as a normaliser provider
in Sourceright.

**Rationale**:

- Google Scholar has no public API.
- Scraping Google Scholar violates its Terms of Service and is legally and
  ethically prohibited.
- Automated access through headless browsers or CAPTCHA-solving constitutes
  unsupported automation.
- Users seeking Google Scholar coverage should use OpenAlex (which indexes much
  of the same content) or individual publisher/provider lookups.

**Status**: `deferred — prohibited permanently`. No manifest, no fixtures, no
live adapter, no CI path. This entry exists solely to document the decision and
prevent future re-prioritisation without explicit governance approval.

### Grey Literature and Repository Notes

Zenodo, OSF, Figshare, and Dataverse are tracked as a single `planned_public_api`
manifest (`provider.repository-records`). Each has a public REST API and can be
normalised into the same `ProviderCandidate` evidence shape. Institutional
repository integration is deferred because of the diversity of platforms
(DSpace, EPrints, Digital Commons, Haplo, etc.). A future track may define a
shared OAI-PMH normaliser adapter.

### Biomedical Preprint Notes

bioRxiv and medRxiv (both Cold Spring Harbor Laboratory) expose a public API
that returns preprint metadata by DOI. These are planned under the repository
provider umbrella. Clinical trial registries (ClinicalTrials.gov, EU-CTR,
ISRCTN, etc.) use non-standard identifier schemes and structured-data formats
that are outside the current normaliser contract; they are deferred to a
future health-sciences track.
