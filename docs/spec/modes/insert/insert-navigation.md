# Insert Mode Navigation

Moving cursor while in insert mode.

## Overview

Navigate without leaving insert mode for efficient editing.
Insert mode provides limited navigation compared to normal
mode, prioritizing text entry.

## Arrow Keys

### Basic Movement

| Key | Action |
|-----|--------|
| `<Left>` | Move one character left |
| `<Right>` | Move one character right |
| `<Up>` | Move one line up |
| `<Down>` | Move one line down |

### Behavior

Arrow keys work as expected within the buffer bounds.
The cursor stays in insert mode. For CJK characters
(display width 2), left/right moves by one full character
(not half).

## Word Movement

### Shift+Arrow

`<S-Left>` and `<S-Right>` move by word. This extends
the selection in some terminal emulators but in kjxlkj
it simply moves the cursor by word.

### Ctrl+Arrow

`<C-Left>` moves to the beginning of the previous word.
`<C-Right>` moves to the beginning of the next word.
Words are defined by `iskeyword`.

## Line Movement

### Home/End

`<Home>` moves to column 0.
`<End>` moves to the end of the line.

### Ctrl+Home/End

`<C-Home>` moves to the first line of the buffer.
`<C-End>` moves to the last line.
Both keep insert mode active.

## Page Movement

### Page Up/Down

`<PageUp>` scrolls one page up, cursor moves to top.
`<PageDown>` scrolls one page down, cursor moves to bottom.

## Normal Mode Command

### Temporary Normal

`<C-o>` executes one normal mode command, then returns
to insert mode. The statusline shows `-- (insert) --`
during the command.

### Examples

| Sequence | Effect |
|----------|--------|
| `<C-o>zz` | Center screen |
| `<C-o>dd` | Delete current line |
| `<C-o>p` | Paste |
| `<C-o>$` | Move to end of line |

### Return to Insert

After the single command completes, the editor returns
to insert mode automatically. Operators that change mode
(like `c`) may alter this.

## Multiple Commands

### Stay in Insert

`<C-o>` only runs ONE command. For multiple commands,
use `<C-o>` multiple times, or exit insert mode briefly.

### Exit Pattern

`<Esc>` followed by commands, then `a`/`i` to re-enter.
This is often more natural for complex operations.

## Character Deletion

### While Moving

Deletion keys work during navigation:
`<BS>` deletes the character before cursor.
`<Del>` deletes the character after cursor.

### Word Deletion

`<C-w>` deletes the word before the cursor.
`<C-u>` deletes from cursor to start of line.

## Line Operations

### Delete to Start

`<C-u>` deletes from cursor position to the beginning
of the line. If the cursor is at column 0, `<C-u>`
joins with the line above.

### Split Line

Press `<CR>` at any cursor position to split the line.
Text after the cursor moves to a new line below.

## Scrolling

### While in Insert

The viewport can be scrolled without moving the cursor
out of insert mode using `<C-o>` with scroll commands.

### Scroll Commands

| Sequence | Action |
|----------|--------|
| `<C-o><C-e>` | Scroll down one line |
| `<C-o><C-y>` | Scroll up one line |
| `<C-o><C-d>` | Scroll down half page |
| `<C-o><C-u>` | Scroll up half page |
| `<C-o>zz` | Center cursor line |

## Search

### From Insert

`<C-o>/` starts a forward search from insert mode.
After the search completes, returns to insert mode
at the found position.

## Marks

### Jump to Mark

`<C-o>` followed by `` ` `` or `'` and a mark letter
jumps to the mark, then returns to insert mode.

## Window Navigation

### Switch Windows

`<C-o><C-w>w` switches to the next window.
`<C-o><C-w>h/j/k/l` switches directionally.
Insert mode continues in the new window.

## Limited Movement

### Stay on Line

Some users prefer to restrict insert-mode navigation
to the current line only. This is not enforced by default
but can be configured by unmapping arrow keys in insert mode.

## Custom Mappings

### Emacs Style

Common Emacs-like mappings for insert mode:
- `<C-a>` move to beginning of line
- `<C-e>` move to end of line
- `<C-f>` move forward one character
- `<C-b>` move backward one character

These must be explicitly configured as they conflict
with default keybindings.
