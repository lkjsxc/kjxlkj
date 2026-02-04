# Testing Spec (Iteration 34)

Back: [/docs/todo/current/wave-planning/README.md](/docs/todo/current/wave-planning/README.md)

## Objective

Define a normative test suite (unit/integration/E2E) that can reproduce:

- Cursor visibility regressions
- Cursor movement boundary regressions
- Viewport follow regressions
- Input lag regressions
- Performance regressions under rapid typing and resize

## Defining documents (direct, normative)

- Spec testing contract:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Latency requirements (drives performance tests):
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Engineering testing guidance:
  - [/docs/technical/testing/README.md](/docs/technical/testing/README.md)
  - [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)
  - [/docs/technical/testing/load.md](/docs/technical/testing/load.md)
- Current test harness surface (when present):
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Requirements

- Tests MUST be specified as Given/When/Then acceptance criteria.
- Tests MUST be stable and deterministic.
- E2E tests MUST operate the editor via terminal input streams.

## Tasks

### A. Cursor visibility regression tests

- [ ] Define scenarios covering every mode and theme.
- [ ] Include overlays (completion, picker, confirm) and verify cursor remains visible.
- [ ] Ensure tests cover redraws, resizes, and mode transitions.

### B. Viewport follow regression tests

- [ ] Define scenarios for:
  - small windows
  - long lines
  - wrap vs no-wrap
  - split viewports
  - scroll offsets and centering commands
- [ ] Include mixed workloads: edits + scrolling + resize.

### C. Input latency regression tests

- [ ] Define tests that assert “no off-by-one key perception” via:
  - input → snapshot → render ordering assertions
  - frame timing expectations
- [ ] Define what “one-key lag” means in observable terms.

### D. Performance regression tests

- [ ] Define throughput targets and “typing burst” scenarios.
- [ ] Include resize storms and scroll bursts.
- [ ] Include terminal output storms (integrated terminal panes).

### E. Doc-to-test traceability (required)

- [ ] For each test suite:
  - link the defining spec documents
  - link the conformance entry that claims support
  - list the known limitations that affect expected behavior
