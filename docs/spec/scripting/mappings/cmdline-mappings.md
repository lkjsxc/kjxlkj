# Command-Line Mappings

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Mappings active in command-line mode (`:`, `/`, `?`).

## Definition (normative)

| Command | Description |
|---|---|
| `:cmap {lhs} {rhs}` | Recursive mapping in command-line mode |
| `:cnoremap {lhs} {rhs}` | Non-recursive mapping in command-line mode |
| `:cunmap {lhs}` | Remove command-line mapping |

## Built-in keys (normative)

These keys have default behavior in command-line mode and may be overridden:

| Key | Default action |
|---|---|
| `Left` / `Right` | Move cursor within command line |
| `Up` / `Down` | Navigate command history (filtered by current prefix) |
| `Ctrl-b` | Move to start of command line |
| `Ctrl-e` | Move to end of command line |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete to start of command line |
| `Tab` | Trigger completion |
| `Ctrl-r {reg}` | Insert register contents |
| `Ctrl-r Ctrl-w` | Insert word under cursor |
| `Ctrl-r Ctrl-a` | Insert WORD under cursor |
| `Ctrl-f` | Open command-line window (editable history) |

## Scope

Command-line mappings apply to all command-line sub-modes: ex commands (`:`), forward search (`/`), and backward search (`?`). To restrict to a specific sub-mode, use expression mappings that check the command-line type.

## Related

- Command-line editing: [/docs/spec/commands/cmdline/cmdline-editing.md](/docs/spec/commands/cmdline/cmdline-editing.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)

