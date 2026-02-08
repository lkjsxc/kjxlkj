# Statusline Configuration

Back: [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)

Configurable statusline display and content.

## Overview

The statusline is a single-line display at the bottom of each window showing file information, mode, cursor position, and custom segments.

## Default Layout

| Position | Content |
|---|---|
| Left | Mode indicator, file path, modified flag |
| Center | (empty by default) |
| Right | File type, line:column, percentage |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `statusline.left` | array | `["mode", "file", "modified"]` | Left segments |
| `statusline.center` | array | `[]` | Center segments |
| `statusline.right` | array | `["filetype", "position", "percent"]` | Right segments |
| `statusline.separator` | string | `" "` | Segment separator |

## Built-in Variables

| Variable | Value |
|---|---|
| `mode` | Current mode (NORMAL, INSERT, etc.) |
| `file` | File path (relative to project root) |
| `modified` | `[+]` if buffer is modified |
| `readonly` | `[-]` if buffer is read-only |
| `filetype` | File type |
| `position` | `line:col` |
| `percent` | Position as percentage |
| `encoding` | File encoding |
| `fileformat` | Line ending format |
| `diagnostics` | Error/warning counts |
| `git_branch` | Current Git branch |
| `zoom_state` | `[Z]` if zoomed |

## Highlight Groups

| Group | Usage |
|---|---|
| `StatusLine` | Active window statusline |
| `StatusLineNC` | Inactive window statusline |
| `StatusLineMode` | Mode indicator segment |

## Inactive Windows

Inactive windows show a dimmed statusline using `StatusLineNC` highlight group. The content may differ (e.g., no mode indicator).

## Related

- Statusline DSL: [/docs/spec/features/ui/statusline/statusline-dsl.md](/docs/spec/features/ui/statusline/statusline-dsl.md)
- Statusline: [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)
