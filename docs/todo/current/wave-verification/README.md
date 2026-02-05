# Wave: Verification and Conformance (Iteration 36)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Make correctness provable and regressions difficult by treating tests and policy checks as part of the specification.

## Checklist (normative)

### A. Documentation policy compliance (always-on)

- [ ] All docs under `/docs/` contain no non-Mermaid fenced blocks.
- [ ] All docs are navigable from:
  - [/docs/README.md](/docs/README.md)
- [ ] All internal links resolve.
- [ ] The repo's verification gate is defined and reproducible locally (see [/docs/reference/CI.md](/docs/reference/CI.md)).

### B. Conformance and limitations are updated for each implemented slice

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] Ensure conformance statements match tests and actual behavior.

### C. Test suite as normative behavior

- [ ] Ensure tests cover:
  - cursor visibility
  - cursor motion boundary behavior
  - viewport follow behavior
  - input ordering (no one-key lag perception)
  - failure recovery (services, file IO, terminal)
- [ ] Prefer deterministic headless E2E tests when possible.

### D. Performance/latency baselines

- [ ] Define performance targets derived from specs.
- [ ] Add regression tests/benchmarks for typing bursts, scrolling, and resize storms.
