# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

This document is the normative testing contract for reconstruction. Detailed per-crate unit requirements are in [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md). Integration, end-to-end, and boundary scenarios are in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md).

## Goals

| Goal | Requirement |
|---|---|
| Correctness | Critical editor behavior MUST be guarded by deterministic automated tests. |
| Reproducibility | Test outcomes MUST be stable across repeated local runs in the same environment. |
| Regression resistance | Every fixed bug MUST gain at least one test that fails on prior broken behavior. |
| CJK fidelity | Wide-character rendering, cursor placement, and wrapping MUST have dedicated tests at every layer. |
| Coverage gating | Each crate MUST include the minimum unit suites specified in [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md) before the crate is considered complete. |

## Required test layers

| Layer | Purpose | Minimum scope |
|---|---|---|
| Unit | Validate local invariants, parsing, and algorithm behavior. | Every public function and every `unsafe` block. |
| Integration | Validate multi-module state transitions and command flows. | At least one scenario per cross-crate API boundary. |
| Headless E2E | Validate editor workflows without terminal transport variance. | All mandatory regression scenarios below. |
| PTY E2E | Validate real interactive path: terminal input decode, dispatch, render side effects. | All boundary PTY scenarios in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md). |

## Determinism rules

| Rule | Requirement |
|---|---|
| Time bounds | Tests MUST use bounded waits and deterministic deadlines; no `sleep`-based synchronization. |
| Verification target | Prefer persisted state assertions (file contents, serialized state) over screen scraping. |
| Ordering | Input sequence ordering MUST be asserted for burst scenarios. |
| Failure behavior | On timeout, tests MUST fail with actionable diagnostic context including test name, expected state, and actual state. |
| Seed control | Any randomized or property-based test MUST accept a reproducible seed. |

## Mandatory regression scenarios

These scenarios MUST remain green at all times.

| ID | Scenario | Defining spec |
|---|---|---|
| REG-01 | Append at EOL (`a`) and return with `Esc` never leaves floating cursor | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| REG-02 | Long line overflow wraps to next display row by default | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| REG-03 | Leader feature chords remain reachable (`<leader>e`, `<leader>t`) | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) |
| REG-04 | Insert `Enter` persists newline through `:wq` | [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md) |
| REG-05 | tmux/multiplexer smoke edit-save flow | [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md) |
| REG-06 | Japanese/Unicode commit and cancel behavior | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) |
| REG-07 | CJK cursor never occupies a half-cell position | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| REG-08 | Width-2 grapheme at wrap boundary produces padding cell, not split | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |

## Baseline implemented suites (reference mapping)

| Area | Existing suites |
|---|---|
| Core workflows and E2E | `src/crates/kjxlkj-core/tests/e2e.rs`, `src/crates/kjxlkj-core/tests/extended_e2e.rs` |
| Cursor/viewport regressions | `src/crates/kjxlkj-core-state/tests/viewport_regression.rs`, `src/crates/kjxlkj-core-state/tests/viewport_scroll_probes.rs` |
| Command parsing/execution | `src/crates/kjxlkj-core-state/tests/command_parsing.rs`, `src/crates/kjxlkj-core-state/tests/cmdline_options.rs` |
| Input and keybinding behavior | `src/crates/kjxlkj-input/tests/extended.rs`, `src/crates/kjxlkj-input/tests/comprehensive.rs` |
| PTY harness and regressions | `src/crates/kjxlkj-host/src/pty_harness.rs`, `src/crates/kjxlkj-host/src/pty_regressions.rs` |

## Incremental verification order

| Phase | Gate |
|---|---|
| Per-leaf implementation | Local unit plus integration suites for touched area. |
| Per-feature completion | Headless E2E plus any required PTY regressions. |
| Iteration close | Full test suite, conformance update, and limitations update. |

## Traceability requirement

Each mandatory scenario MUST be traceable across three artifacts: the defining spec document, the concrete test file path, and the conformance or limitations entry when applicable.

## Related

- Unit test requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- E2E and boundary tests: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Conformance ledger: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Technical regression guidance: [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)
