# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

This document defines the mandatory reconstruction testing contract.

## Goals

| Goal | Requirement |
|---|---|
| Correctness | User-reachable behavior MUST be covered by deterministic tests. |
| Drift prevention | Spec, implementation, and tests MUST remain synchronized. |
| Boundary safety | Unicode, wrapping, windows, terminal, and session edges MUST be tested explicitly. |
| Anti-shortcut | A feature is incomplete unless wired from real input path and verified. |

## Required Layers

| Layer | Required Scope |
|---|---|
| Unit | Local invariants and parser/algorithm behavior for touched modules |
| Integration | Cross-module and cross-crate behavior transitions |
| Headless E2E | Full editing workflows without PTY transport |
| PTY E2E | Real terminal input/output path and resize/focus behavior |
| Boundary/Stress | Long lines, CJK width, resize storms, mode churn, session roundtrips |

## Current In-Repo Baseline (2026-02-09)

| Suite | Current File |
|---|---|
| Dispatch coverage | `src/crates/kjxlkj-core-state/tests/dispatch_tests.rs` |
| Integration scenarios | `src/crates/kjxlkj-core-state/tests/integration_tests.rs` |
| Headless E2E | `src/crates/kjxlkj-core-state/tests/headless_e2e_tests.rs` |
| PTY E2E (model-level) | `src/crates/kjxlkj-core-state/tests/pty_e2e_tests.rs` |
| Regression scenarios | `src/crates/kjxlkj-core-state/tests/regression_tests.rs` |
| Boundary scenarios | `src/crates/kjxlkj-core-state/tests/boundary_tests_1.rs` |
| Boundary scenarios | `src/crates/kjxlkj-core-state/tests/boundary_tests_2.rs` |
| Boundary scenarios | `src/crates/kjxlkj-core-state/tests/boundary_tests_3.rs` |
| Contract/perf checks | `src/crates/kjxlkj-core-state/tests/contract_tests.rs` |

## Preferred Existing Tests (High Signal)

These tests are currently the best reconstruction anchors and SHOULD be preserved first:

| Category | Test |
|---|---|
| Cursor safety | `reg01_append_eol` |
| Wrap behavior | `reg02_long_line_wraps` |
| Unicode handling | `reg06_unicode`, `reg07_cjk_half_cell`, `reg08_wrap_padding` |
| Split lifecycle | `he06_split` |
| Session serialization baseline | `he07_session`, `bd21_session_splits` |
| Resize robustness | `pe05_resize_storm`, `bd16_resize_storm`, `bd17_resize_cjk` |
| Window command reachability | `bd40_ex_commands` |
| Mode churn stability | `bd10_mode_switch`, `bd11_visual_flicker` |

## Mandatory Additions For Next Reconstruction

The next implementation wave MUST add high-signal E2E tests from:

- [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md](/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md)

At minimum, add these missing coverage classes:

1. Real PTY terminal spawn and process lifecycle (`:terminal`, close, SIGWINCH)
2. Explorer open/split integration (`<leader>e`, open file, split from explorer)
3. Shifted printable key normalization (`A`/`Shift+a`) through input decoder
4. Session save/load command wiring (`:SessionSave`, `:SessionLoad`) with layout restore
5. File write/read persistence (`:w`, `:e`) with filesystem assertions
6. Japanese IME composition intercept under terminal input path

## Determinism Rules

- Use bounded timeouts; avoid blind sleeps.
- Use persisted state assertions where possible.
- For PTY tests, assert both behavioral output and process lifecycle.
- Every bug fix MUST add a regression test that fails on old behavior.

## Acceptance Gate

No TODO checkbox may be marked complete unless:

1. A target spec link exists.
2. A passing deterministic test exists.
3. The feature is reachable from runtime input path.
4. Any remaining user-visible gap is listed in `LIMITATIONS`.

## Related

- Unit coverage: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- E2E and boundary matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Current conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Current gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
