# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current repository state.

## Baseline Statement

The repository is currently in reconstructed foundation state. Workspace artifacts and grouped
crate topology are present, while runtime behavior blockers remain open.

## Open Critical Blockers

| ID | Requirement Link | Expected State | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BLOCK-EXP-03` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch/actions are reachable and stable | `M2 missing feature` | high | implement and pass `EXP-01R`..`EXP-06R` |
| `LIM-BLOCK-TERM-03` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows obey shared lifecycle and remain responsive | `M2 missing feature` | high | implement and pass `TERM-01R`..`TERM-07R` |
| `LIM-BLOCK-CURSOR-03` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor remains visible and never targets half-cell continuation states | `M2 missing feature` | high | implement and pass `CUR-07R`..`CUR-11R` |
| `LIM-BLOCK-WRAP-03` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrapping never overflows and preserves wide-grapheme atomicity | `M2 missing feature` | high | implement and pass `WRAP-11R`..`WRAP-16R` |

## Closed in Current Wave

| ID | Requirement Link | Closure Date | Closure Evidence |
|---|---|---|---|
| `LIM-BASELINE-IMPL-03` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | 2026-02-11 | root workspace manifests created, `src/crates/...` grouped tree created, `cargo test -p kjxlkj-test-harness`, and reconstructed-basic verification checks passing |
| `LIM-BLOCK-KEY-03` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | 2026-02-11 | decode-normalize-dispatch path implemented in runtime, `KEY-TRACE-01` and `WR-01R` PTY tests passing in `src/crates/app/kjxlkj-test-harness/tests/key_mode_e2e.rs` |
| `LIM-BLOCK-WIN-03` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | 2026-02-11 | deterministic window tree plus session roundtrip is passing via `WIN-01R`..`WIN-05R` in `src/crates/app/kjxlkj-test-harness/tests/window_nav_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/window_nav_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/window_nav_session_terminal_e2e.rs` |
| `LIM-BLOCK-NAV-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | 2026-02-11 | mixed-window `Ctrl-w` behavior including terminal insert transition is passing via `WINNAV-01R`..`WINNAV-06R` in `src/crates/app/kjxlkj-test-harness/tests/window_nav_e2e.rs`, `src/crates/app/kjxlkj-test-harness/tests/window_nav_more_e2e.rs`, and `src/crates/app/kjxlkj-test-harness/tests/window_nav_session_terminal_e2e.rs` |
| `LIM-BLOCK-TEST-03` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | 2026-02-11 | PTY harness operations are implemented (`spawn`, `send raw`, `send symbolic`, `resize`, `capture frame`, `quit`) and blocker closure for key path is proven by passing `WR-01R` and `KEY-TRACE-01` |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-STATE-02` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | invariant suite for focus history and ancestor rebalance not yet reconstructed | medium | implement state invariants and property tests |
| `LIM-GAP-TRACE-02` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | decode-to-action trace diagnostics not yet reconstructed | medium | implement required trace fields in harness |

## Deferred Items

Deferred only if not user-visible and not correctness-critical.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| `LIM-DEF-PERF-01` | [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md) | performance tuning is blocked behind correctness closure | after high-severity blockers close |

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
