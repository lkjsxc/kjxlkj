# Verification: Tests (Iteration 36)

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Purpose

Treat tests as normative behavior and make regressions hard.

This checklist defines the expectations for unit, integration, and PTY-driven E2E coverage.

## Checklist (normative)

### A. Core test gate

- [ ] `cargo test --workspace` is green.
- [ ] Tests are deterministic (no timing-only assertions).

### B. PTY-driven E2E gate

- [ ] Provide a PTY-driven E2E harness that drives the real TUI path.
- [ ] Prefer persisted outputs for assertions (file writes, structured logs) over screen scraping.
- [ ] Ensure the harness runs in the verification gate and is compatible with CI.

### C. Coverage for known failure classes

- [ ] Cursor visibility invariants.
- [ ] Cursor motion boundary behavior (never panic; correct clamping).
- [ ] Viewport follow behavior (including wrap behavior).
- [ ] Input ordering (no one-key lag perception).
- [ ] Service failure recovery (filesystem, terminal, indexing, LSP as applicable).

### D. Conformance mapping

- [ ] For each conformance statement, ensure a corresponding test exists.
- [ ] For each limitation, ensure a regression test exists when feasible.

## Related

- Testing spec: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- CI gate: [/docs/reference/CI.md](/docs/reference/CI.md)
