# Public API Provider Adapters Spec

Implement or explicitly defer the public API provider adapters: Unpaywall,
OpenCitations, arXiv, Europe PMC, and adjacent public metadata sources.

Each provider needs its own fixture-backed contract because matching keys,
rate-limit behavior, response shape, and confidence semantics differ. Provider
results write sidecar evidence only and must not mutate canonical CSL.

Parallel lanes: one subagent per provider. Shared provider traits, cache policy,
and docs are edited by the lead after lane results are known.
