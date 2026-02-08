# Motion Grammar

The composable grammar for Vim-style operators and motions.

## Grammar Structure (normative)

The fundamental editing grammar is:

`[count] operator [count] motion`

or with text objects:

`[count] operator [count] text-object`

When both counts are present they multiply: `2d3w` deletes 6 words.

## Components

| Component | Role | Examples |
|---|---|---|
| Count | Repetition multiplier (default 1) | `3`, `12`, `100` |
| Operator | Action to apply | `d`, `y`, `c`, `>`, `<`, `=`, `gq`, `g~`, `gu`, `gU` |
| Motion | Cursor movement defining range | `w`, `e`, `b`, `$`, `}`, `f{char}` |
| Text object | Structural selection (operator-pending only) | `iw`, `a"`, `ip`, `at` |

## Operator-Motion Combination

The operator defines *what* to do; the motion defines *where*. The text between the cursor start position and the motion destination is the operated region.

| Input | Operator | Motion | Effect |
|---|---|---|---|
| `dw` | delete | word forward | Delete from cursor to next word start |
| `y$` | yank | end of line | Yank to end of line |
| `c}` | change | paragraph end | Change to end of paragraph |
| `>j` | indent | line down | Indent current + next line |
| `gUiw` | uppercase | inner word | Uppercase the word under cursor |

## Doubled Operators (normative)

An operator followed by itself operates on the current line:

| Input | Effect |
|---|---|
| `dd` | Delete current line |
| `yy` | Yank current line |
| `cc` | Change current line |
| `>>` | Indent current line |
| `<<` | Unindent current line |
| `==` | Auto-indent current line |
| `gqq` | Format current line |
| `gUU` | Uppercase current line |
| `guu` | Lowercase current line |
| `g~~` | Toggle case on current line |

With count `3dd` deletes 3 lines starting from cursor line.

## Count Placement

| Input | Meaning |
|---|---|
| `3dw` | Delete 3 words (count before operator) |
| `d3w` | Delete 3 words (count before motion) |
| `2d3w` | Delete 6 words (counts multiply) |

## Motion Types and Operator Scope

| Motion type | Operator scope |
|---|---|
| Characterwise exclusive | Start to destination (exclusive) |
| Characterwise inclusive | Start to destination (inclusive) |
| Linewise | Full lines from start line to destination line |
| Blockwise | Rectangular block selection |

See [/docs/spec/editing/operators/exclusive-inclusive.md](/docs/spec/editing/operators/exclusive-inclusive.md) for inclusive/exclusive details.

## Text Objects in Operator-Pending

Text objects are only valid after an operator (in operator-pending mode). They define a region around the cursor without moving it.

| Prefix | Meaning |
|---|---|
| `i` | Inner — content between delimiters |
| `a` | Around — content including delimiters |

## Standalone Motions

Without an operator prefix, a motion simply moves the cursor.

## Visual Mode Grammar

In visual mode, motions extend the selection instead of defining an operator range. Operators in visual mode act on the selected region:

`v{motion}...{operator}` — select a region then operate.

## Command-Line Ranges

Ex commands use a different grammar: `:{range}command`. Ranges specify line numbers or patterns. See [/docs/spec/commands/ranges/README.md](/docs/spec/commands/ranges/README.md).

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Motions: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
