# MCP Registry Scorecards — refreshed 2026-07-13

These scorecards separate repository-controlled readiness from live scores
assigned by each registry. `100/100` is recorded only when the registry itself
reports it or every explicitly defined criterion has direct evidence.

## Official MCP Registry

Live query:
`https://registry.modelcontextprotocol.io/v0.1/servers?search=sourceright`

| Criterion | Points | Evidence |
| --- | ---: | --- |
| Active latest listing | 25/25 | `0.1.20` is active and `isLatest: true`. |
| Version-aligned immutable package | 25/25 | OCI identifier is `ghcr.io/edithatogo/sourceright-mcp:0.1.20`. |
| Repository and schema metadata | 25/25 | Repository URL and 2025-12-11 server schema are present. |
| Reproducible checked-in contract | 25/25 | `server.json` and policy tests bind metadata to Cargo version. |

**Evidence-backed readiness: 100/100.** This is a registry metadata and package
binding score, not proof that every downstream MCP client can install or call
the server.

## Glama

Live API:
`https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright`

| Criterion | Points | Evidence |
| --- | ---: | --- |
| Public listing and repository binding | 25/25 | Listing id `c7qsbvekc1`; repository URL is correct. |
| Valid checked-in Glama metadata | 25/25 | `glama.json` validates against the live schema. |
| Discoverable callable tools | 0/25 | Live API currently returns `tools: []`. |
| SPDX license discovery | 0/25 | Live API currently returns `spdxLicense: null`. |

**Live Glama profile completion: 33/100.** The Glama score page reports no
Glama release, no coherence/TDQS score, no detected usage, and an unrecognised
license. It does confirm valid `glama.json`, author verification, passing CI,
and no critical/high vulnerability or code-scanning findings.

The top-level `LICENSE` is now canonical MIT text, and GitHub currently reports
`license.spdx_id=MIT` for the public repository. `LICENSE-MIT`,
`LICENSE-APACHE`, and Cargo's `MIT OR Apache-2.0` expression preserve the
existing dual-license grant. Glama's live API still reports `spdxLicense: null`,
so the remaining gap is a Glama-side rescan/release rather than an unpublished
repository license.

## Smithery

| Criterion | Points | Evidence |
| --- | ---: | --- |
| Server card generated from runtime | 25/25 | Checked-in card matches live tools/resources/prompts. |
| Release-derived local MCPB path | 25/25 | Builder and manifest template are policy-tested. |
| Public accepted listing | 25/25 | Live listing returns HTTP 200 and records publication on 2026-06-09. |
| Green Smithery release scan/install | 0/25 | Current public API exposes the MCPB and tools, but `remote=false`, `deploymentUrl=null`, and the unauthenticated releases endpoint returns HTTP 401. |

**Live Smithery quality score: 81/100.** Smithery describes this as a quality
score based on MCP best practices and reliability. The listing is public but
still identifies the server as local and has no successful current deployment
evidence. Raising the remaining 19 points requires a supported MCPB release or
a real Streamable HTTP endpoint; a static server card is discovery metadata,
not an HTTP MCP transport.

## Required next actions

1. In Glama, claim the server if necessary, configure the Dockerfile build with
   `CMD ["mcp"]`, deploy, pass the MCP handshake test, and publish a Glama
   release. Re-run the API probe and record the resulting tool/license/quality
   fields.
2. Add a Smithery API key as a protected secret, publish the generated Linux
   MCPB through the supported stdio release endpoint, and retain the successful
   scan/install release record.
3. Re-run the live probes and update this scorecard before claiming 100/100.

Repository-side publication evidence: `aaac662` published the canonical
license and MCP sources; `f8c02cf` aligned the CiteWeft source policy and docs;
`1b844f4` restored the dependency-policy exception. The latest public Smithery
listing exposes the expected tools, resources, and prompts, but that is not
equivalent to a successful deployment scan.
