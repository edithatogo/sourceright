# Policy Engine

Sourceright policy checks are deterministic workflow checks over existing CSL
and sidecar data. They do not perform semantic relevance scoring, claim-truth
scoring, or LLM-based verification.

The initial Rust policy module supports:

- DOI-if-available warnings for reference types that commonly carry DOI
  metadata;
- publication-age warnings from CSL `issued.date-parts`;
- reference-order checks for alphabetical-by-title policies;
- URL/archive integrity classes derived from canonical CSL URL fields and
  sidecar provider evidence, including invalid, redirected, archived, missing
  archive, broken, offline, and unchecked URL states;
- unsupported policy schema diagnostics.

URL/archive checks are sidecar-evidence classifiers. They can flag DOI landing
pages, invalid archive URLs, archive snapshots that point at live URLs, missing
archive evidence for non-DOI landing pages, and explicit provider status values
such as broken, offline, or unchecked. They do not fetch URLs in the default
path.

Policy inputs use `sourceright.policy.v1`. Policy outputs use
`sourceright.policy_report.v1` and contain stable issue records with severity,
optional reference id, code, and message.

Journal screening remains separate. Later work can feed policy issues into
journal reports once the policy CLI/API contract is stable.
