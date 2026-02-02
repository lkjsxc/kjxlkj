# Registers

kjxlkj provides vim-style registers for text storage.

## Overview

Registers store yanked, deleted, and other text.

## Named Registers

### Lowercase (a-z)


Examples:
- `"ayw` - Yank word to register 'a'
- `"ap` - Paste from register 'a'
- `"bdd` - Delete line to register 'b'

### Uppercase (A-Z)

Append to register (instead of replace):

- `"Ayw` - Append word to register 'a'

## Special Registers

| Register | Description |
|----------|-------------|
| `"` | Unnamed (default) |
| `0` | Last yank |
| `1-9` | Delete history |
| `+` | System clipboard |
| `*` | Primary selection (X11) |
| `_` | Black hole (discard) |
| `/` | Last search pattern |
| `:` | Last command |
| `.` | Last inserted text |
| `%` | Current filename |
| `#` | Alternate filename |
| `=` | Expression register |

## Clipboard Integration

### System Clipboard


Usage:
- `"+y` - Yank to clipboard
- `"+p` - Paste from clipboard

### Auto-sync


All yanks automatically go to clipboard.

## Viewing Registers


## Expression Register

Insert result of expression:

In insert mode: `<C-r>=`
Then type expression: `2+2<Enter>`
Inserts: `4`


## Configuration


## Keybindings


## Black Hole Register

Delete without affecting registers:

- `"_dd` - Delete line, don't store

Useful when replacing text without losing clipboard.

## Insert Mode Access


- `<C-r>a` - Insert register 'a'
- `<C-r>+` - Insert clipboard
- `<C-r>"` - Insert unnamed register

## Numbered Registers

Automatic delete history:

- `"1` - Most recent delete
- `"2` - Previous delete
- ... up to `"9`

Yanks go to `"0`, not numbered registers.

## Commands

| Command | Description |
|---------|-------------|
| `:let @a = "text"` | Set register |
| `:put a` | Put register on new line |
| `:@:` | Repeat last command |
