# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current repository state.

## Baseline Statement

The repository is currently in docs-only baseline state. Runtime implementation artifacts are
intentionally absent until reconstruction starts.

## Open Critical Blockers

| ID | Requirement Link | Expected State | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BASELINE-IMPL-03` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | grouped workspace and source tree are reconstructed | `M2 missing feature` | high | regenerate workspace manifests and `src/crates/...` tree |
| `LIM-BLOCK-KEY-03` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` decodes to `A` and produces append semantics | `M2 missing feature` | high | implement decode and dispatch, then pass `WR-01R`, `KEY-TRACE-01` |
| `LIM-BLOCK-WIN-03` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split/create/close/rebalance preserves deterministic focus and geometry | `M2 missing feature` | high | implement and pass `WIN-01R`..`WIN-05R` |
| `LIM-BLOCK-NAV-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w h/j/k/l/w/W/p/t/b` deterministic across window types | `M2 missing feature` | high | implement and pass `WINNAV-01R`..`WINNAV-06R` |
| `LIM-BLOCK-EXP-03` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch/actions are reachable and stable | `M2 missing feature` | high | implement and pass `EXP-01R`..`EXP-06R` |
| `LIM-BLOCK-TERM-03` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows obey shared lifecycle and remain responsive | `M2 missing feature` | high | implement and pass `TERM-01R`..`TERM-07R` |
| `LIM-BLOCK-CURSOR-03` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor remains visible and never targets half-cell continuation states | `M2 missing feature` | high | implement and pass `CUR-07R`..`CUR-11R` |
| `LIM-BLOCK-WRAP-03` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrapping never overflows and preserves wide-grapheme atomicity | `M2 missing feature` | high | implement and pass `WRAP-11R`..`WRAP-16R` |
| `LIM-BLOCK-TEST-03` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure requires true PTY E2E evidence | `M4 verification gap` | high | reconstruct PTY harness and enforce blocker gate |

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
