# File Exploration

Browsing and navigating directories.

## Overview

Built-in file explorer for browsing and opening files.
The explorer is rendered as a special buffer in a
side window (left by default). It supports tree-style
or flat directory listing.

## Opening Explorer

### Commands

| Command | Effect |
|---------|--------|
| `:Explore` or `:Ex` | Open explorer in current window |
| `:Sexplore` or `:Sex` | Open in horizontal split |
| `:Vexplore` or `:Vex` | Open in vertical split (left) |
| `:Texplore` or `:Tex` | Open in new tab |

### Variants

`:Vex!` opens on the right side. All explorer commands
accept an optional path argument: `:Ex /some/path`.

## Explorer Layout

### Display

The explorer buffer shows one entry per line:
directories first (sorted), then files (sorted).
Directories end with `/`. The header line shows
the current directory path.

## Navigation

### Movement

Standard normal-mode motions work: `j`/`k` to move,
`gg`/`G` to jump to top/bottom, `/` to search.

### Quick Access

| Key | Action |
|-----|--------|
| `<CR>` | Open file or enter directory |
| `-` | Go to parent directory |
| `~` | Go to home directory |
| `.` | Toggle hidden files |

## File Operations

### Open

`<CR>` on a file opens it in the window that launched
the explorer. `o` opens in a horizontal split. `v` opens
in a vertical split. `t` opens in a new tab.

### Create

`%` prompts for a new file name, creates it, and opens
it for editing. Intermediate directories are created
if needed.

### Rename/Delete

`R` renames the file/directory under the cursor.
`D` deletes after confirmation prompt.

### Copy/Move

`mc` marks a file for copy, `mm` marks for move.
Navigate to target directory, then `mp` to paste
(copy or move the marked file).

## Hidden Files

### Toggle

`.` toggles display of dotfiles and hidden entries.

### Configuration

`explorer.show_hidden = false` in config TOML sets
the default. The toggle overrides per-session.

## Sorting

### Toggle Sort

`s` cycles through sort methods.

### Methods

| Method | Key |
|--------|-----|
| Name (default) | `s` once |
| Time modified | `s` twice |
| Size | `s` three times |
| Extension | `s` four times |

### Configuration

`explorer.sort = "name"` in config TOML.

## File Icons

### Enable

`explorer.icons = true` in config TOML.

### Requires

A Nerd Font or similar icon font must be installed.
Without it, placeholder characters appear.

## Tree View

### Toggle

`T` toggles between flat list and tree view.
Tree view shows nested directories with indent guides.

### Expansion

In tree view, `<CR>` on a directory expands/collapses it.
`zo` expands, `zc` collapses, `za` toggles.

## Filtering

### Quick Filter

`f` enters filter mode. Type a pattern to filter
the visible entries. Only matching files/dirs are shown.

### Pattern

Glob patterns are supported: `*.rs` shows only Rust files.
Regex is used if the pattern starts with `/`.

### Clear

`<Esc>` in filter mode clears the filter.

## Bookmarks

### Set Bookmark

`b` followed by a letter sets a directory bookmark.

### Jump to Bookmark

`'` followed by the letter jumps to the bookmarked directory.

### Manage

`:ExBookmarks` lists all bookmarks.
