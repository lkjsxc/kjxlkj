# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records open mismatches between target spec and current standby
baseline.

## Baseline Limitation

| ID | Observed State | Expected State | Severity | Next Action |
|---|---|---|---|---|
| LIM-BASELINE-01 | Source/runtime artifacts intentionally removed for reimplementation prep | Full implementation reconstructed from canonical docs | high | execute TODO phases from standby |

## Reimplementation Acceptance Blockers

These are mandatory closure targets for the next implementation wave.

| ID | Required Outcome | Spec Link | Planned Test IDs |
|---|---|---|---|
| LIM-BLOCK-KEY-01 | `Shift+a` normalizes to `A` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `WR-01` |
| LIM-BLOCK-CURSOR-01 | `a` at end-of-line differs from `i` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | `WR-02`, `WR-08` |
| LIM-BLOCK-EXP-01 | Explorer launch and split-open are fully wired | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `HE-04`, `HE-05`, `WR-05` |
| LIM-BLOCK-TERM-01 | Terminal launch creates PTY-backed window | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | `HE-06`, `WR-03`, `WR-04`, `PE-01` |
| LIM-BLOCK-WIN-01 | `Ctrl-w` mixed-window navigation is correct | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | `WR-06`, `PE-05` |
| LIM-BLOCK-I18N-01 | Japanese IME composition/cancel/leader isolation are correct | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | `JP-01` to `JP-05`, `PE-04` |
| LIM-BLOCK-WRAP-01 | Long lines never render off-screen | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | `WR-07`, `BD-01`, `BD-02`, `BD-10` |

## Lifecycle Rules

- keep limitations open until deterministic evidence closes each gap
- close limitation and TODO item in the same change
- do not remove entries without proof

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Current TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
