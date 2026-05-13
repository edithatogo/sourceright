# Benchmark-ready checklist

## Internal regression benchmark

- [ ] `sourceright bench` behavior documented.
- [ ] `tasks.yaml` runner status matches reality.
- [ ] baseline files exist for all runnable tasks.
- [ ] fixtures are deterministic and small enough for PR gates.
- [ ] stress tasks are separate from PR gates.
- [ ] benchmark results page exists.
- [ ] no live provider calls required.
- [ ] no API keys required.
- [ ] schema validity is checked where applicable.
- [ ] latency budgets documented where applicable.

## External comparative benchmark

- [ ] clearly marked as future/not yet implemented unless actually done.
- [ ] baselines defined.
- [ ] metrics defined.
- [ ] datasets/fixtures licensed safely.
- [ ] no SOTA claims before results.
