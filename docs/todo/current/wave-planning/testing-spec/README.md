# Testing Spec (Iteration 33)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Objective

Define a normative test suite (unit/integration/E2E) that can reproduce:

- Cursor visibility regressions
- Cursor movement boundary regressions
- Viewport follow regressions
- Input lag regressions
- Performance regressions under rapid typing and resize

## Requirements

- Tests MUST be specified as Given/When/Then acceptance criteria.
- Tests MUST be stable and deterministic.
- E2E tests MUST operate the editor via terminal input streams.

## Tasks

### A. Cursor visibility regression tests

- Define scenarios covering every mode and theme.
- Include overlays (completion, picker, confirm) and verify cursor remains visible.

### B. Viewport follow regression tests

- Define scenarios for:
  - small windows
  - long lines
  - wrap vs no-wrap
  - split viewports
  - scroll offsets and centering commands

### C. Input latency regression tests

- Define tests that assert “no off-by-one key perception” via:
  - input → snapshot → render ordering assertions
  - frame timing expectations

### D. Performance regression tests

- Define throughput targets and “typing burst” scenarios.
- Include resize storms and scroll bursts.
