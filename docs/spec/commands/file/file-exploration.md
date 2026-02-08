# File Exploration

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Commands and UI for browsing files and directories.

## Overview

The editor provides a file explorer panel and command-based file browsing via netrw-style directory buffers.

## File Explorer Panel

| Command | Description |
|---|---|
| `:Explorer` | Toggle file explorer panel |
| `:Explore {path}` | Open directory in explorer |

| Key | Action |
|---|---|
| `<leader>e` | Toggle file explorer |

## Explorer Navigation

| Key | Action |
|---|---|
| `<CR>` | Open file or expand directory |
| `l` | Open / expand |
| `h` | Collapse directory / go to parent |
| `-` | Go to parent directory |
| `q` | Close explorer |
| `/` | Search / filter files |

## File Operations in Explorer

| Key | Action |
|---|---|
| `a` | Create new file |
| `A` | Create new directory |
| `r` | Rename file |
| `d` | Delete file (with confirmation) |
| `p` | Paste (after copy/cut) |
| `y` | Copy file path |

## Tree View

The explorer shows files as a tree with indented directories. Icons are shown for file types when icons are enabled.

| Setting | Default | Description |
|---|---|---|
| `explorer.show_hidden` | `false` | Show hidden (dot) files |
| `explorer.follow` | `true` | Auto-reveal current file in tree |
| `explorer.position` | `left` | Panel position: `left`, `right` |
| `explorer.width` | `30` | Panel width in columns |

## Netrw-Style Directory Listing

When opening a directory path with `:e`, the editor shows a directory listing buffer.

| Key | Action |
|---|---|
| `<CR>` | Open file under cursor |
| `-` | Go to parent |
| `%` | Create new file |
| `d` | Create new directory |
| `R` | Rename |
| `D` | Delete |

## Find File

| Key | Command | Description |
|---|---|---|
| `<leader>ff` | `:FindFile` | Open file finder (fuzzy) |

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
