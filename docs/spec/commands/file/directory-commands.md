# Directory Commands

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Commands for changing and querying the working directory.

## Current directory

| Command | Description |
|---|---|
| `:pwd` | Print the current working directory |
| `:cd {path}` | Change the working directory to `{path}` |
| `:cd` | Change to the home directory (`~`) |
| `:cd -` | Change to the previous working directory |

## Local directory

| Command | Description |
|---|---|
| `:lcd {path}` | Set the working directory for the current window only |
| `:tcd {path}` | Set the working directory for the current tab only |

### Scope hierarchy

| Scope | Command | Affects |
|---|---|---|
| Global | `:cd` | All windows and tabs |
| Tab-local | `:tcd` | All windows in the current tab |
| Window-local | `:lcd` | Only the current window |

Window-local overrides tab-local, which overrides global.

## Auto-directory

| Setting | Type | Default | Description |
|---|---|---|---|
| `editor.autochdir` | boolean | `false` | Automatically change directory to the file's directory when switching buffers |

When `autochdir = true`, opening or switching to a buffer changes the window-local directory to the parent directory of the file.

## Project root detection

| Setting | Type | Default | Description |
|---|---|---|---|
| `editor.root_markers` | array of string | `[".git", "Cargo.toml", "package.json"]` | Files/directories that indicate a project root |
| `editor.root_strategy` | string | `nearest` | `nearest` (closest marker) or `workspace` (topmost marker) |

`:cd` with no argument when `root_strategy` is set changes to the detected project root.

## Events

Changing the directory fires the `DirChanged` event (see [/docs/spec/features/config/hooks-events.md](/docs/spec/features/config/hooks-events.md)), which can trigger autocommands.

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Project config: [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
