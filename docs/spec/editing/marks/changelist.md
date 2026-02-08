# Change List

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Navigate through the list of positions where changes were made.

## Overview

The change list records the position of every change. The editor maintains a per-buffer list of change positions. Navigation commands move through this list.

## Navigation

| Key | Command | Description |
|---|---|---|
| `g;` | `:changes` | Go to older change position |
| `g,` | - | Go to newer change position |
| `:changes` | - | Display the change list |

## How It Works

Each edit operation (insert, delete, change) records the line and column where it occurred. The list grows as edits are made.

| Event | Recorded |
|---|---|
| Text insertion | Position where insert began |
| Text deletion | Position of first deleted character |
| Text change | Position of first changed character |
| Put/paste | Position where text was placed |

## List Size

| Setting | Default | Description |
|---|---|---|
| `changelist_size` | `100` | Maximum entries per buffer |

When the list is full, the oldest entry is removed.

## Relationship to Undo

The change list is separate from the undo tree. `g;`/`g,` only move the cursor — they do not undo or redo changes.

## Session Persistence

Change list positions are saved in the session file so they survive editor restart.

## Related

- Marks overview: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Jump list: [/docs/spec/editing/marks/jumplist.md](/docs/spec/editing/marks/jumplist.md)
