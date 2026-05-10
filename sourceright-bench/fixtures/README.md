# Fixtures

Fixtures must be self-authored, public-domain, or otherwise clearly licensed for
redistribution. Do not add publisher PDFs, private manuscripts, API dumps with
restricted terms, or personally identifying data.

Provider fixtures should preserve enough raw payload to test parsing and
provenance, but live provider calls must remain outside the default benchmark
path.

Large stress fixtures are now stored under `workspace-stress/` and are used by
`tasks-stress.yaml` in scheduled/manual robustness jobs.
