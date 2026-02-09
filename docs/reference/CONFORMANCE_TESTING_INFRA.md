# Conformance: Testing Infrastructure

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

This file records the current state of testing infrastructure and integration enablers.

## Infrastructure Status

| Area | Status | Evidence |
|---|---|---|
| Cargo workspace test execution | `implemented` | `cargo test --workspace` passes |
| Core-state layered suites | `implemented` | `dispatch`, `integration`, `headless_e2e`, `pty_e2e`, `regression`, `boundary`, `contract` |
| Unicode and CJK algorithm checks | `implemented` | `line_wrap`, `cjk_support`, `display_width` tests |
| Window operation test coverage | `partial` | Basic split/cycle/close covered; spatial layout graph coverage missing |
| Terminal service process integration tests | `scaffold-only` | PTY module exists, but end-to-end service wiring tests are missing |
| Explorer integration tests | `partial` | Explorer unit tests exist; runtime key/command/raster integration tests missing |
| Session command end-to-end tests | `partial` | Session model tests exist; command-wired persistence tests missing |
| Filesystem write/read end-to-end tests | `partial` | Current tests mostly assert in-memory flags, not persisted bytes |

## Test Infrastructure Requirements (Not Yet Fully Met)

| Requirement | Current State |
|---|---|
| Real PTY harness asserting spawn/read/write/resize/close | Partial model tests only |
| Deterministic explorer user-path harness | Missing |
| End-to-end IME composition harness | Missing |
| Command-wire verification matrix auto-check | Missing |
| Spec-to-test traceability report generation | Missing |

## Near-Term Infrastructure TODO

- Implement a PTY harness that validates child process lifecycle and terminal window rendering.
- Add file-backed temporary workspace harness for `:w`, `:e`, session load/save, and explorer operations.
- Add automated drift checker that maps `/docs/spec/` requirement IDs to test IDs.

## Related

- Current test surface: [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)
- Target testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- High-risk missing behaviors: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
