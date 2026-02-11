# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and observed runtime behavior.

## Baseline Statement

Implementation artifacts are present, but blocker closure claims from prior waves are not trusted.
This file is authoritative for open high-severity user-visible gaps.

## Open Critical Blockers

| ID | Requirement Link | Expected State | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BLOCK-KEY-03` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` must decode to `A` before mode dispatch and produce append semantics | `M1 correctness` | high | add `WR-01R`, `KEY-TRACE-01`, and raw-bytes PTY trace proof |
| `LIM-BLOCK-WIN-03` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split/create/close/rebalance must preserve one valid focus and geometry invariants | `M1 correctness` | high | close with `WIN-01R`..`WIN-05R` and geometry oracle logs |
| `LIM-BLOCK-NAV-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w h/j/k/l/w/W/p/t/b` must behave deterministically across all window types | `M1 correctness` | high | close with `WINNAV-01R`..`WINNAV-06R` focus trace replay |
| `LIM-BLOCK-EXP-03` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch, navigation, split-open, and file actions must be reachable and stable | `M1 correctness` | high | close with `EXP-01R`..`EXP-06R` on real command/key paths |
| `LIM-BLOCK-TERM-03` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows must obey shared window-tree lifecycle and remain responsive | `M1 correctness` | high | close with `TERM-01R`..`TERM-07R` and child-reap proof |
| `LIM-BLOCK-CURSOR-03` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor must remain visible and never target half-cell continuation states | `M1 correctness` | high | close with `CUR-07R`..`CUR-11R` in resize/focus churn |
| `LIM-BLOCK-WRAP-03` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrapping must never overflow and must preserve wide-grapheme atomicity | `M1 correctness` | high | close with `WRAP-11R`..`WRAP-16R` mixed-window runs |
| `LIM-BLOCK-TEST-03` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure requires true PTY E2E, not unit-only evidence | `M4 verification gap` | high | enforce blocker gate requiring matching `*R` pass evidence |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-STATE-02` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | insufficient state invariants for focus history and ancestor rebalance | medium | add invariant checks and property tests for tree mutations |
| `LIM-GAP-TRACE-02` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | weak decode-to-action traceability in failure diagnostics | medium | add mandatory trace capture fields in PTY harness diagnostics |
| `LIM-GAP-DOC-02` | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | doc inventory drifted from actual markdown count | medium | regenerate coverage parts in this wave and re-verify links |

## Deferred Items

Deferred only if not user-visible and not correctness-critical.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| `LIM-DEF-PERF-01` | [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md) | performance tuning is blocked behind correctness closure | after all high-severity blockers close |

## Closure Rules

A limitation may be closed only when all are true:

1. bug/requirement is reproduced with deterministic steps
2. a regression test is added and passes
3. matching live PTY E2E (`*R`) passes
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), and
   [/docs/todo/current/README.md](/docs/todo/current/README.md) are updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Active TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
