# Mark Navigation

Moving around the buffer using marks efficiently.

## Basic Navigation

### Jump to Mark Line

`'{mark}` jumps to the first non-blank character of the
marked line. This is a linewise motion.

### Jump to Exact Position

`` `{mark} `` jumps to the exact line and column of the
mark. This is a characterwise motion.

## Navigation Patterns

### Round-Trip

1. Set mark `a` at current position: `ma`
2. Navigate elsewhere to inspect code
3. Return to marked position: `` `a ``

### Quick Toggle

`''` or ` `` `` ` jumps between the previous position and
the current one, creating a two-point toggle.

### Multi-Point Navigation

Set marks `a`, `b`, `c` at different locations.
Jump between them with `` `a ``, `` `b ``, `` `c ``.

## Motion vs Jump

### With Operators

Marks work as motions with operators:
- `d'a` deletes from current line to mark `a` (linewise)
- `` d`a `` deletes from cursor to mark `a` (characterwise)
- `` y`a `` yanks from cursor to mark `a`
- `>'a` indents from current line to mark `a`

### Visual Selection

`` v`a `` starts visual selection from cursor to mark `a`.
`V'a` starts linewise visual selection to mark `a` line.

## Efficient Workflows

### Edit and Return

1. `ma` — mark current position
2. Navigate to another location and edit
3. `` `a `` — return to where you were
4. `.` — jump to where last edit was made

### Compare Two Locations

1. `ma` — mark first location
2. Navigate to second location, `mb`
3. `` `a `` and `` `b `` to switch between them

### Mark Range for Operations

1. `ma` at start of range
2. Navigate to end of range
3. `d'a` — delete the range (linewise)

## Global Mark Navigation

Global marks (A-Z) switch files automatically:
`` `A `` opens the file containing mark `A` if not
already open, and jumps to the marked position.
This creates a buffer switch and a jumplist entry.

## Special Mark Navigation

### Last Change Location

`` `. `` jumps to the position of the last change.
`'.` jumps to the line of the last change.

### Last Insert Location

`` `^ `` jumps to where insert mode was last exited.

### Visual Selection Bounds

`` `< `` jumps to the start of the last visual selection.
`` `> `` jumps to the end of the last visual selection.

## Navigation Without Jumplist

Use `` g` `` and `g'` prefix to jump to a mark without
adding the jump to the jumplist:
- `` g`a `` goes to mark `a` without jumplist entry
- `g'a` goes to mark `a` line without jumplist entry

## Combining with Search

1. `ma` — mark position
2. `/pattern<CR>` — search for pattern
3. Edit if needed
4. `` `a `` — return to marked position

## Mark-Based Scrolling

`z'a` scrolls so that mark `a`'s line appears at the
top of the window. Combined with `z.` and `z-` for
middle and bottom.

## Next/Previous Mark

Navigate between set marks:

| Key | Action |
|-----|--------|
| `]'` | Next lowercase mark (by line) |
| `['` | Previous lowercase mark (by line) |
| `` ]` `` | Next mark (exact position) |
| `` [` `` | Previous mark (exact position) |

Marks are ordered by their position in the buffer,
not by their letter name.

## Marks Picker

`:Marks` opens an interactive picker showing all marks
with their positions. Selecting a mark jumps to it.
The picker shows: mark letter, line number, column,
and the text of the marked line.

## Configuration

| Option | Default | Description |
|--------|---------|-------------|
| `show_marks_in_gutter` | `true` | Show mark indicators in sign column |
| `mark_gutter_style` | `"letter"` | Display style: letter or icon |
