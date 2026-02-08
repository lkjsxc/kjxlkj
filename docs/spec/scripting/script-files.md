# Script Files

Back: [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md)

Script files provide configuration and automation. kjxlkj uses TOML for declarative configuration and a simple command-file format for imperative scripts.

## File locations (normative)

| Path | Type | Purpose |
|---|---|---|
| `~/.config/kjxlkj/config.toml` | TOML | Main user configuration |
| `~/.config/kjxlkj/init.kjxlkj` | Command file | Initialization commands (sourced on startup) |
| `.kjxlkj.toml` | TOML | Project-local configuration overrides |
| `~/.config/kjxlkj/themes/{name}.toml` | TOML | Theme definitions |
| `~/.config/kjxlkj/snippets/{ft}.json` | JSON | Snippet definitions per filetype |

## TOML configuration format (normative)

The `config.toml` file uses standard TOML tables:

| Table | Purpose | Example keys |
|---|---|---|
| `[editor]` | Core editor options | `tabstop`, `shiftwidth`, `expandtab`, `number`, `relativenumber`, `wrap`, `scrolloff` |
| `[editor.cursor]` | Cursor rendering | `shape`, `blink` |
| `[terminal]` | Terminal emulator options | `shell`, `scrollback_lines`, `start_insert` |
| `[session]` | Session management | `auto_save`, `auto_restore` |
| `[ui]` | UI preferences | `colorscheme`, `statusline` |
| `[lsp]` | LSP settings | server configs per language |
| `[mappings.normal]` | Normal-mode keybindings | key-to-action pairs |
| `[mappings.insert]` | Insert-mode keybindings | key-to-action pairs |
| `[mappings.visual]` | Visual-mode keybindings | key-to-action pairs |
| `[filetype]` | Per-filetype overrides | `[filetype.rust]`, etc. |

## Command file format (normative)

Files sourced with `:source` contain one ex command per line. Lines starting with `"` are comments. Empty lines are ignored.

## Loading order (normative)

On startup, configuration is loaded in this order:

1. System defaults (compiled into binary)
2. `~/.config/kjxlkj/config.toml`
3. `~/.config/kjxlkj/init.kjxlkj`
4. `.kjxlkj.toml` (project-local, if present and trusted)

Later sources override earlier ones for the same key.

## Sourcing at runtime

| Command | Action |
|---|---|
| `:source {file}` | Execute commands from `{file}` |
| `:source %` | Re-source the current file |

## Related

- Project config: [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
- Source command: [/docs/spec/commands/execution/source-command.md](/docs/spec/commands/execution/source-command.md)
- Startup sequence: [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)

