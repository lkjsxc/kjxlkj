# Wave: Verification and Conformance (Iteration 34)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Make correctness provable and regressions difficult by treating tests and policy checks as part of the specification.

## Checklist (normative)

### A. Documentation policy compliance (always-on)

- [x] All docs under `/docs/` contain no non-Mermaid fenced blocks.
- [x] All docs are navigable from:
  - [/docs/README.md](/docs/README.md)
- [x] All internal links resolve.
- [x] The repo's verification gate is defined and reproducible locally (see [/docs/reference/CI.md](/docs/reference/CI.md)).

### B. Conformance and limitations are updated for each implemented slice

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] Ensure conformance statements match tests and actual behavior.

### C. Test suite as normative behavior

- [x] Ensure tests cover:
  - cursor visibility
  - cursor motion boundary behavior
  - viewport follow behavior
  - input ordering (no one-key lag perception)
  - failure recovery (services, file IO, terminal)
- [x] Prefer deterministic headless E2E tests when possible.

### D. Performance/latency baselines

- [x] Define performance targets derived from specs.
- [x] Add regression tests/benchmarks for typing bursts, scrolling, and resize storms.
