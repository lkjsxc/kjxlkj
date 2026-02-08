# Fuzzy Finder

Back: [docs/spec/features/navigation/README.md](docs/spec/features/navigation/README.md)

Fast navigation across workspace using fuzzy matching.

## Overview

The finder is a modal overlay that supports fuzzy
text search across multiple data sources: files,
buffers, symbols, commands, and more. It provides
a unified interface for navigation and discovery.

## Activation

| Command | Source | Description |
|---------|--------|-------------|
| `:Files` | Files | Find files in workspace |
| `:Buffers` | Buffers | Switch between open buffers |
| `:Commands` | Commands | Execute ex command by name |
| `:Symbols` | Symbols | LSP workspace symbols |
| `:Lines` | Lines | Search lines in current buffer |
| `:AllLines` | Lines | Search lines across all buffers |
| `:Marks` | Marks | Jump to a mark |
| `:Registers` | Registers | View and paste registers |
| `:RecentFiles` | Recent | Recently opened files |
| `:Keymaps` | Keymaps | Search keybindings |
| `:Help` | Help tags | Open help topic |
| `:Diagnostics` | LSP | Navigate diagnostics |

## Layout

### Window Structure

The finder overlay occupies a centered floating
window, typically 80% of editor width and 60% of
height. The layout has three sections:

| Section | Position | Content |
|---------|----------|---------|
| Prompt | Top | Input field with search text |
| Results | Middle | Filtered, ranked match list |
| Preview | Right | Preview of selected item |

### Responsive Sizing

On narrow terminals (< 80 columns), preview is
hidden. The finder adapts to terminal size.

## Fuzzy Matching Algorithm

### Scoring

Matches are ranked by a fuzzy score computed from:
1. Character match positions (consecutive = higher)
2. Word boundary bonuses (CamelCase, snake_case)
3. Path separator bonuses (for file paths)
4. Exact prefix match bonus
5. Shorter matches rank higher at equal scores

### Case Sensitivity

Matching is case-insensitive by default. If the query
contains an uppercase letter, matching switches to
case-sensitive (smart-case).

### Multi-Word Queries

Space-separated tokens are matched independently.
All tokens must appear in the candidate for it to
match. Order does not matter.

## Navigation Keys

| Key | Action |
|-----|--------|
| `Ctrl-n` / `Down` | Next result |
| `Ctrl-p` / `Up` | Previous result |
| `Enter` | Accept selected item |
| `Esc` / `Ctrl-c` | Close finder |
| `Ctrl-s` | Open in horizontal split |
| `Ctrl-v` | Open in vertical split |
| `Ctrl-t` | Open in new tab |
| `Tab` | Toggle multi-select on item |
| `Ctrl-a` | Select all |
| `Ctrl-l` | Clear query |

## Preview

### File Preview

For file sources, the preview shows the file content
with syntax highlighting centered on the matched line.

### Symbol Preview

For symbol sources, the preview shows the source file
at the symbol definition location.

### Preview Toggle

`Ctrl-/` toggles the preview pane on and off.

## File Source Details

### Search Root

File search uses the workspace root as the base
directory. The workspace root is determined by:
1. Explicit `--workspace` argument
2. Git repository root
3. Current working directory

### Ignore Patterns

Files matching `.gitignore` patterns are excluded.
Additional patterns can be configured via
`finder.ignore_patterns`.

### Hidden Files

Hidden files (starting with `.`) are excluded by
default. Toggle with `Ctrl-h` during search.

## Multi-Select

### Batch Operations

In multi-select mode, multiple items can be selected
with `Tab`. After confirmation, all selected items
are opened (e.g., in separate buffers or splits).

### Quickfix Integration

`Ctrl-q` sends all multi-selected items to the
quickfix list for later navigation.

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `finder.layout` | string | "center" | Window position |
| `finder.width` | float | 0.8 | Width ratio |
| `finder.height` | float | 0.6 | Height ratio |
| `finder.preview` | bool | true | Show preview |
| `finder.ignore_patterns` | list | [] | Extra ignore globs |
| `finder.max_results` | int | 1000 | Limit results |
| `finder.smart_case` | bool | true | Smart case matching |

## Performance

### Async File Scanning

File scanning runs in the FS service task. Results
stream incrementally to the finder as they are found.

### Debounced Input

Filtering is debounced by 50ms to avoid excessive
re-computation during fast typing.

### Result Limit

Only the top N results (default: 1000) are computed
and displayed to avoid performance degradation on
large workspaces.

## Related

- File explorer: [docs/spec/features/navigation/file-explorer.md](docs/spec/features/navigation/file-explorer.md)
- Recent files: [docs/spec/commands/session/recent-files.md](docs/spec/commands/session/recent-files.md)
- Buffer switching: [docs/spec/features/buffer/buffer-switching.md](docs/spec/features/buffer/buffer-switching.md)
