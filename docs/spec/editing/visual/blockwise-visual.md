# Blockwise Visual Mode

Back: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)

Rectangular (block) selections across multiple lines.

## Entry (normative)

| Key | Action |
|---|---|
| `Ctrl-v` | Enter Visual Block mode from Normal |
| `Ctrl-v` (from Visual Char or Line) | Switch to Visual Block |

## Selection shape (normative)

The selection is a rectangle defined by the anchor position and the current cursor position. The anchor is the position when block mode was entered.

| Property | Calculation |
|---|---|
| Left column | `min(anchor.col, cursor.col)` |
| Right column | `max(anchor.col, cursor.col)` |
| Top line | `min(anchor.line, cursor.line)` |
| Bottom line | `max(anchor.line, cursor.line)` |

Lines shorter than the right column extend only to their actual length (no virtual space is selected).

## `$` extension (normative)

Pressing `$` in Visual Block mode extends the selection to the end of each line, regardless of line length. This creates a ragged-right selection.

## Block insert (normative)

| Key | Action |
|---|---|
| `I` | Insert text before the block on every selected line |
| `A` | Append text after the block on every selected line |

After pressing `I` or `A`, the user types text and presses `Esc`. The typed text is then replicated on all selected lines at the block boundary.

## Block operations (normative)

| Key | Action |
|---|---|
| `d` / `x` | Delete the rectangular block; lines shift left to fill |
| `c` | Delete the block, enter Insert; typed text replicated on all lines |
| `y` | Yank the block as blockwise text |
| `r{char}` | Replace every character in the block with `{char}` |
| `>` / `<` | Indent / dedent the entire lines (not just the block region) |
| `~` | Toggle case of all characters in the block |
| `u` / `U` | Lowercase / uppercase all characters in the block |

## Paste behavior (normative)

Pasting blockwise-yanked text with `p` or `P` inserts it as a rectangular block at the cursor position, one fragment per line.

## CJK handling

When a block boundary intersects a width-2 CJK grapheme, the grapheme is included in the selection if its starting column falls within the block range.

## Related

- Visual mode: [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- Visual selections: [/docs/spec/editing/visual/visual-selections.md](/docs/spec/editing/visual/visual-selections.md)
- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

