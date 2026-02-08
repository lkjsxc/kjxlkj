# Visual Selections

Back: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)

Types and behavior of visual mode selections.

## Overview

Visual mode highlights a region of text as the "selection." Operators act on the selected text. Three sub-modes define the shape of the selection.

## Character-wise Selection (`v`)

Selects a contiguous range of characters from the start position to the cursor position. The selection wraps across lines.

| Aspect | Behavior |
|---|---|
| Start | Position when `v` was pressed |
| End | Current cursor position |
| Shape | Contiguous character range |
| Line wrap | Selection spans across line boundaries |

## Line-wise Selection (`V`)

Selects entire lines from the start line to the cursor line.

| Aspect | Behavior |
|---|---|
| Start | Line where `V` was pressed |
| End | Current cursor line |
| Shape | Full lines |
| Partial line | Not possible; always selects complete lines |

## Block-wise Selection (`<C-v>`)

Selects a rectangular block defined by two corners.

| Aspect | Behavior |
|---|---|
| Corners | Start position and cursor position define opposite corners |
| Shape | Rectangle (column × row) |
| CJK | If a wide character is partially overlapped, the full character is included |

## Selection Expansion

Motions expand or contract the selection:

| Motion | Effect |
|---|---|
| `w` | Extend selection one word forward |
| `b` | Move cursor (anchor stays), shrinks/extends |
| `}` | Extend to next paragraph boundary |
| `G` | Extend to end of file |

## Switching Modes

| Key | From | To |
|---|---|---|
| `v` | Visual-line or block | Visual-char |
| `V` | Visual-char or block | Visual-line |
| `<C-v>` | Visual-char or line | Visual-block |
| `o` | Any | Swap anchor and cursor |
| `O` | Block | Move cursor to other corner on same line |

## Reselect

`gv` reselects the previous visual selection with the same mode and range (adjusted for text changes).

## Visual Marks

| Mark | Meaning |
|---|---|
| `'<` | Start of last visual selection |
| `'>` | End of last visual selection |

## Operators on Selection

After selecting, press an operator key:

| Key | Operator |
|---|---|
| `d` | Delete selection |
| `y` | Yank selection |
| `c` | Change selection |
| `>` | Indent |
| `<` | Unindent |
| `gU` | Uppercase |
| `gu` | Lowercase |
| `=` | Re-indent |

## Related

- Visual mode: [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
