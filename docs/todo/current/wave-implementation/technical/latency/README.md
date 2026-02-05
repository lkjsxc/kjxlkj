# Technical: Latency (Iteration 34)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement responsiveness and latency constraints, including measurement and regression detection.

## Defining documents (direct, normative)

- Latency:
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

## Checklist

### A. Define measurable targets

- [x] Define measurable latency/throughput targets derived from the spec.
- [x] Specify measurement methodology (hardware/terminal assumptions, warmup, variance tolerance).

### B. Add regression harness

- [x] Add deterministic performance tests/benchmarks for typing, scrolling, and resize storms.
  - [regression/README.md](regression/README.md)
- [x] Add an interactive E2E smoke test that drives the real TUI and asserts "no one-key lag" behavior.

### C. Idle CPU behavior

- [x] Ensure idle CPU usage remains low (no busy-loop redraw) when there is no input and no animation.

### D. Render coalescing and backpressure

- [x] Ensure render coalescing is snapshot-sequence-aware (drop stale, render latest).
- [x] Ensure input backpressure does not drop events and does not cause "one-key lag" perception.
