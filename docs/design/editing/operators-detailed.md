# Operators

Commands that operate on text.

## Overview

Operators work with motions and text objects using the grammar:
`{operator}[count]{motion}` or `{operator}[count]{text-object}`.
Counts on operator and motion multiply: `2d3w` deletes 6 words.
In visual mode, the operator acts on the selection: select text then press `d`.

## Core Operators

| Operator | Description |
|----------|-------------|
| `d` | Delete |
| `c` | Change |
| `y` | Yank (copy) |
| `>` | Indent right |
| `<` | Indent left |
| `=` | Auto-indent |
| `gq` | Format |
| `gu` | Lowercase |
| `gU` | Uppercase |
| `g~` | Toggle case |

## Delete

### Examples

- `dw` -- delete from cursor to next word start
- `d$` -- delete from cursor to end of line
- `d3j` -- delete current line and 3 lines below (linewise)
- `dip` -- delete inner paragraph
- `"add` -- delete line into register `a`

### Registers

Deleted text goes to default register (`""`). Small deletes (less than one line)
also go to `"-`. Use `"xd{motion}` to delete into register `x`.
Capital `"Ad{motion}` appends to register `a`.

## Change

### Examples

- `cw` -- delete to word end and enter insert mode
- `cc` -- change entire line (clear content, keep indent)
- `ci"` -- change inner double-quoted string
- `c$` or `C` -- change from cursor to end of line
- `c2e` -- change through 2 word ends

### Enters Insert Mode

After change, you're in insert mode.

## Yank

### Examples

- `yw` -- yank from cursor to next word start
- `yy` or `Y` -- yank current line
- `y3j` -- yank current line plus 3 below (linewise)
- `yi(` -- yank text inside parentheses

### Registers

Yanked text goes to default register (`""`) and yank register (`"0`).
Use `"ayy` to yank current line into register `a`.
`"Ayy` appends the line to register `a` instead of replacing.
System clipboard: `"+y{motion}` (clipboard) or `"*y{motion}` (primary selection).

## Indent

### Examples

- `>>` -- indent current line by `shiftwidth`
- `>3j` -- indent current line and 3 below
- `>ip` -- indent inner paragraph
- `<ap` -- dedent around paragraph

### Configuration

Indent size is controlled by `shiftwidth` (default 4). `shiftround` snaps
indentation to the nearest multiple of `shiftwidth` when enabled.

## Format

### Examples

- `gqap` -- reflow paragraph to `textwidth`
- `gqq` -- format current line
- `gq3j` -- format current + 3 lines below

## Case Operators

### Examples

- `gUw` -- uppercase next word
- `guw` -- lowercase next word
- `g~w` -- toggle case of next word
- `gUU` -- uppercase entire line
- `guu` -- lowercase entire line
- `g~~` -- toggle case of entire line

## Doubled Operators

Line-wise operation:

| Operator | Line Version |
|----------|--------------|
| `d` | `dd` |
| `c` | `cc` |
| `y` | `yy` |
| `>` | `>>` |
| `<` | `<<` |
| `=` | `==` |

## With Counts

Counts before the operator repeat the operation. Counts can also appear before
the motion; they multiply. Examples:

| Input | Effect |
|-------|--------|
| `3dd` | Delete 3 lines |
| `2yy` | Yank 2 lines |
| `5>>` | Indent 5 lines |
| `2d3w` | Delete 6 words (2 x 3) |
| `3gUw` | Uppercase 3 words |

## Visual Mode

### Apply to Selection

In visual mode, select text first then press the operator key:
- `v3wd` -- select 3 words then delete
- `Vjjy` -- select 3 lines then yank
- `viB>` -- select inner `{}` block then indent
- `gUV` is equivalent to selecting the line in visual then pressing `gU`

Visual mode operators are always single-key (no motion needed).

## Operator-Pending Mode

After pressing operator:
- Cursor blinks differently
- Waiting for motion/object
- `<Esc>` cancels

## Custom Operators

### Configuration

Custom operators are not supported in the initial implementation. Future
extension: allow plugins to register operators via `register_operator(key,
callback)` where callback receives the selected range and buffer reference.

## Repeat

### Dot Operator

`.` repeats last operator + motion/object.

- `dw` then `.` -- repeats `dw` (delete next word again)
- `ci"hello<Esc>` then `.` -- changes next inner-quoted string to "hello"
- Counts with dot: `3.` repeats the last change 3 times
- Register changes are not part of the repeat; `.` always uses default register

See also: [/docs/design/editing/motions-detailed.md](/docs/design/editing/motions-detailed.md),
[/docs/design/editing/text-objects-detailed.md](/docs/design/editing/text-objects-detailed.md)