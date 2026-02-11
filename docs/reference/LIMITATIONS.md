# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current repository state.

## Baseline Statement

The repository is currently in reconstructed foundation state. Workspace artifacts and grouped
crate topology are present, and blocker-plus-secondary reconstruction gaps are closed with live PTY
and deterministic unit evidence. Profiling and performance observability baselines are now wired.

## Open Critical Blockers

| ID | Requirement Link | Expected State | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| none | n/a | no high-severity blocker remains open in this wave | n/a | n/a | maintain release gate evidence on each behavior change |

## Closed in Current Wave

| ID | Requirement Link | Closure Date | Closure Evidence |
|---|---|---|---|
| `LIM-BASELINE-IMPL-03` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | 2026-02-11 | root workspace manifests created, `src/crates/...` grouped tree created, `cargo test -p kjxlkj-test-harness`, and reconstructed-basic verification checks passing |
| `LIM-BLOCK-KEY-03` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | 2026-02-11 | decode-normalize-dispatch path implemented in runtime, `KEY-TRACE-01` and `WR-01R` PTY tests passing in `src/crates/app/kjxlkj-test-harness/tests/key_mode_e2e.rs` |
| `LIM-BLOCK-WIN-03` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | 2026-02-11 | deterministic window tree plus session roundtrip is passing via `WIN-01R`..`WIN-05R` in `src/crates/app/kjxlkj-test-harness/tests/window_nav_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/window_nav_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/window_nav_session_terminal_e2e.rs` |
| `LIM-BLOCK-NAV-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | 2026-02-11 | mixed-window `Ctrl-w` behavior including terminal insert transition is passing via `WINNAV-01R`..`WINNAV-06R` in `src/crates/app/kjxlkj-test-harness/tests/window_nav_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/window_nav_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/window_nav_session_terminal_e2e.rs` |
| `LIM-BLOCK-EXP-03` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | 2026-02-11 | explorer route/open-target/mixed-focus plus long-label and drift churn baselines are passing via `EXP-01R`..`EXP-06R` in `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_paths_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_stress_e2e.rs` |
| `LIM-BLOCK-TERM-03` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | 2026-02-11 | terminal route/lifecycle/responsiveness/race baselines are passing via `TERM-01R`..`TERM-07R` and `BD-RACE-01` in `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_paths_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/explorer_terminal_stress_e2e.rs` |
| `LIM-BLOCK-CURSOR-03` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | 2026-02-11 | cursor visibility, wide-grapheme span, continuation exclusion, and wrap-boundary baselines are passing via `CUR-07R`..`CUR-11R` in `src/crates/app/kjxlkj-test-harness/tests/cursor_wrap_e2e.rs` |
| `LIM-BLOCK-WRAP-03` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | 2026-02-11 | wrap overflow, deterministic breakpoints, resize storm, tiny geometry, and cross-window bounds baselines are passing via `WRAP-11R`..`WRAP-16R` in `src/crates/app/kjxlkj-test-harness/tests/cursor_wrap_e2e.rs` and `src/crates/app/kjxlkj-test-harness/tests/cursor_wrap_more_e2e.rs` |
| `LIM-GAP-STATE-02` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | 2026-02-11 | focus-history and nested-rebalance invariants are covered by deterministic unit tests in `src/crates/core/kjxlkj-core-state/src/windows/tests.rs` |
| `LIM-GAP-TRACE-02` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | 2026-02-11 | failure traces include `layout_summary`, bounded `frame_excerpt`, and bounded raw+normalized action history in `src/crates/app/kjxlkj/src/main.rs`; covered by `KEY-TRACE-05` and `KEY-TRACE-07` in `src/crates/app/kjxlkj-test-harness/tests/key_mode_e2e.rs` |
| `LIM-BLOCK-TEST-03` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | 2026-02-11 | PTY harness operations are implemented (`spawn`, `send raw`, `send symbolic`, `resize`, `capture frame`, `quit`) and blocker closure for key path is proven by passing `WR-01R` and `KEY-TRACE-01` |
| `LIM-DEF-PERF-01` | [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md) | 2026-02-11 | opt-in runtime profiling metrics and probes are emitted via `PROFILE` line in `src/crates/app/kjxlkj/src/profiling.rs`, integrated in `src/crates/app/kjxlkj/src/main.rs`, and verified by `PERF-01R`..`PERF-03R` in `src/crates/app/kjxlkj-test-harness/tests/profiling_e2e.rs` |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| none | n/a | no medium-severity reconstruction gap remains open in this wave | n/a | monitor release profile evidence on each behavior change |

## Deferred Items

Deferred only if not user-visible and not correctness-critical.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| none | n/a | no deferred limitation remains open in this wave | monitor release-profile regressions on each behavior change |

## Closure Rules

A limitation may be closed only when all are true:

1. requirement is implemented and reachable from real input paths
2. deterministic regression tests pass
3. matching live PTY E2E (`*R`) passes for blocker rows
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), and
   [/docs/todo/current/README.md](/docs/todo/current/README.md) are updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Active TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
