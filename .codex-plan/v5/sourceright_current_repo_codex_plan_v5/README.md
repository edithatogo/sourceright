# Sourceright current-repo Codex plan v5

This ZIP is **not a bulk overlay**. It is a current-repo-aware Codex instruction pack for the live `edithatogo/sourceright` repository.

The repository has advanced beyond the earlier overlay plan. Use this pack to tell Codex to audit the current repo first, then add only missing pieces in small, additive slices.

## What this pack is for

- Give Codex a safe `AGENTS.md` template.
- Provide slice-by-slice prompts.
- Provide optional reference scaffolds for schemas, plugin manifests, demos, benchmarks, policies, citation-manager profiles, and MCP contracts.
- Avoid destabilising the existing single-crate Rust implementation.

## What this pack is not

- It is not a replacement repo.
- It is not intended to be copied wholesale into the repo.
- It should not trigger a workspace split.
- It should not overwrite existing Rust modules.

## Fast local use

```bash
git clone https://github.com/edithatogo/sourceright.git
cd sourceright
git checkout -b current-repo-sota-plan

mkdir -p ../sourceright-plan-v5
unzip ~/Downloads/sourceright_current_repo_codex_plan_v5.zip -d ../sourceright-plan-v5

cp ../sourceright-plan-v5/sourceright_current_repo_codex_plan_v5/AGENTS.md.template ./AGENTS.md
codex
```

Then paste the first prompt from:

```text
../sourceright-plan-v5/sourceright_current_repo_codex_plan_v5/codex/prompts/00-inspect-current-repo.md
```

## Short instruction to Codex

> The old overlay is stale. Use the current repo as source of truth. Do not bulk-apply the overlay. Audit first, then add missing pieces in small additive slices: schemas, plugin manifests, demos, benchmarks, policy/recency, citation-manager profiles, provider fixture expansion, and read-only MCP server work. Do not split the crate yet.
