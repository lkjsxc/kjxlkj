# Session Management

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

kjxlkj supports session persistence for workflow continuity. Multi-window state is saved to and loaded from JSON files.

## What sessions save

| Category | Saved data |
|---|---|
| Buffers | File paths, modified flags, buffer options, encoding |
| Window layout | Full layout tree (splits, weights, window types) |
| Cursor positions | Per-window cursor `(line, grapheme_offset)` |
| Viewport state | Per-window `top_line`, `left_col`, `wrap` |
| Terminal windows | Window position and size (process state is NOT saved) |
| Tab pages | Tab ordering and active tab |
| Working directory | The `cwd` at save time |
| Marks | Global marks (`A-Z`) with file path and position |
| Registers | Named registers (`a-z`) content (optional, configurable) |

## Session file format (JSON, normative)

Sessions MUST be stored as JSON files. The schema is defined below.

### Top-level structure

| Field | Type | Description |
|---|---|---|
| `version` | integer | Schema version (currently `1`) |
| `cwd` | string | Working directory at save time |
| `timestamp` | string | ISO 8601 timestamp of save |
| `tabs` | array of Tab | Tab pages |
| `active_tab` | integer | Index of the active tab in the `tabs` array |
| `buffers` | array of BufferRef | Buffer metadata |
| `marks` | object | Global marks: key is mark char, value is MarkRef |
| `registers` | object | Named registers: key is register char, value is string (optional) |

### Tab structure

| Field | Type | Description |
|---|---|---|
| `layout` | LayoutNode | Root of the window layout tree |
| `focused_window` | integer | Index of the focused window in leaf traversal order |

### LayoutNode (recursive)

| Field | Type | Description |
|---|---|---|
| `type` | string | One of `leaf`, `hsplit`, `vsplit` |
| `children` | array of LayoutNode | Present when `type` is `hsplit` or `vsplit` |
| `weights` | array of float | Weight for each child (same length as `children`) |
| `window` | WindowRef | Present when `type` is `leaf` |

### WindowRef

| Field | Type | Description |
|---|---|---|
| `content_type` | string | `buffer` or `terminal` |
| `buffer_path` | string or null | File path for buffer windows; null for terminal windows |
| `cursor_line` | integer | Cursor line (0-based) |
| `cursor_grapheme` | integer | Cursor grapheme offset (0-based) |
| `top_line` | integer | Viewport top line |
| `left_col` | integer | Viewport left column (0 when wrap is true) |
| `wrap` | boolean | Whether soft-wrap is enabled |
| `options` | object | Per-window options (scrolloff, line numbers, etc.) |

### BufferRef

| Field | Type | Description |
|---|---|---|
| `path` | string | Absolute file path |
| `encoding` | string | Character encoding (e.g. `utf-8`) |
| `modified` | boolean | Whether buffer had unsaved changes |

### MarkRef

| Field | Type | Description |
|---|---|---|
| `path` | string | File path |
| `line` | integer | Line number (0-based) |
| `grapheme` | integer | Grapheme offset (0-based) |

## Session file location

| Location | Path |
|---|---|
| Default directory | `~/.local/share/kjxlkj/sessions/` |
| Named sessions | `~/.local/share/kjxlkj/sessions/{name}.json` |
| Auto sessions | `~/.local/share/kjxlkj/sessions/auto_{hash}.json` where `hash` is a hash of the working directory path |

## Commands

| Command | Description |
|---|---|
| `:SessionSave [name]` | Save current session; uses auto-name if no name given |
| `:SessionSave! [name]` | Force overwrite existing session |
| `:SessionLoad [name]` | Load session by name or open a picker |
| `:SessionDelete [name]` | Delete a saved session |
| `:SessionNew` | Discard current session and start fresh |

## Auto-session behavior

When `session.auto_save` is `true`:

- On exit, the session is saved automatically to the auto-session file for the current `cwd`.
- On startup in a directory with an existing auto-session, the session is restored automatically.
- The auto-session file name is derived from a deterministic hash of the absolute `cwd` path.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `session.auto_save` | `true` | Auto-save session on exit |
| `session.auto_restore` | `true` | Auto-restore session on startup |
| `session.save_registers` | `false` | Include register contents in session file |
| `session.directory` | `~/.local/share/kjxlkj/sessions/` | Session storage directory |

## Load algorithm

1. Parse JSON file and validate `version` field.
2. Change `cwd` to saved working directory.
3. Open buffers from `buffers` array (skip missing files with a warning).
4. Reconstruct the layout tree from the `tabs` array.
5. For each leaf window, match `buffer_path` to opened buffers.
6. For terminal windows, create a new terminal (previous process is not restored).
7. Restore cursor positions and viewport state per window.
8. Restore global marks and optionally registers.
9. Set active tab and focused window.

## Error handling

| Error | Behavior |
|---|---|
| Missing buffer file | Open with empty buffer; show warning notification |
| Invalid JSON | Abort load; show error notification |
| Unknown version | Abort load; show "unsupported session version" error |

## Related

- Session features: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Workspace management: [/docs/spec/features/session/workspaces.md](/docs/spec/features/session/workspaces.md)
