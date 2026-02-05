# Technical: Testing (Iteration 35)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement the testing strategy as a normative part of the spec.

## Defining documents (direct, normative)

- Spec testing:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Engineering testing guidance:
  - [/docs/technical/testing/README.md](/docs/technical/testing/README.md)

## Checklist

- [x] Implement PTY-driven E2E harness and regressions:
  - [pty-e2e/README.md](pty-e2e/README.md)
- [x] Establish deterministic unit/integration/E2E layers.
- [x] Ensure E2E tests operate via terminal input streams (or headless equivalent as specified).
- [x] Add regression tests for cursor/viewport/input latency invariants.
