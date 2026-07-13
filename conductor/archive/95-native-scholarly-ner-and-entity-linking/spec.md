# Native Scholarly NER and Entity Linking Spec

## Goal

Provide an optional, independent, source-grounded entity schema and
deterministic lexicon baseline without coupling reference extraction to the
legacy GROBID-NER Java runtime.

## User outcome

Consumers can inspect entity mentions, class mappings, confidence, and
separate linking candidates while preserving source text and keeping canonical
CSL untouched.

## Scope

- Versioned extensible entity vocabulary and mapping relations.
- Deterministic lexicon recognizer with source spans and domain-pack metadata.
- Separate entity-link evidence with registry/version/query/method/score.
- Self-authored fixture and model/data ledger.
- GROBID-NER audit recording Java/API/version, 27-class scope, bibliography
  exclusion, mixed data availability, and no mandatory runtime dependency.

## Out of scope

- Treating GROBID-NER as a citation parser or required runtime.
- General, biomedical, or legal performance claims from one fixture.
- Bundling unclear datasets, model weights, or external registries.
- Silent ontology assignment or mutation of mention text/class/CSL.

## Data contracts

Mentions retain source spans, original labels, mapped class, mapping relation,
confidence, and optional link candidates. Link evidence is separate from NER
output. Mapping relations expose lossy conversions rather than hiding them.

## Claim boundary

This track proves a self-authored deterministic lexicon baseline and schema.
NER claims require named domain/language/label cohorts; no GROBID-NER or
biomedical/legal accuracy is claimed.

## Evidence level target

Fixture-backed native schema and baseline with auditable provenance, no network,
no model downloads, and citation extraction unchanged when NER is disabled.

## Parallelization plan

- Lane A: GROBID-NER provenance and data ledger.
- Lane B: vocabulary/mapping contract.
- Lane C: deterministic baseline and fixture metrics.
- Lane D: learned model/linking/domain packs only after independent evaluation.
