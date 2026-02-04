# Proposal: Performance and CPU Regression Harness

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem statement

Reported gaps include:

- CPU usage higher than expected (vs Vim/Neovim) when idle or under typing/scrolling
- large-file open significantly slower than baseline editors

Without a repeatable regression harness, these issues are likely to recur or worsen during feature development.

## Defining documents

- TODO leaf:
  - [/docs/todo/current/wave-implementation/technical/latency/regression/README.md](/docs/todo/current/wave-implementation/technical/latency/regression/README.md)
- Latency/order spec (no one-key lag, coalescing, idle rendering):
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Large-file spec (viewport-bounded snapshots, idle CPU posture):
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Load and regression testing guidance:
  - [/docs/technical/testing/load.md](/docs/technical/testing/load.md)
  - [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)

## Proposed approach

### A. Separate “correctness under stress” from “speed”

1. Deterministic stress tests (hard gate):
   - prove no dropped input and correct final state under typing/scroll/resize storms
   - prove viewport/cursor invariants remain true
2. Benchmarks (trend signal):
   - track relative performance changes over time
   - avoid brittle absolute budgets in CI unless extremely stable

### B. Add an idle CPU probe

Define a test/probe that can detect:

- continuous redraw when idle
- continuous snapshot rebuilding without input

This should map directly to the spec requirement to avoid idle busy-loop rendering.

### C. Large-file open probe

Define a probe measuring:

- time-to-first-snapshot after opening a large file
- peak memory allocations (when feasible)

Use it to guide streaming I/O improvements and avoid regressions.

## Test plan (required)

- Deterministic headless tests for ordering correctness under stress.
- At least one interactive PTY E2E that asserts the “no one-key lag” guarantee in the real TUI path.
- Benchmarks for snapshot/render/open that can be run locally and used in CI as a trend report when CI is present.

## Risks / open questions

- Comparative “Vim vs kjxlkj” measurements are environment-sensitive; treat them as manual baselines unless stabilized.
- Benchmarks can cause CI flakiness if turned into hard gates; prefer trend reporting unless budgets are robust.

