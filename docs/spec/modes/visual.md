# Visual Mode

Visual mode provides explicit selection regions for operator actions.
The selection is always visible and continuously updates as the cursor moves.

## Variants

| Mode | Entry | Selection Type |
|------|-------|---------------|
| Visual (char) | `v` | Character-wise |
| Visual Line | `V` | Line-wise |
| Visual Block | `Ctrl-v` | Block/column |

## Sub-mode Switching

Pressing a different visual entry key while already in Visual mode switches sub-modes.

| From | Press | Result |
|------|-------|--------|
| Char (`v`) | `V` | Switch to Line-wise |
| Char (`v`) | `Ctrl-v` | Switch to Block-wise |
| Line (`V`) | `v` | Switch to Char-wise |
| Line (`V`) | `Ctrl-v` | Switch to Block-wise |
| Block (`Ctrl-v`) | `v` | Switch to Char-wise |
| Block (`Ctrl-v`) | `V` | Switch to Line-wise |

Switching sub-modes SHALL preserve the anchor position. The selection is
reinterpreted under the new sub-mode semantics immediately.

## Selection Semantics by Sub-mode

| Sub-mode | Selection Region |
|----------|-----------------|
| Char | Contiguous range of characters from anchor to cursor, inclusive on both ends. |
| Line | All complete lines from the anchor line to the cursor line, inclusive. |
| Block | Rectangular region defined by anchor and cursor as opposite corners. |

## Selection Behavior

| Aspect | Behavior |
|--------|----------|
| Anchor | Fixed at the position where Visual mode was entered |
| Cursor | Extends or contracts the selection boundary |
| Motions | All Normal-mode motions move the cursor and reshape the selection |
| `o` | Swap anchor and cursor ends (see Additional Visual Commands) |

## Block Mode Specifics

Block selection forms a rectangle in **screen columns**, not byte offsets.
Wide characters (e.g., CJK) and tabs are measured by display width.

| Key | Action |
|-----|--------|
| `I` | Enter Insert mode at the left edge of the block; typed text is replicated to every selected line on `Esc`. |
| `A` | Enter Insert mode at the right edge of the block; typed text is appended on every selected line on `Esc`. |
| `c` | Delete block contents, enter Insert on the first selected line; text is replicated to all lines on `Esc`. |
| `r{char}` | Replace every character in the block with `{char}`. |
| `$` | Extend selection to end of each line individually, producing a ragged right edge. |

## Operators in Visual Mode

When an operator is applied, it acts on the exact selection and then returns to Normal mode.

| Key | Action |
|-----|--------|
| `d` | Delete selection |
| `y` | Yank (copy) selection |
| `c` | Change (delete + enter Insert) |
| `>` | Indent selection |
| `<` | Outdent selection |
| `~` | Toggle case of each character |
| `=` | Reindent selection |
| `gq` | Format selection (reflow text to `textwidth`) |
| `u` | Lowercase selection |
| `U` | Uppercase selection |
| `J` | Join selected lines into one line |
| `p` / `P` | Put: replace selection with register contents |
| `!` | Filter selection through an external command |

## Additional Visual Commands

| Key | Action |
|-----|--------|
| `o` | Swap anchor and cursor positions |
| `O` | In Block mode: move cursor to the other corner on the same line |
| `gv` | Reselect the previous visual area (range and sub-mode) |
| `Ctrl-a` | Increment numbers within the selection |
| `Ctrl-x` | Decrement numbers within the selection |
| `:` | Enter Command mode with the range `'<,'>` auto-filled |
| `I` | Insert at the start of each line (Block mode; see Block Mode Specifics) |
| `A` | Append at the end of each line (Block mode; see Block Mode Specifics) |

## Visual Search

| Key | Action |
|-----|--------|
| `*` | Search forward for the selected text literally |
| `#` | Search backward for the selected text literally |

When a visual selection is active, `*` and `#` SHALL search for the exact
selected text, overriding their Normal-mode behavior of searching for the
word under the cursor.

## Count with Visual

- A count before `v`, `V`, or `Ctrl-v` multiplies the size of the most
  recent visual area. For example, if the last selection covered 3 lines,
  `2V` reselects 6 lines starting from the cursor.
- Operators invoked from Visual mode do NOT consume a count; the selection
  itself defines the operative range.

## Highlighting

- The selection MUST be visually distinct from normal text at all times.
- The cursor position within the selection MUST remain identifiable (e.g.,
  via a distinct highlight or cursor shape).
- The highlighting style is configurable via the `Visual` highlight group.

## Exit

| Trigger | Result |
|---------|--------|
| `Esc` | Cancel selection, return to Normal mode |
| Operator | Execute operator on the selection, return to Normal mode |
| Same sub-mode key | Pressing the current sub-mode key (e.g., `v` while in Char) exits Visual mode |

## Invariants

1. A visual selection SHALL NOT cross buffer boundaries.
2. Anchor and cursor SHALL always refer to valid positions in the buffer.
3. Operators in Visual mode SHALL operate on the exact selection, then
   return to Normal mode.
4. `gv` SHALL restore the most recent visual selection, including both the
   range and the sub-mode.
5. Undo after a visual operation SHALL restore the original text and
   deselect (return to Normal mode).

## Related

- Editing overview: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Visual editing details: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Multiple cursors: [/docs/spec/features/editing/multicursor.md](/docs/spec/features/editing/multicursor.md)
