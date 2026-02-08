# Join and Split Operations

Line joining and splitting.

## Overview

Join combines adjacent lines; split breaks a line at the cursor position or wraps text to a specified width.

## Join Operations (normative)

| Key / Command | Action |
|---|---|
| `J` | Join current line with the next; replaces the newline with a single space; removes leading whitespace from the joined line |
| `gJ` | Join without adding a space and without removing leading whitespace |
| `{count}J` | Join `count` lines (current + count-1 following lines) |
| `:[range]join` | Join lines in range with space |
| `:[range]join!` | Join lines in range without space (like `gJ`) |

## J Behavior Details

1. Delete the newline at the end of the current line.
2. If the next line starts with whitespace, remove it.
3. Insert a single space (unless the current line ends with a space, or the next line is empty, or the current line ends with a join-related character like `.` which gets two spaces in some implementations — kjxlkj uses single space always).
4. Cursor is placed at the join point.

## gJ Behavior Details

1. Delete the newline at the end of the current line.
2. No space is inserted.
3. No whitespace is removed from the next line.
4. Cursor is placed at the join point.

## Split Operations

There is no dedicated split key. To split a line:

- Position cursor at the split point and press `r` followed by `Enter` (replace character with newline).
- Or use `i` followed by `Enter` then `Esc`.

## Text Wrapping (gq)

| Key / Command | Action |
|---|---|
| `gq{motion}` | Format (wrap) text within the motion range to `textwidth` |
| `gqq` | Format current line |
| `gqap` | Format current paragraph |
| `:[range]gq` | Format lines in range |

The `textwidth` option (default 0, meaning no wrapping) controls the target line width. When `textwidth` is set, `gq` reflows text by breaking at word boundaries.

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
