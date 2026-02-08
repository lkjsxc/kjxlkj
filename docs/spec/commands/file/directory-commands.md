# Directory Commands

Working directory management.

## Overview

Commands for changing and managing working directories.
The editor supports global, window-local, and tab-local
working directories, forming a hierarchy.

## Current Directory

### Check

`:pwd` displays the current working directory.

### Output

Prints the effective working directory for the current
window. If a window-local directory is set, it shows that;
otherwise the global directory.

## Change Directory

### Global

`:cd {path}` changes the global working directory.
Affects all windows that do not have a local directory set.

### Examples

| Command | Effect |
|---------|--------|
| `:cd /home/user/project` | Absolute path |
| `:cd src` | Relative to current dir |
| `:cd -` | Previous directory |
| `:cd` | Home directory |
| `:cd %:h` | Directory of current file |

## Local Directory

### Window Local

`:lcd {path}` sets a window-local working directory.
Only the current window is affected.

### Use Case

Each split can have its own working directory, useful for
working on multiple projects simultaneously.

### Check

`:pwd` shows the effective directory. Window-local
takes precedence over global.

## Tab Directory

### Tab Local

`:tcd {path}` sets a tab-local working directory.
All windows in the tab inherit this unless they have
their own `:lcd`.

### Hierarchy

Priority order (highest first):
1. Window-local (`:lcd`)
2. Tab-local (`:tcd`)
3. Global (`:cd`)

## Home Directory

### Shortcuts

`~` expands to `$HOME` in all path arguments.
`:cd ~` and `:cd` (no args) both go to home.

## Previous Directory

### Return

`:cd -` returns to the previous working directory.
The editor maintains a stack of one previous directory
per scope (global, tab, window).

### Stack

Only one level of previous directory is remembered
per scope. Repeated `:cd -` toggles between two dirs.

## Path Expansion

### Variables

| Variable | Expansion |
|----------|-----------|
| `%` | Current file path |
| `%:h` | Current file directory |
| `%:t` | Current file name (tail) |
| `%:r` | Current file without extension |
| `%:e` | Current file extension |
| `#` | Alternate file path |

### Modifiers

Modifiers chain: `%:p:h` gives the absolute directory
of the current file.

## Auto Directory

### Follow Files

When `autochdir = true`, the working directory
automatically changes to the directory of the current
file whenever you switch buffers or windows.

### On Open

`:e /some/path/file.rs` with `autochdir` changes
the working directory to `/some/path/`.

## Project Root

### Jump to Root

`:cd` with no arguments goes to home. For project root:
`:ProjectRoot` (built-in command) searches upward for
root markers.

### Detection

Root markers checked in order:
`.git`, `.hg`, `Cargo.toml`, `package.json`,
`pyproject.toml`, `.kjxlkj.toml`.
Configurable via `project.root_markers` in config TOML.

## Directory Listing

### List Contents

`:!ls` runs `ls` in the current directory and shows output.

### In Buffer

`:r !ls` reads directory listing into the current buffer.

## Make Directory

### Create

`:!mkdir -p {path}` creates directories.
The `%` prompt for `:Explore` also creates directories.

## Remove Directory

### Delete Empty

`:!rmdir {path}` removes empty directories.

### Delete Contents

`:!rm -rf {path}` removes recursively. This is destructive
and irreversible. No built-in safe-delete command;
users must use shell commands.

## File System Navigation

### Explore

`:Ex {path}` opens the file explorer at the specified path.
Without argument, opens at the current directory.

## Directory Stack

### Manual Stack

Not built-in. Can be scripted via user commands that
maintain a list of directories and provide push/pop.
