# Change List

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

Navigate through positions where changes were made in the buffer.

## Overview

The change list records the cursor position each time a text modification occurs. This allows jumping between edit locations using `g;` and `g,`.

## Navigation

| Key | Action |
|---|---|
| `g;` | Jump to the previous (older) change position |
| `g,` | Jump to the next (newer) change position |

## Change list contents

Each entry stores:

| Field | Type | Description |
|---|---|---|
| `line` | integer | Line number where the change occurred |
| `col` | integer | Column (grapheme offset) of the change |

## Capacity

| Setting | Default | Description |
|---|---|---|
| Change list size | 100 | Maximum number of entries per buffer |

When the list is full, the oldest entry is removed.

## Recording changes

A new entry is added to the change list whenever:

- Text is inserted (Insert mode input)
- Text is deleted (operator `d`, `x`, etc.)
- Text is changed (operator `c`)
- A substitution is performed (`:s`)
- An undo or redo operation occurs

Multiple adjacent edits in Insert mode (continuous typing) are grouped into a single change list entry at the position where typing began.

## Relationship to marks

The `'.` and `` `. `` marks point to the position of the last change. The change list provides a full history, whereas these marks only point to the most recent change.

## Command

| Command | Description |
|---|---|
| `:changes` | Display the change list for the current buffer |

## Related

- Jump list: [/docs/spec/features/navigation/jumplist.md](/docs/spec/features/navigation/jumplist.md)
- Marks: [/docs/spec/features/navigation/marks.md](/docs/spec/features/navigation/marks.md)
- Change list (editing spec): [/docs/spec/editing/marks/changelist.md](/docs/spec/editing/marks/changelist.md)
