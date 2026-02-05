# Verification: Performance and Latency (Iteration 36)

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Purpose

Prevent performance regressions by defining measurable targets and a reproducible probe harness.

## Checklist (normative)

### A. Targets derived from spec

- [ ] Identify the performance and latency constraints implied by:
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [ ] Record the chosen targets in a single canonical location under `/docs/reference/`.

### B. Probe harness

- [ ] Implement deterministic probes for:
  - typing bursts
  - scroll bursts
  - resize storms
- [ ] Ensure probes can run headlessly and are compatible with CI.

### C. Regression policy

- [ ] Define what constitutes a regression (absolute thresholds and/or relative baselines).
- [ ] Add regression tests/benchmarks and make them part of the verification gate.

## Related

- CI gate: [/docs/reference/CI.md](/docs/reference/CI.md)
