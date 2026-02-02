# Ex Commands

Vim-compatible command-line commands.

## Overview

Ex commands are entered after pressing `:` and
provide powerful text manipulation.

## Navigation

| Command | Description |
|---------|-------------|
| `:e file` | Edit file |
| `:w` | Write (save) |
| `:q` | Quit |
| `:wq` | Write and quit |
| `:q!` | Quit without saving |
| `:{n}` | Go to line n |

## File Operations

| Command | Description |
|---------|-------------|
| `:e file` | Edit file |
| `:w` | Save |
| `:w file` | Save as |
| `:wa` | Save all |
| `:sav file` | Save as and switch |

## Buffer Commands

| Command | Description |
|---------|-------------|
| `:bn` | Next buffer |
| `:bp` | Previous buffer |
| `:bd` | Delete buffer |
| `:b{n}` | Go to buffer n |
| `:buffers` | List buffers |

## Window Commands

| Command | Description |
|---------|-------------|
| `:sp` | Horizontal split |
| `:vs` | Vertical split |
| `:close` | Close window |
| `:only` | Close other windows |

## Search and Replace

### Pattern


### Flags

| Flag | Meaning |
|------|---------|
| `g` | Global (all matches) |
| `c` | Confirm each |
| `i` | Case insensitive |
| `n` | Count only |

### Examples


## Range

### Syntax


### Specifiers

| Range | Meaning |
|-------|---------|
| `.` | Current line |
| `$` | Last line |
| `%` | Entire file |
| `'<,'>` | Visual selection |
| `+n` | n lines below |
| `-n` | n lines above |

### Examples


## Global Command

### Syntax


### Examples


## Sorting


## External Commands


## Settings


## Custom Commands


## Help

