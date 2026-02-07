# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

This document is the normative testing contract for reconstruction.

## Goals

| Goal | Requirement |
|---|---|
| Correctness | Critical editor behavior MUST be guarded by deterministic automated tests. |
| Reproducibility | Test outcomes MUST be stable across repeated local runs in the same environment. |
| Regression resistance | Every fixed bug MUST gain at least one test that fails on prior broken behavior. |

## Required test layers

| Layer | Required purpose |
|---|---|
| Unit | Validate local invariants and parsing/algorithm behavior. |
| Integration | Validate multi-module state transitions and command flows. |
| Headless E2E | Validate editor workflows without terminal transport variance. |
| PTY E2E | Validate real interactive path: terminal input decode -> dispatch -> render side effects. |

## Determinism rules

| Rule | Requirement |
|---|---|
| Time bounds | Tests MUST use bounded waits and deterministic deadlines. |
| Verification target | Prefer persisted state assertions (file contents, serialized state) over screen scraping. |
| Ordering | Input sequence ordering MUST be asserted for burst scenarios. |
| Failure behavior | On timeout, tests MUST fail with actionable diagnostic context. |

## Baseline implemented suites (reference mapping)

| Area | Existing suites |
|---|---|
| Core workflows and E2E | `src/crates/kjxlkj-core/tests/e2e.rs`, `src/crates/kjxlkj-core/tests/extended_e2e.rs` |
| Cursor/viewport regressions | `src/crates/kjxlkj-core-state/tests/viewport_regression.rs`, `src/crates/kjxlkj-core-state/tests/viewport_scroll_probes.rs` |
| Command parsing/execution | `src/crates/kjxlkj-core-state/tests/command_parsing.rs`, `src/crates/kjxlkj-core-state/tests/cmdline_options.rs` |
| Input and keybinding behavior | `src/crates/kjxlkj-input/tests/extended.rs`, `src/crates/kjxlkj-input/tests/comprehensive.rs` |
| PTY harness and regressions | `src/crates/kjxlkj-host/src/pty_harness.rs`, `src/crates/kjxlkj-host/src/pty_regressions.rs` |

## Mandatory regression scenarios

These scenarios are mandatory and must remain green.

| Scenario | Defining spec |
|---|---|
| Append at EOL (`a`) and return with `Esc` never leaves floating cursor | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| Long line overflow wraps to next display row by default | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| Leader feature chords remain reachable (`<leader>e`, `<leader>t`) | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) |
| Insert `Enter` persists newline through `:wq` | [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md) |
| tmux/multiplexer smoke edit-save flow | [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md) |
| Japanese/Unicode commit and cancel behavior | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) |

## Expert boundary E2E suite (required additions)

The reconstruction plan MUST include these boundary PTY E2E tests even when headless suites already pass.

| Test ID | Boundary focus |
|---|---|
| `pty_append_eol_mode_churn` | repeated `a` and `Esc` with mixed ASCII/CJK text |
| `pty_wrap_long_cjk_line` | very long Japanese line wraps and remains navigable |
| `pty_leader_vs_ime_space` | IME conversion `Space` does not trigger leader mapping |
| `pty_tmux_detach_resume` | editor remains consistent after multiplexer detach/attach |
| `pty_resize_storm_with_wrap` | rapid terminal resizes keep cursor visible and state coherent |

## Incremental verification order

| Phase | Gate |
|---|---|
| Per-leaf implementation | local unit + integration suites for touched area |
| Per-feature completion | headless E2E plus any required PTY regressions |
| Iteration close | full test suite and conformance/limitations update |

## Traceability requirement

Each mandatory scenario SHOULD be traceable across three artifacts:

- defining spec document
- concrete test file/path
- conformance or limitations entry when applicable

## Related

- Conformance ledger: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Technical regression guidance: [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)
