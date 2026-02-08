# g-Prefixed Operators

Back: [docs/spec/editing/operators/README.md](docs/spec/editing/operators/README.md)

Operators and commands accessed via the `g` prefix.

## Overview

The `g` prefix provides access to additional operators
and commands that extend the standard operator set.
These cover case conversion, formatting, cursor
movement, and display-line operations.

## Case Operators

### gu - Lowercase

`gu{motion}` converts text covered by motion to
lowercase. `guu` lowercases the entire line.

### gU - Uppercase

`gU{motion}` converts text to uppercase. `gUU`
uppercases the entire line.

### g~ - Toggle Case

`g~{motion}` toggles case of each character. `g~~`
toggles case of the entire line.

### Tilde Operator

`~` toggles case of the character under cursor and
advances. With `tildeop` set, `~` acts as an operator
requiring a motion (like `g~`).

## Formatting Operator

### gq - Format Text

`gq{motion}` formats text to `textwidth` columns.
`gqq` formats the current line. `gqap` formats
the current paragraph.

### gw - Format Without Cursor Move

`gw{motion}` formats like `gq` but keeps the cursor
at its original position.

### Format Rules

- Lines are broken at word boundaries
- Existing line breaks within paragraphs are removed
- Paragraphs are separated by blank lines
- List items (starting with `-`, `*`, etc.) preserve
  their indentation
- Code blocks are not reformatted

## Display Line Operations

### gj / gk - Display Line Movement

| Key | Action |
|-----|--------|
| `gj` | Down one display line |
| `gk` | Up one display line |
| `g0` | Start of display line |
| `g^` | First non-blank of display line |
| `g$` | End of display line |
| `gm` | Middle of display line |

These differ from `j`/`k` when `wrap` is enabled
and long lines span multiple display rows.

## Information Commands

### ga - Character Info

`ga` displays the character under the cursor:
- Character representation
- Decimal code point
- Hex code point
- Octal code point
- Digraph codes (if any)

### g8 - UTF-8 Bytes

`g8` shows the raw UTF-8 byte sequence of the
character under the cursor.

### gCtrl-g - Cursor Position

`g<Ctrl-g>` shows detailed cursor position:
column, line, word count, character count, byte
offset.

## Go-To Commands

### gg - Go to Line

`gg` goes to the first line. `{N}gg` goes to
line N. This is equivalent to `{N}G`.

### gd - Go to Definition

`gd` goes to local declaration of the word under
cursor. Searches from the start of the current
function/block.

### gD - Go to Global Definition

`gD` goes to global declaration. Searches from
the start of the file.

### gf - Go to File

`gf` opens the file whose path is under the cursor.
`gF` opens the file and jumps to the line number
after the filename (e.g., `file.rs:42`).

## Join and Put

### gJ - Join Without Space

`gJ` joins lines without inserting a space between
them. Unlike `J`, it does not add any separator.

### gp / gP - Put After/Before with Cursor

`gp` puts text after cursor and moves cursor after
the pasted text. `gP` puts before and moves cursor.
(Standard `p`/`P` leave cursor at start of paste.)

## Visual Commands

### gv - Reselect Visual

`gv` reselects the last visual selection. The
selection area is restored exactly as it was.

### gn / gN - Search and Select

`gn` searches forward for the last search pattern
and visually selects the match. `gN` searches
backward. These can be used with operators:
`dgn` deletes the next search match.

## Related

- Operators overview: [docs/spec/editing/operators/README.md](docs/spec/editing/operators/README.md)
- Motions: [docs/spec/editing/motions/README.md](docs/spec/editing/motions/README.md)
- Line motions: [docs/spec/editing/motions/line-motions.md](docs/spec/editing/motions/line-motions.md)
