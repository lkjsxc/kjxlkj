# Mark Commands

Ex commands for viewing and managing marks.

## Viewing Marks

### :marks

Display all marks with their positions. Without arguments
shows all set marks. With arguments shows only specified
marks: `:marks aB` shows marks `a` and `B`.

### Output Format

| Column | Content |
|--------|---------|
| mark | Mark character (a-z, A-Z, 0-9, special) |
| line | 1-based line number |
| col | 0-based byte column |
| file/text | File path (global marks) or line preview (local) |

## Setting Marks

### Normal Mode

`m{a-zA-Z}` sets a mark at the current cursor position.
Lowercase marks (`a`-`z`) are buffer-local. Uppercase
marks (`A`-`Z`) are global (persist across files).

### Command Mode

`:mark {char}` or `:k{char}` sets a mark at the current line.
Column is set to 0.

## Deleting Marks

### :delmarks

Delete specific marks. Accepts individual characters
and ranges.

### Examples

| Command | Effect |
|---------|--------|
| `:delmarks a` | Delete mark `a` |
| `:delmarks a-d` | Delete marks `a`, `b`, `c`, `d` |
| `:delmarks aB` | Delete marks `a` and `B` |
| `:delmarks!` | Delete all lowercase marks in buffer |

## Mark Operations

### Copy Mark Position

`:let pos = getpos("'a")` returns a list `[bufnum, line, col, off]`.

### Check Mark Exists

`getpos("'a")` returns `[0, 0, 0, 0]` if mark `a` is not set.

## Jump Commands

### Exact Position

`` `{mark} `` jumps to the exact line and column of the mark.

### Line Start

`'{mark}` jumps to the first non-blank character on the
mark's line.

### With Range

`:'a,'b d` deletes from mark `a`'s line to mark `b`'s line.

## marks Command Output

### Columns

| Column | Content |
|--------|---------|
| mark | Mark character |
| line | Line number |
| col | Column number |
| file/text | Filename (global) or line preview (local) |

### Filtering

`:marks a-z` shows only lowercase marks.
`:marks A-Z` shows only uppercase (global) marks.
`:marks <>` shows `<` and `>` (last visual selection).

## Integration Examples

### Delete All Marks in Buffer

`:delmarks!` removes all lowercase marks (`a`-`z`) in
the current buffer. Uppercase and special marks are
not affected.

### Jump to Next Mark

No built-in "next mark" command. A custom mapping can
iterate through marks by checking `getpos()` for each
letter and finding the nearest one after the cursor.

### List Marks in Picker

`:Telescope marks` (if picker integration is configured)
shows marks in the fuzzy finder.

## Mark Range Commands

Marks are valid in ranges for ex commands:
`:'a,'bs/foo/bar/g` substitutes between marks `a` and `b`.
`:'a,'b>` indents the marked range.

## Configuration

Marks are automatically saved in the session file when
`session.marks = true` in config TOML. Global marks
(`A`-`Z`) persist across sessions by default.

## Keybindings

| Key | Action |
|-----|--------|
| `m{a-zA-Z}` | Set mark |
| `` `{mark} `` | Jump to exact position |
| `'{mark}` | Jump to line start |
| `` `[ `` | Start of last change |
| `` `] `` | End of last change |
| `` `< `` | Start of last visual selection |
| `` `> `` | End of last visual selection |
| `''` | Jump to position before last jump |

## API Reference

| Function | Return |
|----------|--------|
| `getpos("'{mark}")` | `[bufnum, line, col, off]` |
| `setpos("'{mark}", list)` | Set mark position |
