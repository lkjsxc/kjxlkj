# Insert Mode Navigation

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Navigation commands available while remaining in insert mode.

## Overview

In insert mode, the cursor can be repositioned using arrow keys, Home/End, Page Up/Down, and special key combinations without leaving insert mode.

## Arrow Keys

| Key | Movement |
|---|---|
| `<Left>` | One character left |
| `<Right>` | One character right |
| `<Up>` | One line up |
| `<Down>` | One line down |

Arrow key movement respects virtual-edit settings and CJK character boundaries (skip full grapheme cluster).

## Word Movement

| Key | Movement |
|---|---|
| `<S-Left>` | One word backward |
| `<S-Right>` | One word forward |
| `<C-Left>` | One WORD backward |
| `<C-Right>` | One WORD forward |

Word boundaries use the same definition as normal mode `w`/`b`.

## Line Movement

| Key | Movement |
|---|---|
| `<Home>` | Move to first non-blank character of line |
| `<End>` | Move to end of line |
| `<C-Home>` | Move to first line of buffer |
| `<C-End>` | Move to last line of buffer |

## Page Movement

| Key | Movement |
|---|---|
| `<PageUp>` | Scroll one page up |
| `<PageDown>` | Scroll one page down |

Cursor repositions to keep visible after page scroll.

## Normal Mode Command

`<C-o>` enters insert-normal mode: executes one normal-mode command, then returns to insert mode.

See: [/docs/spec/modes/insert/insert-normal.md](/docs/spec/modes/insert/insert-normal.md)

## Character Deletion

| Key | Effect |
|---|---|
| `<BS>` / `<C-h>` | Delete character before cursor |
| `<Del>` | Delete character under cursor |
| `<C-w>` | Delete word before cursor |

## Line Operations

| Key | Effect |
|---|---|
| `<C-u>` | Delete from cursor to start of line |
| `<CR>` / `<C-m>` | Insert new line (split at cursor) |
| `<C-j>` | Insert new line (same as `<CR>`) |

## Scrolling

| Key | Effect |
|---|---|
| `<C-x><C-e>` | Scroll window down one line |
| `<C-x><C-y>` | Scroll window up one line |

Cursor stays on the same buffer position; the viewport shifts.

## Search

`<C-o>` followed by `/` or `?` performs a single search command and returns to insert mode.

## Marks

`<C-o>` followed by a mark command (`` ` `` or `'`) jumps to the mark, then returns to insert mode.

## Window Navigation

Use `<C-o><C-w>{cmd}` to issue a single window command from insert mode.

| Sequence | Effect |
|---|---|
| `<C-o><C-w>w` | Switch to next window |
| `<C-o><C-w>h` | Switch to window left |

## Custom Mappings

Insert-mode navigation can be extended with mappings. Example Emacs-style:

| Mapping | From | To |
|---|---|---|
| `inoremap <C-a>` | `<Home>` | Start of line |
| `inoremap <C-e>` | `<End>` | End of line |
| `inoremap <C-f>` | `<Right>` | Forward one character |
| `inoremap <C-b>` | `<Left>` | Backward one character |

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Insert-normal: [/docs/spec/modes/insert/insert-normal.md](/docs/spec/modes/insert/insert-normal.md)
- Cursor: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
