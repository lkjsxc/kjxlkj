# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records open user-visible mismatches between current behavior and
spec.

## Open Limitations

| ID | Symptom | Expected | Severity | Spec Link | Next Action |
|---|---|---|---|---|---|
| LIM-INPUT-01 | `Shift+a` may not dispatch as `A` | shifted printable normalization is deterministic | high | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | close `WR-01` |
| LIM-CURSOR-01 | `a` at EOL behaves like `i` | `a` inserts after last grapheme | high | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | close `WR-02` |
| LIM-EXPLORER-01 | explorer launch may fail | `:Explorer` and `<leader>e` open explorer window | high | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | close `WR-05` |
| LIM-TERMINAL-01 | terminal launch may fail | `:terminal` and `<leader>t` open PTY-backed window | high | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | close `WR-03` and `WR-04` |
| LIM-WINDOW-01 | `Ctrl-w` mixed-window navigation is unreliable | directional focus works across buffer/explorer/terminal | high | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | close `WR-06` |
| LIM-I18N-01 | Japanese IME behavior is incorrect | composition, cancel, and leader isolation are correct | high | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | close `JP-01` to `JP-05` |
| LIM-WRAP-01 | long lines may overflow off-screen | content remains on-screen with wrap rules | high | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | close `WR-07` and `BD-01` |
| LIM-WIRE-01 | implemented features may be unreachable | all claimed features are input-to-output wired | high | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | run phase gates with evidence |
| LIM-SVC-01 | LSP/Git/Index remain incomplete | service features are fully wired and tested | medium | [/docs/spec/features/README.md](/docs/spec/features/README.md) | close phase 4 |
| LIM-A11Y-01 | accessibility evidence is missing | accessibility behavior verified deterministically | medium | [/docs/spec/ux/accessibility.md](/docs/spec/ux/accessibility.md) | add tests and evidence |

## Lifecycle Rules

- keep limitation open until deterministic evidence closes the exact gap
- close limitation and TODO item in the same change
- never remove limitation entries without proof

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
