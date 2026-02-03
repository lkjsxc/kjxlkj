# Technical: Latency (Iteration 33)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement responsiveness and latency constraints, including measurement and regression detection.

## Defining documents (direct, normative)

- Latency:
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

## Checklist

- [ ] Define measurable latency targets derived from the spec.
- [ ] Add deterministic performance tests/benchmarks for typing, scrolling, and resize storms.
- [ ] Ensure render coalescing and backpressure do not cause one-key lag.

