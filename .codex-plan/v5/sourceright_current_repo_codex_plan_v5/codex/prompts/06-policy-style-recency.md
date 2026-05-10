Proceed with Slice 5: policy, style, and recency planning plus minimal compatible implementation.

First inspect whether style, policy, recency, journal, or screening modules already exist. Then add the smallest compatible design.

Potential additive targets:

```text
docs/policy-engine.md
docs/style-and-recency.md
examples/policies/journal-vancouver.yaml
examples/policies/preprint.yaml
examples/policies/repository-deposit.yaml
examples/policies/legal-filing.yaml
schemas/sourceright.policy.schema.json if not already added
```

If a Rust module fits cleanly, keep it minimal and deterministic.

Suggested policy concepts:
- citation family detection/checking
- reference order policy
- DOI-if-available policy
- preprint policy
- retraction/correction review policy
- publication-age warning policy
- superseded guideline/standard warning policy

Do not add semantic relevance, claim-truth scoring, or LLM-based claim verification yet.
Do not change existing journal-screen behavior unless tests are updated.
