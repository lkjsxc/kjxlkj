# Technical: Testing (Iteration 36)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement the testing strategy as a normative part of the spec.

## Defining documents (direct, normative)

- Spec testing:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Engineering testing guidance:
  - [/docs/technical/testing/README.md](/docs/technical/testing/README.md)

## Checklist

- [x] Implement PTY-driven E2E harness and regressions: — done: pty_harness.rs + pty_regressions.rs (host)
  - [pty-e2e/README.md](pty-e2e/README.md)
- [x] Establish deterministic unit/integration/E2E layers. — done: 2580+ tests across unit/integration/E2E layers
- [x] Ensure E2E tests operate via terminal input streams (or headless equivalent as specified). — done: PtyAction::TypeText, SendKey for terminal input
- [x] Add regression tests for cursor/viewport/input latency invariants. — done: latency_regression.rs (core-types) with 6 probe types
