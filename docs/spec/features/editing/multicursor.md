# Multi-Cursor Editing

Back: [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

Edit at multiple cursor positions simultaneously.

## Overview

Multi-cursor mode allows placing multiple cursors and editing at all positions at once.

## Activation

| Key | Command | Description |
|---|---|---|
| `<C-d>` | `:MultiCursorWord` | Add cursor at next occurrence of current word |
| `<C-S-d>` | `:MultiCursorAllWords` | Add cursor at all occurrences |
| `<A-j>` | `:MultiCursorDown` | Add cursor on line below |
| `<A-k>` | `:MultiCursorUp` | Add cursor on line above |
| `<Esc>` | - | Exit multi-cursor mode |

## Behavior

Each cursor operates independently:

- Insert mode keystrokes are applied at all cursor positions.
- Motions move all cursors.
- Operators apply at each cursor's range.

## Visual Multi-Cursor

From visual mode, `<C-d>` adds cursors at each occurrence of the selected text.

## Limitations

- All cursors must be in the same buffer.
- Some complex operations (macros, some ex commands) only apply to the primary cursor.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `multicursor.highlight` | `MultiCursor` | Highlight group for extra cursors |

## Related

- Editing features: [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)
- Visual mode: [/docs/spec/modes/visual/README.md](/docs/spec/modes/visual/README.md)
