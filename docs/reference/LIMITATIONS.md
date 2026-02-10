# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open, user-visible mismatches between target spec and real behavior.

## Open Critical Blockers

| ID | Requirement Link | Observed State | Expected State | Class | Severity | Next Action |
|---|---|---|---|---|---|---|
| `LIM-BLOCK-KEY-02` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` reported non-working in interactive use | `Shift+a` MUST dispatch exactly as `A` through real decode path | `M1 correctness` | high | add live PTY key-event E2E and decoder trace assertions (`WR-01R`) |
| `LIM-BLOCK-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split view behavior reported broken | split create/focus/resize/close MUST stay deterministic across mixed windows | `M1 correctness` | high | add geometry-aware split E2E matrix (`WIN-01R`..`WIN-05R`) |
| `LIM-BLOCK-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer behavior reported non-working | `:Explorer`, `<leader>e`, navigation, open-in-split, and file ops MUST be reachable | `M1 correctness` | high | add explorer interactive flow E2E (`EXP-01R`..`EXP-06R`) |
| `LIM-BLOCK-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows reported unstable in mixed-window usage | terminal windows MUST obey shared window-tree focus and lifecycle rules | `M1 correctness` | high | add mixed terminal/edit E2E and close/reopen stress (`TERM-01R`..`TERM-07R`) |
| `LIM-BLOCK-CURSOR-02` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor display bugs reported | cursor MUST remain visible and grapheme-safe across wrap, resize, and mode churn | `M1 correctness` | high | add render + PTY cursor overlay E2E (`CUR-07R`..`CUR-11R`) |
| `LIM-BLOCK-WRAP-02` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | line wrapping bugs reported in real use | no on-screen overflow, no half-cell split, deterministic continuation rows | `M1 correctness` | high | add long-line interactive viewport E2E (`WRAP-11R`..`WRAP-16R`) |
| `LIM-BLOCK-NAV-02` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | multi-window movement reported unreliable | `Ctrl-w h/j/k/l/w/W/p/t/b` MUST work across buffer/explorer/terminal windows | `M1 correctness` | high | add directional navigation oracle tests with golden focus traces (`WINNAV-01R`..`WINNAV-06R`) |
| `LIM-BLOCK-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | tests pass but failures still reach users | high-risk features MUST be gated by true runtime E2E checks, not only model-level tests | `M4 verification gap` | high | expand mandatory E2E matrix and make blocker closure dependent on it |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TRACE-01` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | insufficient traceability from raw key events to core actions | medium | add mandatory decoder/core trace points in test harness |
| `LIM-GAP-STATE-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | insufficient invariant checks for focus history and layout transitions | medium | add state-transition boundary suite |

## Evidence Notes

- On 2026-02-10, `cargo test --workspace` passed (230 tests).
- User-visible blockers remain open because existing tests are not sufficient proof of production behavior.

## Closure Rules

A limitation may be closed only when all are true:

1. bug is reproduced by a deterministic regression test
2. fix is validated by a live-path E2E test for the same surface
3. related TODO items are checked with direct evidence links
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) is updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- TODO execution: [/docs/todo/current/README.md](/docs/todo/current/README.md)
