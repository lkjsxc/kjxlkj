# Line Motions

Back: [docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Motions that move within the current line.

## Line Position Motions

| Key | Action | Type |
|-----|--------|------|
| `0` | First column (column 0) | Exclusive |
| `^` | First non-blank character | Exclusive |
| `$` | End of line (last character) | Inclusive |
| `g_` | Last non-blank character | Inclusive |
| `gm` | Middle of display line | Exclusive |
| `{N}\|` | Column N (1-based) | Exclusive |

## Display Line Variants

When `wrap` is enabled, a buffer line may span
multiple display rows. The `g` variants navigate
by display line rather than buffer line.

| Key | Action | Type |
|-----|--------|------|
| `g0` | First column of display line | Exclusive |
| `g^` | First non-blank of display line | Exclusive |
| `g$` | End of display line | Inclusive |
| `gj` | Down one display line | Linewise |
| `gk` | Up one display line | Linewise |

## Count Behavior

### Dollar with Count

`$` with count greater than 1 moves down `count-1`
lines to the end of that line:
- `d$` (count=1): delete to end of current line
- `d2$`: delete to end of next line (2 lines total)

### Column Motion with Count

`{N}|` moves to the Nth display column on the
current line (1-based). If the line is shorter
than N columns, the cursor moves to the last column.

## Operator Interaction

| Combination | Result |
|-------------|--------|
| `d0` | Delete to start of line (exclusive) |
| `d$` / `D` | Delete to end of line (inclusive) |
| `d^` | Delete to first non-blank (exclusive) |
| `c$` / `C` | Change to end of line |
| `y0` | Yank to start of line |
| `y$` / `Y` | Yank to end of line |
| `c^` | Change to first non-blank |

## CJK Display Column Handling

### Column Counting

Column positions for `{N}|` use display columns,
not byte offsets or grapheme indices. A CJK
character occupies 2 display columns.

### Column Alignment

If `{N}|` targets the second display column of a
CJK character, the cursor moves to the start of
that CJK grapheme (always aligns to grapheme
boundary). The cursor never lands in the "middle"
of a wide character.

### Example

Text: `あいう` (columns 1-2, 3-4, 5-6)
- `1|` -> cursor on `あ` (column 1)
- `2|` -> cursor on `あ` (column 1, aligned)
- `3|` -> cursor on `い` (column 3)
- `5|` -> cursor on `う` (column 5)

## Blank Line Behavior

### First Non-Blank on Empty Line

On an empty line, `^` places the cursor at column 0
(same as `0`). On a line with only whitespace, `^`
moves to the last whitespace character.

### Dollar on Empty Line

`$` on an empty line places the cursor at column 0
(the virtual end-of-line position).

## Visual Mode

In visual mode, line motions extend the selection:
- `v$` extends selection to end of line
- `v0` extends selection to start of line
- `v^` extends selection to first non-blank

## Related

- Vertical motions: [docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- Cursor model: [docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Display wrapping: [docs/spec/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- CJK width: [docs/technical/unicode.md](/docs/technical/unicode.md)
