# Regression Testing (Correctness + Performance)

Back: [/docs/technical/testing/README.md](/docs/technical/testing/README.md)
Guidance for preventing correctness and performance regressions.

## Principles

1. Prefer deterministic tests that assert invariants over tests that assert absolute timings.
2. When performance matters, gate on algorithmic behavior (O(viewport) vs O(file)), not raw speed.
3. Use benchmarks as trend signals; avoid hard CI budgets unless measurements are proven stable.

Normative requirements live in:

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

## Correctness regressions (hard gates)

Examples of good regression guards:

- “no input dropped” under typing/scroll/resize storms
- viewport/cursor visibility invariants under resize and long-line navigation
- deterministic headless scripts for bug repros (stable E2E surface)
- multiplexer PTY smoke flows (tmux/WezTerm) for attach/detach and key-latency regressions
- Japanese/IME conversion commit/cancel behavior with persisted file assertions

When a bug is fixed:

- add a test that fails on the old behavior
- link the test to the defining spec and (if user-visible) update limitations/conformance

## Performance regressions

### A. Deterministic probes (hard gates)

These should fail on algorithmic regressions and be stable under CI variance:

- viewport-bounded snapshot materialization checks
- long-line rendering virtualization checks (no full-line per-frame materialization)
- idle CPU probes (no busy-loop redraw)

See TODO leaf:

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

### B. Benchmarks (trend signals)

Benchmarks SHOULD measure:

- snapshot generation cost vs viewport size
- rendering cost vs viewport area
- file open time-to-first-snapshot for large inputs

When CI is present, benchmarks MAY be reported for trend tracking (for example: as a workflow artifact or PR comment), but SHOULD NOT block merges unless stability is demonstrated.

## Reporting

When a regression is found, record:

- the spec expectation (link)
- the reproduction probe/test (link)
- expected vs observed behavior
- whether the mitigation is “fix” or “explicit limitation/degradation”

If user-visible behavior is intentionally degraded, update:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
