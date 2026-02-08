# Live Grep

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

Project-wide search with real-time results using an external grep tool.

## Overview

Live grep searches all files in the project directory for a pattern. Results are displayed in a picker as the user types. The search is delegated to an external tool (ripgrep by default).

## Activation

| Key | Action |
|---|---|
| `<leader>fg` | Open live grep picker |
| `<leader>fG` | Live grep with additional options |

| Command | Description |
|---|---|
| `:LiveGrep [pattern]` | Open live grep with optional initial pattern |

## Search tool

| Setting | Default | Description |
|---|---|---|
| `grep.command` | `rg` (ripgrep) | External grep command |
| `grep.args` | `["--color=never", "--no-heading", "--with-filename", "--line-number", "--column"]` | Default arguments |

The editor spawns the grep command as a subprocess and streams results.

## Real-time behavior

As the user types the pattern:

1. Debounce keystrokes (150ms default)
2. Kill any running grep subprocess
3. Spawn new grep subprocess with updated pattern
4. Stream results into the picker

## Result display

Each result line shows:

| Column | Content |
|---|---|
| File path | Relative path from project root |
| Line | Line number |
| Column | Column number |
| Match | Matched text with surrounding context |

## Navigation

| Key | Action |
|---|---|
| `<CR>` | Open file at match location |
| `<C-v>` | Open in vertical split |
| `<C-s>` | Open in horizontal split |
| `<C-q>` | Send all results to quickfix list |
| `<Esc>` | Close picker |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `grep.debounce_ms` | integer | `150` | Debounce interval |
| `grep.max_results` | integer | `1000` | Maximum results |
| `grep.respect_gitignore` | boolean | `true` | Respect `.gitignore` patterns |

## Quickfix integration

Pressing `<C-q>` sends all current results to the quickfix list for persistent navigation with `:cnext` / `:cprev`.

## Related

- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- Quickfix: [/docs/spec/features/navigation/quickfix.md](/docs/spec/features/navigation/quickfix.md)
