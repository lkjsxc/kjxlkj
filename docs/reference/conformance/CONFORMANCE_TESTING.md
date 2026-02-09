# Conformance: Testing Surface

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

This file records current verified test coverage, not target-only expectations.

## Baseline Evidence (2026-02-09)

| Command | Result |
|---|---|
| `cargo test --workspace` | pass |
| `cargo test --workspace -- --list` | 523 tests listed |

## Verified Test Suites

| Suite | Status | Evidence |
|---|---|---|
| Dispatch tests | `implemented` | `src/crates/kjxlkj-core-state/tests/dispatch_tests.rs` |
| Integration tests | `implemented` | `src/crates/kjxlkj-core-state/tests/integration_tests.rs` |
| Headless E2E tests | `implemented` | `src/crates/kjxlkj-core-state/tests/headless_e2e_tests.rs` |
| PTY E2E tests | `partial` | `src/crates/kjxlkj-core-state/tests/pty_e2e_tests.rs` (mostly model/path checks, limited real PTY process assertions) |
| Regression tests | `implemented` | `src/crates/kjxlkj-core-state/tests/regression_tests.rs` |
| Boundary tests | `implemented` | `src/crates/kjxlkj-core-state/tests/boundary_tests_1.rs`, `boundary_tests_2.rs`, `boundary_tests_3.rs` |
| Contract tests | `implemented` | `src/crates/kjxlkj-core-state/tests/contract_tests.rs` |

## Strength Classification

| Quality Tier | Current Cases |
|---|---|
| Strong | Cursor/wrap/CJK regressions, split cycle basics, resize storms, mode churn |
| Medium | Session serialization model checks, command dispatch reachability |
| Weak | Real terminal spawn lifecycle, explorer integration, filesystem write persistence |

## Required Near-Term Coverage Closures

| Gap | Required Test Type |
|---|---|
| Real `:terminal` process wiring | PTY E2E with process lifecycle assertions |
| Explorer key workflow (`<leader>e`, open/split) | PTY E2E plus integration |
| `A` / `Shift+a` key normalization | Input decode + dispatch integration |
| Session command wiring (`SessionSave/SessionLoad`) | Integration with filesystem roundtrip |
| Real file IO (`:w`, `:e`) | Integration with on-disk assertions |
| Japanese IME terminal path | PTY E2E composition/commit/cancel scenarios |

## Related

- Target testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Boundary matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
