# Latency: Regression Harness (Iteration 34)

Back: [/docs/todo/current/wave-implementation/technical/latency/README.md](/docs/todo/current/wave-implementation/technical/latency/README.md)

## Scope

Define and implement a repeatable regression harness for latency/throughput issues, including:

- CPU usage while idle (no busy-loop rendering)
- perceived “one-key lag” (ordering + rendering convergence)
- performance under typing/scroll/resize bursts
- large-file + long-line responsiveness posture

## Defining documents (direct, normative)

- Ordering + “no one-key lag” guarantees:
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Large-file posture (includes long lines and idle CPU expectations):
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Testing strategy contract:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
  - [/docs/technical/testing/load.md](/docs/technical/testing/load.md)
  - [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)

## Targets (expressed as testable requirements)

Targets MUST be expressed in a way that is stable under CI variance:

- Prefer relative comparisons (same run, different code path) over strict millisecond budgets.
- If absolute budgets are used, they MUST include generous tolerances and be limited to low-noise measurements.
- Tests MUST fail on algorithmic regressions (O(N) vs O(viewport)) rather than machine-specific speed.

## Checklist

### A. Deterministic performance probes

- [x] Add a deterministic typing-burst probe that asserts:
  - all characters are applied in order
  - the final snapshot reflects the final input (no off-by-one lag)
- [x] Add deterministic scroll + resize probes that assert cursor visibility invariants remain true.

### B. Benchmarks (repeatable, not flaky)

- [ ] Add a benchmark suite that measures:
  - snapshot generation cost vs viewport size
  - rendering cost vs viewport area
  - file open time-to-first-snapshot for large inputs
- [ ] Ensure benchmarks are runnable locally and can be used in CI as trend signals (not hard gates) when CI is present.

### C. Idle CPU regression

- [x] Add a probe that detects busy-loop redraw when idle (no input, no animation).

### D. Reporting and drift handling

- [x] When regressions are found, record:
  - the spec expectation (link)
  - the reproduction probe (link)
  - the observed behavior
  - whether the mitigation is “fix” or “explicit limitation/degradation”
- [ ] Update user-visible limitations when behavior is intentionally degraded:
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

