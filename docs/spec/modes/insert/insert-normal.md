# Insert-Normal Mode

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Execute one Normal-mode command while in Insert mode, then return automatically.

## Overview

Insert-normal mode allows executing a single normal mode command without fully leaving insert mode. This is useful for repositioning, scrolling, or performing quick edits.

## Command

Press `<C-o>` in insert mode. The mode indicator changes to `(insert)`. After one normal-mode command completes, the editor returns to insert mode.

## Basic Usage

How to use insert-normal mode.

### Movement

| Sequence | Effect |
|---|---|
| `<C-o>w` | Move forward one word |
| `<C-o>b` | Move backward one word |
| `<C-o>$` | Move to end of line |
| `<C-o>0` | Move to start of line |
| `<C-o>gg` | Move to top of file |
| `<C-o>G` | Move to bottom of file |

### Editing

| Sequence | Effect |
|---|---|
| `<C-o>dd` | Delete current line |
| `<C-o>D` | Delete to end of line |
| `<C-o>u` | Undo |
| `<C-o>p` | Paste after cursor |

## With Count

`<C-o>3w` — move forward 3 words, then return to insert mode.

## With Operators

`<C-o>dw` — delete a word, then return to insert mode. The operator-motion pair counts as one command.

## Scrolling

| Sequence | Effect |
|---|---|
| `<C-o>zz` | Center cursor line on screen |
| `<C-o>zt` | Scroll cursor line to top |
| `<C-o>zb` | Scroll cursor line to bottom |
| `<C-o><C-e>` | Scroll down one line |
| `<C-o><C-y>` | Scroll up one line |

## Window Commands

| Sequence | Effect |
|---|---|
| `<C-o><C-w>w` | Switch to next window |
| `<C-o><C-w>h` | Move to left window |
| `<C-o><C-w>+` | Increase window height |

## Search

| Sequence | Effect |
|---|---|
| `<C-o>/pattern<CR>` | Search forward |
| `<C-o>*` | Search for word under cursor |
| `<C-o>#` | Search backward for word under cursor |

## Marks

| Sequence | Effect |
|---|---|
| `<C-o>ma` | Set mark `a` at cursor position |
| `` <C-o>`a `` | Jump to mark `a` |

## Ex Commands

`<C-o>:w<CR>` — save the file and return to insert mode.

The ex command counts as the single normal-mode command.

## Multiple Commands

Only ONE normal-mode command is executed per `<C-o>`. To run multiple, press `<C-o>` again after each returns to insert mode.

## Visual Mode

`<C-o>v` enters visual mode. After completing the visual operation, control returns to insert mode.

## Differences vs Full Exit

| Aspect | `<C-o>` (insert-normal) | `<Esc>` (exit fully) |
|---|---|---|
| Returns to insert | Automatically after 1 command | Must press `i`/`a` again |
| Undo break | Does NOT create an undo break | Creates an undo break |
| Dot repeat | Does not reset `.` context | Resets `.` context |
| Cursor | Stays where command leaves it | Moves left 1 column |

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Insert navigation: [/docs/spec/modes/insert/insert-navigation.md](/docs/spec/modes/insert/insert-navigation.md)
- Normal mode: [/docs/spec/modes/normal/README.md](/docs/spec/modes/normal/README.md)
