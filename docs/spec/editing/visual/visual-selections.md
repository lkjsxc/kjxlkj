# Visual Selections

Visual mode selection types and behavior.

## Overview

Visual mode allows selecting text interactively before
applying operators. Three visual sub-modes exist:
character, line, and block.

## Character Visual (v)

### Enter

Press `v` in normal mode. Statusline shows `-- VISUAL --`.

### Selection

Text is selected from the start position to the cursor.
The selection is character-granular.

### Inclusive

The character under the cursor is always included in
the selection. Selection covers start through cursor.

## Line Visual (V)

### Enter

Press `V` (shift-v) in normal mode.
Statusline shows `-- VISUAL LINE --`.

### Selection

Entire lines are selected. Even if the cursor moves
mid-line, the full lines from start to cursor are selected.

### Operators

All operators act on whole lines when applied.

## Block Visual (Ctrl-V)

### Enter

Press `<C-v>` in normal mode.
Statusline shows `-- VISUAL BLOCK --`.

### Selection

A rectangular block of text is selected defined by
two corner positions. Useful for columnar editing.

### Width

The block width is determined by the start and cursor
columns. All lines between start and cursor rows are
included with the same column range.

## Extending Selection

### Motions

All normal mode motions work to extend the selection:
`w`, `e`, `b`, `}`, `{`, `gg`, `G`, `/pattern`, etc.

### Text Objects

`viw` selects inner word, `va(` selects around parens.
In visual mode, text objects extend the selection.

### Other End

`o` moves the cursor to the other end of the selection.
`O` in block mode moves to the other corner.

## Operators on Selection

### Common Operators

| Key | Action |
|-----|--------|
| `d` | Delete selection |
| `y` | Yank selection |
| `c` | Change selection (delete + insert) |
| `>` | Indent selection |
| `<` | Outdent selection |
| `=` | Auto-format selection |
| `~` | Toggle case |
| `u` | Lowercase selection |
| `U` | Uppercase selection |
| `J` | Join selected lines |
| `:` | Enter command mode with range |

## Reselect

### gv

`gv` reselects the previous visual selection.
The same mode (char/line/block) is restored.

### After Operator

After an operator acts on a visual selection, `gv`
reselects the same region (adjusted for changes).

## Selection Adjustment

### Increase/Decrease

In visual mode, the selection can be adjusted with
any motion. `<C-v>` + arrow keys adjust block selection.

### Switch Mode

Press `v`, `V`, or `<C-v>` during visual to switch
sub-mode without losing the anchor position.

## Register Interaction

### Specify Register

`"ay` yanks selection into register `a`.
`"ap` pastes from register `a`.

### Put Over Selection

In visual mode, `p` replaces the selection with the
register contents. The replaced text goes into the
unnamed register.

## Search in Selection

### Restrict to Selection

`/\%Vpattern` restricts search to the visual selection.
In command mode from visual: `:'<,'>s/old/new/g`.

## CJK Considerations

### Wide Characters

Visual selection boundaries align to grapheme clusters.
For CJK characters (display width 2), the selection
highlight covers both columns of each wide character.

### Block Mode

In visual block mode with CJK, the column boundaries
may not align exactly due to double-width characters.
The selection rounds outward to include any partially
covered wide character.
