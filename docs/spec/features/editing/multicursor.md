# Multi-Cursor Editing

Back: [docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

Simultaneous editing at multiple cursor positions.

## Overview

Multi-cursor mode allows placing multiple cursors
in the buffer and editing at all positions
simultaneously. Each cursor has its own position,
mode state, and selection.

## Creating Cursors

### From Normal Mode

| Key | Action |
|-----|--------|
| `Ctrl-d` | Add cursor at next match of word |
| `Ctrl-Shift-d` | Add cursor at prev match |
| `Ctrl-Alt-j` | Add cursor on line below |
| `Ctrl-Alt-k` | Add cursor on line above |
| `Ctrl-Alt-a` | Add cursor at all matches |

### From Visual Mode

| Key | Action |
|-----|--------|
| `Ctrl-d` | Select next occurrence of selection |
| `Ctrl-Shift-d` | Skip current, select next |
| `I` | Place cursor at start of each line |
| `A` | Place cursor at end of each line |

### From Search

After `/pattern`, `Ctrl-Alt-a` creates a cursor
at every match position in the buffer.

## Cursor Behavior

### Independent Cursors

Each cursor operates independently:
- Has its own position (line, column)
- Has its own selection (if in visual mode)
- Receives the same keystrokes simultaneously
- Produces individual undo entries (grouped)

### Primary Cursor

One cursor is designated as the primary cursor.
The viewport follows the primary cursor. The
status line shows the primary cursor's position.

### Cursor Merging

When two cursors occupy the same position (e.g.,
after a motion), they are automatically merged
into a single cursor. This prevents duplicate
edits at the same location.

## Editing Operations

### Supported Operations

All normal mode operators work with multi-cursor:

| Operation | Behavior |
|-----------|----------|
| Insert text | Same text inserted at each cursor |
| Delete | Each cursor deletes independently |
| Change | Each cursor changes its region |
| Yank | Each cursor yanks (primary register) |
| Paste | Same text pasted at each cursor |

### Motion Independence

After a motion, each cursor moves independently.
For example, `w` moves each cursor to its own next
word, which may be at a different column on each
line.

### Selection Per Cursor

In visual-multi mode, each cursor has its own
selection region. Operators act on each individual
selection.

## Exiting Multi-Cursor

| Key | Action |
|-----|--------|
| `Esc` | Exit multi-cursor, keep primary |
| `Ctrl-c` | Exit and clear all extra cursors |

## Undo Behavior

### Grouped Undo

All edits across all cursors within a single
operation are grouped as one undo entry. Pressing
`u` undoes the edit at all cursor positions.

## Display

### Cursor Rendering

| Element | Appearance |
|---------|------------|
| Primary cursor | Normal cursor style |
| Secondary cursors | Dimmed or different color |
| Selections | Highlighted per cursor |

### Status Line

Shows cursor count: `[3 cursors]` when multiple
cursors are active.

## Limitations

### Cross-Buffer

Multi-cursor operates within a single buffer only.
Cursors cannot span multiple buffers.

### Command Mode

Ex commands are executed once, not per-cursor.
The command operates on the state seen by the
primary cursor.

### Macro Interaction

Macros are recorded/played with multi-cursor state.
Each cursor receives the macro keystrokes.

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `multicursor.enabled` | bool | true | Enable feature |
| `multicursor.highlight` | string | "dim" | Secondary cursor style |

## Related

- Visual mode: [docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Operators: [docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Search: [docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
