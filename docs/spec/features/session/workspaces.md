# Workspaces

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Workspaces allow opening multiple project folders as a single editing environment. Each folder retains its own LSP instances, git state, and optional settings overrides while sharing a unified buffer list and window layout.

## Creating workspaces

| Method | Required behavior |
|---|---|
| CLI arguments | `kjxlkj folder1/ folder2/` MUST open a workspace containing both folders. Order of arguments MUST define folder order. |
| `:WorkspaceAdd {path}` | MUST append `{path}` as a new root folder. Duplicate paths MUST be rejected with an error. |
| `:WorkspaceOpen {file}` | MUST load a `.kjxlkj-workspace` file and replace the current workspace. Unsaved-change safeguards apply. |

## Workspace file format

Workspace files MUST use JSON with the extension `.kjxlkj-workspace`.

| Field | Type | Description |
|---|---|---|
| `version` | integer | Schema version (currently `1`). MUST be present. |
| `folders` | array of FolderEntry | Ordered list of root folders. MUST contain at least one entry. |
| `settings` | object | Global workspace settings overrides. SHOULD be optional. |

Each FolderEntry contains:

| Field | Type | Description |
|---|---|---|
| `path` | string | Absolute or workspace-file-relative path to the folder. |
| `name` | string or null | Display name override for the folder in the explorer. |
| `settings` | object or null | Per-folder settings overrides (see Per-Folder Settings). |

## File explorer

### Multi-root view

The file explorer MUST display each root folder as a top-level node. Folders MUST appear in the order defined by the workspace file or CLI arguments. Each root node MUST be collapsible independently. The explorer SHOULD visually distinguish root nodes from nested directories (e.g. bold label or icon).

### Folder operations

| Operation | Required behavior |
|---|---|
| Add folder | `:WorkspaceAdd {path}` MUST append a new root and refresh the explorer. |
| Remove folder | `:WorkspaceRemove` MUST remove the currently focused root folder. If it is the last folder, the command MUST be refused. |
| Reorder | `:WorkspaceMove {index}` SHOULD move the focused root folder to the specified position. |

## Navigation

| Key / Command | Required behavior |
|---|---|
| `<leader>1` through `<leader>9` | MUST jump focus to the root folder at that ordinal position. Out-of-range MUST report an error. |
| `<leader>wp` | MUST open a workspace picker listing all root folders for fuzzy selection. |
| `:WorkspaceSwitch {name}` | MUST focus the root folder whose `name` or directory basename matches `{name}`. |

### Finder integration

The file finder (fuzzy file open) MUST search across all workspace folders. Each result SHOULD display the folder name prefix so the user can distinguish files with identical names across roots. The scope SHOULD be filterable to a single folder via a qualifier (e.g. `folder:frontend/ query`).

## Per-folder settings

Per-folder settings override global settings for files within that folder. The resolution order MUST be: per-folder settings, then workspace-level settings, then user-global settings.

| Setting category | Examples |
|---|---|
| Formatting | `indent_style`, `indent_width`, `end_of_line` |
| LSP | `lsp.server`, `lsp.root_dir`, `lsp.settings` |
| File associations | `file_types`, `ignore_patterns` |

Settings MUST be keyed by their normal config path. An implementor MUST merge per-folder settings shallowly: a per-folder object replaces the same-level global object rather than deep-merging.

## LSP integration

Each workspace folder SHOULD launch its own LSP server instances when the folder contains language-specific project markers (e.g. `Cargo.toml`, `package.json`). The editor MUST route LSP requests to the correct server based on the file's containing root folder. Diagnostics from different servers MUST remain isolated per folder.

## Git integration

Each folder MUST maintain independent git state:

| Aspect | Requirement |
|---|---|
| Gutter signs | MUST reflect the diff of the file against its own repository HEAD. |
| Branch display | The statusline MUST show the branch of the active buffer's repository. |
| Operations | Git commands (stage, commit, diff) MUST operate on the repository that owns the active buffer. |

## Search

### Workspace-wide search

`:WorkspaceSearch {pattern}` or the search UI MUST search across all root folders. Results MUST be grouped or prefixed by folder name. The user SHOULD be able to include or exclude specific folders from the search scope.

### Finder scope

The fuzzy finder MUST default to workspace-wide scope. A `folder:` prefix SHOULD narrow the scope. When no workspace is active, the finder MUST fall back to the current working directory.

## Workspace commands

| Command | Required behavior |
|---|---|
| `:WorkspaceAdd {path}` | Add a root folder. MUST reject duplicates. |
| `:WorkspaceRemove` | Remove the focused root folder. MUST refuse if it is the last folder. |
| `:WorkspaceSave [path]` | Save the workspace to a `.kjxlkj-workspace` file. If `path` is omitted, MUST save to the previously loaded workspace file or prompt. |
| `:WorkspaceOpen {file}` | Load a workspace file, replacing the current workspace. |
| `:WorkspaceMove {index}` | Reorder the focused root folder to position `{index}`. |

## Workspace session

### Auto-save

When `session.auto_save` is `true` and a workspace file is loaded, the session system MUST save workspace-specific session data (open buffers, layout, cursor positions) alongside the workspace file path. On restore, the session loader MUST reload the workspace file first, then restore per-window state.

### Restore

The restore process MUST recover:

| Data | Requirement |
|---|---|
| Open buffers | MUST reopen buffers, matching each to its root folder. Missing files MUST substitute an empty buffer with a warning. |
| Window layout | MUST restore the split tree and tab structure. |
| Cursor positions | MUST restore per-window cursor line and column. |
| Fold state | SHOULD restore fold levels if fold data was persisted. |
| Active folder | MUST restore which root folder was focused in the explorer. |

## Recommendations

Workspace files are most useful for multi-crate Rust repositories, polyglot monorepos, and microservice architectures where each service directory has its own toolchain and LSP configuration. Users SHOULD commit workspace files to version control when the folder set is stable.

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit | workspace file parse/serialize, folder add/remove/reorder, settings merge order |
| Integration | multi-root explorer rendering, LSP routing, git isolation |
| PTY E2E | open workspace, navigate across folders, search, save/restore session |

## Related

- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Project config: [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
- File explorer: [/docs/spec/features/navigation/file-explorer.md](/docs/spec/features/navigation/file-explorer.md)
