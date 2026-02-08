# Buffer-Local Options

Options scoped to individual buffers.

## Overview

Buffer-local options apply only to a specific buffer.
Each buffer maintains its own copy of these options
independent of global defaults.

## Option Scope

### Global Options

Global options apply everywhere: `hlsearch`, `ignorecase`.
Setting them changes behavior across all buffers.

### Buffer-Local Options

Buffer-local options include: `filetype`, `tabstop`,
`shiftwidth`, `expandtab`, `textwidth`, `formatoptions`,
`syntax`, `spelllang`, `fileencoding`, `fileformat`,
`modifiable`, `readonly`, `undofile`, `swapfile`.

### Window-Local Options

Window-local options: `number`, `relativenumber`,
`signcolumn`, `foldmethod`, `wrap`, `linebreak`,
`cursorline`, `cursorcolumn`, `scrolloff`, `colorcolumn`.

## Setting Commands

### Set for Current Buffer

`:setlocal tabstop=4` sets tabstop for the current
buffer only. Other buffers keep their own value.

### Check Current Value

`:setlocal tabstop?` shows the buffer-local value.
`:set tabstop?` shows the effective value.

### Reset to Global

`:setlocal tabstop<` resets to the global default.

## Filetype Detection

### Automatic

When a file is opened, its filetype is detected from
extension, shebang line, and content inspection.

### Configuration

Filetype-specific options are set in language config:
`[languages.rust]` section with `tabstop = 4`, etc.

## Common Buffer-Local Options

| Option | Type | Description |
|--------|------|-------------|
| `filetype` | string | Detected file type |
| `tabstop` | number | Tab display width |
| `shiftwidth` | number | Indent step size |
| `expandtab` | bool | Use spaces for tab |
| `textwidth` | number | Line wrap column (0=off) |
| `fileencoding` | string | File encoding |
| `fileformat` | string | Line ending style |
| `modifiable` | bool | Allow buffer modification |
| `readonly` | bool | Read-only flag |
| `undolevels` | number | Max undo levels |

## Modeline

### Syntax

The first or last 5 lines of a file can contain a
modeline: `// vim: tabstop=4 shiftwidth=4 expandtab`

### Security

Modelines are parsed but sandboxed. Only safe options
can be set via modelines. `modeline = true` enables
modeline parsing (default: true).

## Interaction

### Buffer Switch

When switching buffers, the editor applies the target
buffer's local options automatically.

### New Buffer

New buffers inherit the global defaults until modified
by filetype detection or explicit `:setlocal` commands.
