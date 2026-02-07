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

- [ ] Add a deterministic typing-burst probe that asserts:
  - all characters are applied in order
  - the final snapshot reflects the final input (no off-by-one lag)
  — done: `memory_latency_probes.rs` typing_burst_200/500 tests verify ordering + final state
- [ ] Add deterministic scroll + resize probes that assert cursor visibility invariants remain true. — done: latency_regression.rs probe_scroll_clamp, probe_resize_cursor

### B. Benchmarks (repeatable, not flaky)

- [ ] Add a benchmark suite that measures: — done: benchmark_suite.rs with 6 benchmark kinds
  - snapshot generation cost vs viewport size
  - rendering cost vs viewport area
  - file open time-to-first-snapshot for large inputs
- [ ] Ensure benchmarks are runnable locally and can be used in CI as trend signals (not hard gates) when CI is present. — done: benchmark_suite.rs format_benchmark_report, all_passed

### C. Idle CPU regression

- [ ] Add a probe that detects busy-loop redraw when idle (no input, no animation). — done: latency_regression.rs probe_busy_loop

### D. Reporting and drift handling

- [ ] When regressions are found, record: — done: ProbeResult with kind, passed, message, elapsed_us
  - the spec expectation (link)
  - the reproduction probe (link)
  - the observed behavior
  - whether the mitigation is “fix” or “explicit limitation/degradation”
- [ ] Update user-visible limitations when behavior is intentionally degraded: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

