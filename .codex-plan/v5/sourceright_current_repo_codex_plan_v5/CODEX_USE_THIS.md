# How to give this plan to Codex

Use this package as instruction material, not as a blind patch.

## 1. Prepare a branch

```bash
git clone https://github.com/edithatogo/sourceright.git
cd sourceright
git checkout -b current-repo-sota-plan
```

## 2. Unpack this package outside the repo

```bash
mkdir -p ../sourceright-plan-v5
unzip ~/Downloads/sourceright_current_repo_codex_plan_v5.zip -d ../sourceright-plan-v5
```

## 3. Copy only the Codex instruction file

```bash
cp ../sourceright-plan-v5/sourceright_current_repo_codex_plan_v5/AGENTS.md.template ./AGENTS.md
```

## 4. Start Codex

```bash
codex
```

Paste `codex/prompts/00-inspect-current-repo.md` first. Do not ask Codex to modify files until it has produced the audit and slice plan.

## 5. Run slices sequentially

Use the prompts in order:

1. `00-inspect-current-repo.md`
2. `01-baseline-checks.md`
3. `02-schema-contracts.md`
4. `03-plugin-registry.md`
5. `04-demonstrators.md`
6. `05-benchmark-harness.md`
7. `06-policy-style-recency.md`
8. `07-provider-fixture-expansion.md`
9. `08-citation-manager-profiles.md`
10. `09-mcp-readonly-plan.md`
11. `10-final-review.md`

## Important

The folder `reference_additive_scaffolds/` is reference material only. Let Codex adapt it to the current repo after inspection.
