# Sourceright Product Context

Sourceright is reference verification infrastructure for documents, agents, and research workflows. Its first job is to extract references from documents or text, canonicalise them as CSL JSON, verify and enrich them against reliable citation providers, then produce clean downstream reference files for tools such as EndNote, Zotero, and BibLaTeX.

## Product Intent

- Build a strong reference verification core, not a bibliography formatter.
- Make CSL JSON the canonical reference format while storing verification provenance in a sidecar file.
- Move each reference through extraction, standardisation, cleaning, automated verification, conflict resolution, and manual review.
- Produce downstream XML, ENW, RIS, BibLaTeX, and YAML exports from verified references.
- Use agents and subagents for manual verification queues after automated checks have exhausted deterministic options.
- Start with academic references and citations, then add legal citation support as a distinct future model and provider strategy.
- Roadmap the broader claim/source/provenance universe without making claim verification a v1 blocker.

## Initial Users

- Researchers and writers who need cleaner reference files than ad hoc reference-manager imports can produce.
- Academic authors working with mixed-quality reference lists, DOCX manuscripts, Markdown drafts, PDFs, and pasted bibliography text.
- Operators using AI agents to audit and repair citation-heavy documents.
- Coding-agent users who need CLI/MCP reference verification inside repo workflows.
- Future legal users working with cases, legislation, neutral citations, pinpoints, courts, jurisdictions, and public legal citation APIs.

## Core Workflow

1. Ingest documents or pasted text.
2. Extract reference-list entries and in-text citations.
3. Create or update canonical `references.csl.json`.
4. Standardise, clean, deduplicate, and validate records.
5. Query academic providers such as Crossref, DOI resolution, DataCite, OpenAlex, PubMed/NCBI, and ORCID where useful.
6. Record provenance, confidence, provider matches, conflicts, and manual-review status in a verification sidecar.
7. Resolve safe provider/canonical conflicts without silent overwrites.
8. Reconcile in-text citations against the reference list.
9. Move uncertain records to an agent/manual review queue.
10. Generate reference integrity and journal screening reports.
11. Export verified references to XML, ENW, RIS, BibLaTeX, and YAML.

## Success Criteria

- Sourceright can be installed and run as a local CLI.
- Sourceright can expose the same reference workflow over MCP.
- A document can be converted into CSL JSON plus verification metadata and downstream export files.
- Automated verification is deterministic, auditable, and safe to hand to agents for manual review.
- Journals can consume the same local workflow as a citation-integrity screening contract without asserting that errors are AI-generated.
- Legal citation and broader claim/source/provenance work are explicit roadmap tracks with clear boundaries.
- The legacy `humanizer-next` workflow is audited before behaviour is ported, replaced, or discarded.
