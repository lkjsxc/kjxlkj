# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current state.

## Baseline Limitation

| ID | Requirement Link | Observed State | Expected State | Class | Severity | Next Action |
|---|---|---|---|---|---|---|
| `LIM-BASELINE-02` | [/docs/spec/README.md](/docs/spec/README.md) | docs-only standby baseline; implementation artifacts intentionally removed | reconstructed runtime that satisfies canonical spec | `M2 missing feature` | high | execute TODO phases to regenerate workspace and close blocker rows |

## Open Critical Blockers

These blockers remain open for the next reconstruction wave.

| ID | Requirement Link | Expected State | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-BLOCK-KEY-02` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches exactly as `A` through real decode path | `M1 correctness` | high | add live PTY key-event E2E + decoder trace assertions (`WR-01R`) |
| `LIM-BLOCK-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/focus/resize/close is deterministic across mixed windows | `M1 correctness` | high | add geometry-aware split E2E matrix (`WIN-01R`..`WIN-05R`) |
| `LIM-BLOCK-NAV-02` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w h/j/k/l/w/W/p/t/b` works across buffer/explorer/terminal | `M1 correctness` | high | add directional navigation oracle tests (`WINNAV-01R`..`WINNAV-06R`) |
| `LIM-BLOCK-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch, navigation, split-open, and file ops are reachable and stable | `M1 correctness` | high | add explorer interactive flow E2E (`EXP-01R`..`EXP-06R`) |
| `LIM-BLOCK-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows obey shared window-tree focus and lifecycle rules | `M1 correctness` | high | add mixed terminal/edit E2E and close/reopen stress (`TERM-01R`..`TERM-07R`) |
| `LIM-BLOCK-CURSOR-02` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor remains visible and grapheme-safe across wrap/resize/churn | `M1 correctness` | high | add cursor overlay E2E (`CUR-07R`..`CUR-11R`) |
| `LIM-BLOCK-WRAP-02` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | no overflow, no half-cell split, deterministic continuation rows | `M1 correctness` | high | add long-line viewport E2E (`WRAP-11R`..`WRAP-16R`) |
| `LIM-BLOCK-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | high-risk features gated by true runtime E2E, not only model-level tests | `M4 verification gap` | high | enforce mandatory live E2E gate for blocker closure |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TRACE-01` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | insufficient traceability from raw key events to core actions | medium | add mandatory decoder/core trace points in test harness |
| `LIM-GAP-STATE-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | insufficient invariant checks for focus history and layout transitions | medium | add state-transition boundary suite |
| `LIM-GAP-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | grouped crate topology (`app/core/platform/services`) not yet regenerated | medium | migrate workspace paths and manifests during reconstruction |

## Closure Rules

A limitation may be closed only when all are true:

1. bug/requirement is covered by deterministic regression test
2. fix is validated by matching live-path E2E (`*R`) evidence
3. related TODO items are checked with direct evidence links
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) is updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- TODO execution: [/docs/todo/current/README.md](/docs/todo/current/README.md)
