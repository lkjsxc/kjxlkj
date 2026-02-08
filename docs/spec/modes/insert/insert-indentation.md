# Insert Indentation

Managing indentation in insert mode.

## Overview

Control indentation while typing with automatic and
manual methods. Indentation behavior depends on three
settings: `autoindent`, `smartindent`, and language-specific
indent expressions.

## Manual Indentation

### Increase Indent

`<C-t>` inserts one `shiftwidth` of indentation at the
beginning of the current line without moving the cursor
position relative to the text.

### Decrease Indent

`<C-d>` removes one `shiftwidth` of indentation from the
beginning of the current line.

### Remove All Indent

`0<C-d>` removes all indentation from the current line.
The `0` is consumed by the indent command, not inserted.

### Restore Indent

`^<C-d>` temporarily removes indent for the current line
and restores it on the next line (useful for labels and
preprocessor directives).

## Auto-Indent

### Behavior

When pressing Enter in insert mode:
- The new line inherits the indentation of the previous line
- If `smartindent` is active, indent may increase for
  block-opening keywords (`{`, `if`, `def`, `fn`)
- If `indentexpr` is set, it overrides `smartindent`

### Configuration

`autoindent = true` (default: `true`). Copies indent from
the current line to the new line on `<CR>`.

## Smart Indent

### Code Awareness

`smartindent = true` adds one `shiftwidth` after lines
ending with `{` and removes one after `}` at line start.
Works for C-like languages.

### Keywords

Smart indent increases after: `{`, `if`, `else`, `while`,
`for`, `do`, `switch`, `case`. Decreases at `}`, `end`.
Language-specific keywords are configured via `indentexpr`.

## Indent Settings

### Tab Width

`tabstop` (integer): display width of a `\t` character.
Default: 8. Common settings: 2, 4, 8.

### Expand Tabs

`expandtab` (bool): when `true`, pressing `<Tab>` inserts
spaces instead of a tab character. Default: `false`.

## Tab Key Behavior

### Standard

When `expandtab = false`, `<Tab>` inserts a literal tab
character (`\t`).

### With expandtab

When `expandtab = true`, `<Tab>` inserts `shiftwidth` spaces
(or enough spaces to reach the next `shiftwidth` boundary).

### Literal Tab

`<C-v><Tab>` always inserts a literal tab character,
regardless of `expandtab` setting.

## Indentation Commands

### From Normal Mode

`>>` / `<<` shift right/left by `shiftwidth`.
`==` re-indents based on `indentexpr`.

### In Insert Mode

`<C-t>` / `<C-d>` are the insert-mode equivalents.

## Entering Insert

### Indentation Modes

| Command | Indent Behavior |
|---------|----------------|
| `o` | Open below, auto-indent the new line |
| `O` | Open above, auto-indent the new line |
| `cc` / `S` | Clear line, preserve indent |
| `i` / `a` | No indent change |
| `I` | Cursor at first non-blank |

### o/O Behavior

`o` opens a new line below and applies auto-indent rules.
If the current line ends with `{`, the new line is indented
one level deeper.

## Auto-Indent Triggers

### After Keywords

Language-specific keywords trigger increased indent.
For Rust: `fn`, `if`, `else`, `match`, `loop`, `for`,
`while`, `impl`, `struct`, `enum`.

### After Braces

Opening braces `{`, `(`, `[` increase indent.
Closing braces `}`, `)`, `]` decrease indent.

## File Type Settings

### Per Language

Indent settings are configurable per filetype under
`[languages.{lang}]` in TOML. Common settings:
- Rust: `indent_unit = "    "` (4 spaces)
- Python: `indent_unit = "    "` (4 spaces)
- JavaScript: `indent_unit = "  "` (2 spaces)
- Go: `expandtab = false`, `tabstop = 4`

## Paste Indentation

### Problem

Pasting text in insert mode with `autoindent` active
can produce cascading indentation since each pasted
newline triggers auto-indent.

### Solution

`:set paste` before pasting disables auto-indent
temporarily. `:set nopaste` restores. Alternatively,
use `<C-r>+` or `p`/`P` from normal mode.

## Indent While Typing

### Break Line

When `<CR>` is pressed in the middle of a line, the new
line receives auto-indent AND the text after the cursor
moves to the new line at the indented position.
