# Provider Strategy

Academic verification should be built as an ordered provider roadmap. Each provider adds candidates, provenance, and confidence signals; provider data must never silently overwrite the canonical reference record.

Use the registry status matrix in [Plugin Registry](plugin-registry#status-matrix)
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

The adapters write only provider evidence and do not touch canonical CSL JSON.
Default tests stay fixture-backed and the live smoke helpers report skipped
states with explicit reasons when credentials or opt-in flags are missing.

## Merge Rule

Provider results are candidates with provenance. They must not silently overwrite canonical references. Any proposed change to canonical fields must be represented as a sidecar candidate, conflict, or explicit merge decision with source attribution.
