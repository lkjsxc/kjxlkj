# Mark Motions

Navigate using marks.

## Overview

Marks save cursor positions. Two motion families restore them: backtick (exact position) and single-quote (line start).

## Setting Marks

`m{x}` sets mark `{x}` at the current cursor position.

| Mark range | Scope | Persisted |
|---|---|---|
| `a`-`z` | Current buffer only | In session file |
| `A`-`Z` | Global (any buffer) | In session file, includes file path |

## Mark Motion Commands (normative)

| Key | Motion type | Destination |
|---|---|---|
| `` `{x} `` | Exact | Line and column of mark `{x}` |
| `'{x}` | Linewise | First non-blank of the line of mark `{x}` |
| ``` `` ``` | Exact | Position before last jump |
| `''` | Linewise | Line before last jump |

Both motion types are exclusive when used as movements, and can serve as operator targets (e.g., `` d`a `` deletes from cursor to mark `a`).

## Special Automatic Marks (normative)

| Mark | Set when | Position |
|---|---|---|
| `` ` `` (backtick) | Any jump command | Position before the jump |
| `'` (single-quote) | Any jump command | Line of position before jump |
| `"` | Leaving a buffer | Last cursor position in buffer |
| `[` | After a change/yank | Start of changed/yanked text |
| `]` | After a change/yank | End of changed/yanked text |
| `<` | After visual selection | Start of visual area |
| `>` | After visual selection | End of visual area |
| `.` | After a change | Position of last change |
| `^` | After insert mode exit | Last insert-mode cursor position |

## Operator + Mark Interaction

Marks define the other end of an operator range.

| Example | Effect |
|---|---|
| `` d`a `` | Delete from cursor to exact position of mark `a` (exclusive) |
| `d'a` | Delete from current line to line of mark `a` (linewise) |
| `` y`a `` | Yank from cursor to mark `a` (exclusive) |
| `c'a` | Change from current line to line of mark `a` (linewise) |

If the mark is in a different buffer (global mark A-Z), the command switches to that buffer first.

## Visual Selection with Marks

`` v`a `` selects from cursor to mark `a`. `v'a` selects linewise to the mark line.

## Commands (normative)

| Command | Action |
|---|---|
| `:marks` | List all marks with line, column, and text |
| `:marks {args}` | List only specified marks |
| `:delmarks {args}` | Delete specified marks (e.g., `:delmarks a b c`) |
| `:delmarks!` | Delete all lowercase marks for the current buffer |

## Jump List Relationship

Mark motions (`` ` `` and `'`) are jump commands: they add the pre-jump position to the jump list. Navigation within a buffer using marks is thus recoverable with `Ctrl-o` / `Ctrl-i`.

## Error Cases

| Condition | Behavior |
|---|---|
| Mark not set | Error: "E20: Mark not set" |
| Mark in deleted line | Error: "Mark not set" (invalidated) |
| Global mark in closed file | Open the file, then jump |

## Related

- Marks overview: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Jump list: [/docs/spec/editing/motions/jumps/README.md](/docs/spec/editing/motions/jumps/README.md)
