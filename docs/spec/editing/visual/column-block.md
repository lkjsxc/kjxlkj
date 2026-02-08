# Column/Block Editing

Visual block mode operations.

## Overview

Visual block mode (`Ctrl-v`) selects a rectangular region of text. Operations on this block apply to each line independently, enabling column-oriented editing.

## Entering Block Mode (normative)

| Key | Action |
|---|---|
| `Ctrl-v` | Enter visual block mode from Normal |
| `Ctrl-v` (from visual) | Switch to block mode |

Extend selection with standard motions (`h`, `j`, `k`, `l`, `w`, `$`, etc.). `$` extends to end of each line independently (ragged right edge).

## Block Operations (normative)

| Key | Action |
|---|---|
| `I` | Insert text before the block on every line; text is typed once, then applied to all lines on `Esc` |
| `A` | Append text after the block on every line |
| `c` or `s` | Change: delete block content and enter insert mode; replacement applied to all lines |
| `d` or `x` | Delete the block content |
| `r{char}` | Replace every character in the block with `{char}` |
| `y` | Yank the block (blockwise register) |
| `>` / `<` | Indent/unindent selected lines |
| `~` | Toggle case of block content |
| `U` | Uppercase block content |
| `u` | Lowercase block content |

## Block Insert (I) Behavior

1. User presses `I` in block mode.
2. Cursor moves to the top-left corner of the block.
3. User types text (appears on the first line only during editing).
4. On `Esc`, the same text is inserted at the beginning of the block column on every line in the selection.
5. Lines shorter than the block's left column are not modified.

## Block Append (A) Behavior

Same as block insert, but text is added after the right edge of the block. If `$` was used to extend to end-of-line, text is appended at the end of each line.

## Block Yank and Put

- `y` in block mode stores text as blockwise in the register.
- `p` with a blockwise register pastes the block starting at the cursor position, with each stored line placed on successive screen lines at the same column.
- If the pasted block has fewer lines than the target, only those lines are affected.

## Column Number Sequences

`g Ctrl-a` in visual block mode creates incrementing number sequences. Select a column of numbers (or zeros), then `g Ctrl-a` increments each successive line by 1.

## Related

- Visual mode: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
