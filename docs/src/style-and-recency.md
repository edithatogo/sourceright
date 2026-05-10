# Style And Recency

Style and recency checks are separate from provider verification. They express
journal, repository, legal, or workflow rules that may require review even when
a reference is bibliographically valid.

Initial policy concepts:

- reference order: appearance, alphabetical, or unspecified;
- DOI policy: required if available, optional, or not required;
- publication-age warnings for older references;
- guideline-age warnings for later guideline-specific checks;
- preprint and retraction policies as manifest/schema contracts.

Retraction, correction, superseded-guideline, and repository-deposit checks need
provider-backed evidence before they should affect reports. Until then, they
belong in policy documents and plugin manifests rather than silent automated
decisions.

Once provider-backed evidence is available, Sourceright can surface it as
editor-facing recency and integrity signals while keeping the canonical CSL
record unchanged.
