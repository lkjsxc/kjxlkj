# Project-Local Configuration

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Override global settings on a per-project basis using a configuration file in the project root.

## Configuration hierarchy

Priority from highest to lowest:

| Priority | Source | Description |
|---|---|---|
| 1 | Command-line arguments | Flags passed to the binary |
| 2 | Buffer-local settings | Per-buffer overrides set by commands |
| 3 | Project config | `.kjxlkj.toml` in project root |
| 4 | Workspace config | Multi-root workspace settings |
| 5 | User config | `~/.config/kjxlkj/config.toml` |
| 6 | System defaults | Compiled-in defaults |

When a setting is defined at multiple levels, the highest-priority source wins.

## Project config file

### Location

The project configuration file MUST be named `.kjxlkj.toml` and placed at the root of the project directory (the directory containing `.git` or the working directory at startup).

### Discovery algorithm

1. Start from the opened file's directory (or `cwd` if no file).
2. Walk upward looking for `.kjxlkj.toml`.
3. Stop at the first `.git` directory boundary or filesystem root.
4. If found, merge the project config on top of user config.

### Example

| Key | Value | Effect |
|---|---|---|
| `editor.tab_width` | `4` | Override tab width for this project |
| `editor.insert_spaces` | `true` | Use spaces instead of tabs |
| `editor.line_numbers` | `relative` | Show relative line numbers |
| `editor.wrap` | `true` | Enable line wrapping |
| `filetype.python.tab_width` | `4` | Python-specific tab width |

## Workspace config

For multi-root workspaces, a `.kjxlkj-workspace.toml` file at the workspace root can define settings that apply across all roots.

| Field | Type | Description |
|---|---|---|
| `roots` | array of string | Paths to workspace roots |
| `settings` | table | Settings applied to all roots |
| `per_root` | table of tables | Per-root setting overrides keyed by root path |

## Security

### Trusted workspaces

The first time a project config is loaded from a new directory, the editor MUST prompt the user to trust the workspace. Untrusted workspaces use only user-level settings.

| Setting | Default | Description |
|---|---|---|
| `security.trust_all` | `false` | Skip trust prompts (dangerous) |
| `security.trusted_paths` | `[]` | Pre-trusted directory paths |

### Restricted settings

The following settings MUST NOT be overridden by project config for security reasons:

| Setting | Reason |
|---|---|
| `terminal.shell` | Arbitrary command execution risk |
| `editor.external_commands` | Arbitrary command execution risk |
| `security.*` | Prevents privilege escalation |

## Filetype overrides

Project config can override settings per filetype within the project scope.

| Key pattern | Example | Effect |
|---|---|---|
| `filetype.{lang}.tab_width` | `filetype.rust.tab_width = 4` | Rust files use 4-space tabs |
| `filetype.{lang}.format_on_save` | `filetype.go.format_on_save = true` | Auto-format Go files on save |
| `filetype.{lang}.lsp.server` | `filetype.python.lsp.server = "pyright"` | Use pyright for Python |

## LSP configuration

### Project-specific servers

| Key | Type | Description |
|---|---|---|
| `lsp.servers` | array of table | LSP server definitions |
| `lsp.servers[].name` | string | Server identifier |
| `lsp.servers[].command` | string | Server executable path |
| `lsp.servers[].args` | array of string | Command-line arguments |
| `lsp.servers[].filetypes` | array of string | File types this server handles |
| `lsp.servers[].settings` | table | Server-specific initialization options |

## Formatter settings

| Key | Type | Description |
|---|---|---|
| `format.command` | string | External formatter command |
| `format.args` | array of string | Arguments to formatter |
| `format.stdin` | boolean | Pass content via stdin (default: `true`) |
| `format.on_save` | boolean | Auto-format on `:w` |

## Git integration

### Auto-detect

When a `.git` directory exists at or above the project root, git features (gitsigns, blame, branch display) are automatically enabled.

### Override

| Key | Type | Description |
|---|---|---|
| `git.enabled` | boolean | Force enable/disable git integration |
| `git.gitsigns` | boolean | Show gutter signs for changes |
| `git.blame` | boolean | Show inline blame |

## Ignore files

### Custom ignores

| Key | Type | Description |
|---|---|---|
| `files.exclude` | array of glob | Patterns to exclude from file explorer and finder |
| `search.exclude` | array of glob | Additional patterns to exclude from search |
| `files.associations` | table | Override filetype detection by glob pattern |

## Environment variables

| Key | Type | Description |
|---|---|---|
| `env` | table of string | Environment variables set for subprocesses (terminal, LSP, formatters) |

Example: `env.RUSTFLAGS = "-D warnings"` sets `RUSTFLAGS` for all subprocesses.

## Session settings

| Key | Type | Description |
|---|---|---|
| `session.auto_save` | boolean | Override auto-session behavior for this project |
| `session.name` | string | Fixed session name for this project |

## Related

- Config implementation: [/docs/spec/features/config/implementation.md](/docs/spec/features/config/implementation.md)
- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Autocommands: [/docs/spec/features/config/autocommands.md](/docs/spec/features/config/autocommands.md)
