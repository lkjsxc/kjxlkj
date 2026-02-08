# Terminal Mode Mappings

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Mappings active when the terminal emulator window is focused in terminal-insert mode.

## Definition (normative)

| Command | Description |
|---|---|
| `:tmap {lhs} {rhs}` | Recursive mapping in terminal mode |
| `:tnoremap {lhs} {rhs}` | Non-recursive mapping in terminal mode |
| `:tunmap {lhs}` | Remove terminal-mode mapping |

## Default terminal keybindings (normative)

| Key | Action |
|---|---|
| `Ctrl-\ Ctrl-n` | Exit to Normal mode (stop forwarding keys to PTY) |
| `Esc Esc` | Exit to Normal mode (alternative double-escape) |
| `Ctrl-w h/j/k/l` | Navigate to adjacent window (pass-through to window system) |
| `Ctrl-w c` | Close terminal window |

All other keys are forwarded to the PTY as raw input.

## Custom mappings

Users may define mappings to add terminal-specific shortcuts. The `{rhs}` of a terminal mapping can include `<C-\><C-n>` to exit to Normal mode before executing further actions.

## Passthrough vs interception

By default, most keys pass through to the PTY. Only mapped sequences and the built-in exit sequences are intercepted. If a user maps `Esc` in terminal mode, single-`Esc` no longer passes to the PTY.

## Auto-insert on focus

When `terminal.start_insert` is true (default), focusing a terminal window automatically enters terminal-insert mode. The user can override this with a mapping.

## Related

- Terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)

