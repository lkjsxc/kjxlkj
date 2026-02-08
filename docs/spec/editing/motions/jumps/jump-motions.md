# Jump Motions

Back: [/docs/spec/editing/motions/jumps/README.md](/docs/spec/editing/motions/jumps/README.md)

Motions that add entries to the jump list.

## Overview

Jump motions are motions that move the cursor a large distance or to a different location in the buffer. They record the cursor position in the jump list before moving.

## Motions classified as jumps

| Motion | Description |
|---|---|
| `gg` | Go to first line (or line N with count) |
| `G` | Go to last line (or line N with count) |
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Next search match |
| `N` | Previous search match |
| `*` | Search word under cursor forward |
| `#` | Search word under cursor backward |
| `%` | Go to matching bracket |
| `(` / `)` | Previous/next sentence |
| `{` / `}` | Previous/next paragraph |
| `H` / `M` / `L` | Top/middle/bottom of window |
| `` ` ``mark | Jump to mark position |
| `'`mark | Jump to mark line |
| `Ctrl-o` | Go to older position in jump list |
| `Ctrl-i` | Go to newer position in jump list |
| `:N` | Go to line N |
| `:e {file}` | Open a different file |

## Non-jump motions

The following are NOT jump motions and do NOT update the jump list:

| Motion | Description |
|---|---|
| `h` / `j` / `k` / `l` | Character/line movement |
| `w` / `b` / `e` | Word movement |
| `0` / `$` / `^` | Line boundary movement |
| `f` / `t` / `F` / `T` | Character find |

## Jump list entry

Before executing a jump motion, the current position `(buffer, line, column)` is pushed onto the jump list. This allows returning to the previous position with `Ctrl-o`.

## Cross-file jumps

Jumps that change the current buffer (e.g., `:e`, `gd` to a definition in another file) add the previous buffer position to the jump list. `Ctrl-o` returns to the previous file.

## Count with jumps

Most jump motions accept a count. For example, `3gg` goes to line 3. The count does not affect whether the motion is a jump.

## Related

- Jump list: [/docs/spec/editing/marks/jumplist.md](/docs/spec/editing/marks/jumplist.md)
- Mark motions: [/docs/spec/editing/motions/jumps/mark-motions.md](/docs/spec/editing/motions/jumps/mark-motions.md)
- Motions overview: [/docs/spec/editing/motions/motions.md](/docs/spec/editing/motions/motions.md)
