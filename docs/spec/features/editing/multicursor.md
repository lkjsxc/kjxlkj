# Multiple Cursors

## User intent

Make repeated edits across multiple locations simultaneously.

## Model (normative)

| Concept | Requirement |
|---|---|
| Primary cursor | The cursor that anchors viewport and mode decisions. Always exists. Index 0. |
| Secondary cursors | Additional carets that receive mirrored edits. Sorted by buffer position. |
| Selection sets | Each cursor may carry a selection range (for Visual-mode operations). |
| Conflict policy | Overlapping edits MUST be resolved deterministically: process cursors bottom-to-top so offset adjustments propagate correctly. |

## Key bindings (normative)

| Key | Action |
|---|---|
| `Ctrl-d` (or configured key) | Add cursor at next occurrence of current word/selection |
| `Ctrl-k Ctrl-d` | Skip current match, add cursor at next match |
| `Ctrl-Shift-l` | Add cursors at all occurrences of current word/selection |
| `Esc` | Remove all secondary cursors, keep primary |

## Editing semantics (normative)

1. When multiple cursors are active, all Normal-mode and Insert-mode operations apply at each cursor independently.
2. The core MUST compute all cursor positions before applying any edits, then apply edits bottom-to-top.
3. After each edit, cursor positions above the edit point are adjusted by the byte/line delta of the edit.
4. If two cursors end up at the same position after an edit, they are merged into one (deduplication).
5. The entire multi-cursor operation is a single undo unit.

## Operator behavior with multiple cursors (normative)

| Operation | Behavior |
|---|---|
| `d{motion}` | Delete the motion range at each cursor |
| `c{motion}` | Change the motion range at each cursor; all enter Insert mode |
| `y{motion}` | Yank from each cursor; register holds concatenated text with newlines |
| `p` | Paste at each cursor from register (if register has N lines and there are N cursors, distribute one line per cursor) |
| Insert text | Characters typed are inserted at all cursor positions simultaneously |

## Viewport behavior

The viewport follows the primary cursor. Secondary cursors may be off-screen; this is acceptable. The statusline SHOULD indicate the number of active cursors (e.g., "3 cursors").

## Constraints

- Multi-cursor operations MUST preserve editor responsiveness.
- Undo MUST revert the entire multi-cursor change as one step.
- Maximum cursor count: 10000 (to prevent accidental performance degradation).

## Related

- Operators: [/docs/spec/editing/operators/operators.md](/docs/spec/editing/operators/operators.md)
- Visual mode: [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Undo: [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
