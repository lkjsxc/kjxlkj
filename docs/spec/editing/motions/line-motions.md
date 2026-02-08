# Line Motions

Back: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Motions that move within the current line.

## Line position motions (normative)

| Key | Action | Motion type |
|---|---|---|
| `0` | Move to first column (column 0) | Exclusive |
| `^` | Move to first non-blank character | Exclusive |
| `$` | Move to end of line (last character) | Inclusive |
| `g_` | Move to last non-blank character | Inclusive |
| `g0` | Move to first column of display line (wrap-aware) | Exclusive |
| `g^` | Move to first non-blank of display line | Exclusive |
| `g$` | Move to end of display line | Inclusive |
| `gm` | Move to middle of display line (by column width) | Exclusive |
| `{N}\|` | Move to column N (1-based) | Exclusive |

## Count behavior

`$` with a count greater than 1 moves down `count - 1` lines to the end of that line. This makes `d$` with `count = 1` delete to end of current line, but `d2$` deletes to end of the next line (linewise).

## Display line vs buffer line

When `wrap` is enabled, a single buffer line may occupy multiple display rows. The `g` variants (`g0`, `g$`, `gj`, `gk`) navigate display lines rather than buffer lines.

## Operator interaction

| Combination | Result |
|---|---|
| `d0` | Delete from cursor to start of line (exclusive) |
| `d$` / `D` | Delete from cursor to end of line (inclusive) |
| `d^` | Delete from cursor to first non-blank (exclusive) |
| `c$` / `C` | Change from cursor to end of line |
| `y0` | Yank from cursor to start of line |

## CJK interaction

Column positions for `{N}\|` are in display columns (not grapheme indices). A CJK character at display column 5 occupies columns 5-6. Moving to column 6 via `6\|` places the cursor on that same CJK grapheme (cursor always on grapheme boundary).

## Related

- Character motions: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Display line semantics: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
