# Insert Normal Mode

Execute normal commands from insert.

## Overview

Run one normal mode command without fully leaving insert
mode. The `<C-o>` key enters a temporary normal mode
that auto-returns to insert after one command.

## Command

### Syntax

`<C-o>{normal-command}` in insert mode.

### Behavior

1. Enter temporary normal mode (statusline: `-- (insert) --`)
2. Execute exactly one normal mode command
3. Automatically return to insert mode

## Basic Usage

### Movement

| Sequence | Effect |
|----------|--------|
| `<C-o>w` | Move forward one word |
| `<C-o>b` | Move backward one word |
| `<C-o>0` | Move to beginning of line |
| `<C-o>$` | Move to end of line |
| `<C-o>gg` | Move to first line |
| `<C-o>G` | Move to last line |

### Editing

| Sequence | Effect |
|----------|--------|
| `<C-o>dd` | Delete current line |
| `<C-o>D` | Delete to end of line |
| `<C-o>p` | Paste after cursor |
| `<C-o>u` | Undo last change |
| `<C-o><C-r>` | Redo |

## With Count

### Counted Commands

`<C-o>3w` moves 3 words forward.
`<C-o>5j` moves 5 lines down.
The count is part of the single command.

## With Operators

### Operator + Motion

`<C-o>dw` deletes a word. The operator+motion pair
counts as one command.

### Returns to Insert

After the operator+motion completes, insert mode resumes.
If the operator opens insert mode (like `c`), the behavior
depends: `<C-o>cw` changes a word and stays in insert mode
(the `c` operator implies insert mode entry).

## Scrolling

### Center/Top/Bottom

| Sequence | Effect |
|----------|--------|
| `<C-o>zz` | Center current line |
| `<C-o>zt` | Current line to top |
| `<C-o>zb` | Current line to bottom |

### Scroll Lines

| Sequence | Effect |
|----------|--------|
| `<C-o><C-e>` | Scroll down one line |
| `<C-o><C-y>` | Scroll up one line |
| `<C-o><C-d>` | Scroll down half page |
| `<C-o><C-u>` | Scroll up half page |

## Window Commands

### Switch Windows

`<C-o><C-w>w` cycles to the next window.
`<C-o><C-w>h/j/k/l` navigates directionally.
Insert mode continues in the target window.

### Window Size

`<C-o><C-w>+` increases window height.
`<C-o><C-w>-` decreases window height.

## Search

### Quick Search

`<C-o>/pattern<CR>` searches forward and returns to insert.
`<C-o>*` searches for word under cursor.

### Word Under Cursor

`<C-o>*` is useful for checking other occurrences of the
word being typed.

## Marks

### Jump to Mark

`<C-o>'a` jumps to mark `a`'s line, returns to insert.

### Set Mark

`<C-o>ma` sets mark `a` at current position, returns to insert.

## Ex Commands

### Run Command

`<C-o>:cmd<CR>` runs an ex command from insert mode.

### Stay in Insert

After the ex command completes, returns to insert mode.
This is useful for `:w` (save) without leaving insert.

## Multiple Commands

### Not Directly

`<C-o>` runs ONE command only. It does not accept
sequences of multiple commands.

### Workarounds

Exit to normal mode, perform multiple commands, re-enter
insert mode with `a` or `i`.

### Macro

`<C-o>@a` executes macro `a` as one command. The macro
can contain multiple commands.

## Visual Mode

### Quick Select

`<C-o>v` enters visual mode. However, visual mode
commands typically end in normal mode, not insert.

Note: Visual mode exits to normal mode, not insert mode.

### Workaround

Use `<C-o>` with an operator+text-object instead:
`<C-o>diw` deletes inner word and returns to insert.

## Differences

### vs Full Exit

`<C-o>{cmd}` preserves the insert session: undo groups
continue, and the `.` repeat register includes the
full insert session. `<Esc>{cmd}i` starts a new insert
session and creates a new undo point.
