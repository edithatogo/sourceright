# Track 88: Open Source Documentation and Maintenance Hardening

## Intent

Sourceright should present as a serious open source technical preview:
clear enough for a new user to try quickly, precise enough for maintainers
to govern responsibly, and disciplined enough that public claims match the
current repository evidence.

This track documents the improvement program. It does not itself rewrite the
public documentation or change product behavior.

## Current Issues

The initial audit identified these documentation and maintenance risks:

- Public status docs appear stale or internally inconsistent where README and
  release notes still describe completed extraction, provider, disambiguation,
  archive, and writeback tracks as remaining hardening work.
- Installation and distribution wording appears inconsistent where installation
  docs describe crates.io as prepared while release status records accepted
  release surfaces.
- README is not optimized as the GitHub front door because install, docs-site,
  and 60-second trial paths are not immediately visible near the top.
- Community-health coverage exists but needs maturity: support, changelog,
  governance, maintainer, security, and conduct expectations need clearer
  routing and enforcement detail.
- Issue templates do not capture enough reproducibility, privacy, version,
  operating-system, command, affected-surface, and evidence-boundary data.
- CI and status presentation are inconsistent where README badge coverage does
  not match the workflows maintainers rely on.
- Docs workflow maintenance may drift from the main CI posture where toolchain,
  action pinning, and install commands differ.

## Target Standard

The completed implementation should provide:

- A README that answers what Sourceright is, who it is for, how to install it,
  how to try it quickly, where the docs live, and what maturity claims are
  currently evidenced.
- Documentation pages that keep install, release, feature status, benchmark,
  and technical-preview wording consistent.
- Community-health files that route contributors, security reporters, support
  requests, maintainers, governance questions, and release-history readers to
  the right place.
- Issue and pull request templates that collect enough information to reproduce
  bugs, classify evidence and privacy boundaries, and avoid vague feature
  requests.
- Maintenance workflow documentation that explains CI, coverage, docs, release,
  dependency updates, branch-protection assumptions, and stale-issue handling.
- A public-facing explanation of how Conductor tracks guide governance without
  exposing internal implementation noise as user documentation.

## Documentation Audit Checklist

Future implementation should evaluate each candidate edit against this
checklist:

- Is the statement current relative to completed tracks, release status, and
  repository tests?
- Does it distinguish technical-preview, pilot-ready, benchmark-scaffold, and
  production claims?
- Can a new user install and run a minimal example from the README without
  hunting through the docs site?
- Are security, support, governance, and maintenance contacts/routing explicit?
- Do templates collect version, operating system, command, repro workspace,
  affected surface, privacy status, and expected versus actual behavior?
- Do badges and CI descriptions match the workflows that actually gate PRs and
  releases?
- Are docs workflow dependencies pinned or justified consistently with the
  repository's maintenance policy?
- Does Conductor governance remain useful to contributors without becoming
  public-facing noise?

## Boundaries

This track should not cause provider evidence to overwrite canonical CSL data,
should not alter product behavior, and should not claim production readiness
unless the repository evidence supports it.
