# Undo Persistence

Back: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)

Persistent undo history across editor sessions.

## Overview

Undo persistence saves undo history to disk so that undo/redo operations survive editor restarts. Each buffer's undo tree is serialized to a file when the buffer is written or when the editor exits.

## File location

| Setting | Default | Description |
|---|---|---|
| `undo.persistent` | `true` | Enable persistent undo |
| `undo.dir` | `~/.local/share/kjxlkj/undo/` | Directory for undo files |

## Undo file naming

The undo file name is derived from the buffer's absolute file path by replacing path separators with `%` characters. Example: `/home/user/project/src/main.rs` becomes `%home%user%project%src%main.rs`.

## Undo file format

The undo file is a binary format containing:

| Section | Content |
|---|---|
| Header | Magic bytes, version number, file hash |
| Entries | Serialized undo tree nodes (edit operations with before/after content) |
| Cursor | Cursor position at each undo node |

## File hash validation

The undo file stores a hash of the buffer content at save time. On load, if the current file content hash does not match, the undo file is discarded (the file was modified outside the editor).

## Undo tree serialization

Each undo entry stores:

| Field | Type | Description |
|---|---|---|
| `edit_type` | enum | Insert, delete, or replace |
| `range` | (start, end) | Byte range affected |
| `old_content` | bytes | Content before the edit |
| `new_content` | bytes | Content after the edit |
| `cursor_before` | (line, col) | Cursor position before edit |
| `cursor_after` | (line, col) | Cursor position after edit |
| `timestamp` | u64 | Unix timestamp of the edit |

Branch nodes in the undo tree store pointers to their children, preserving the full undo tree (not just linear undo).

## Size limits

| Setting | Default | Description |
|---|---|---|
| `undo.max_file_size` | `10485760` (10 MB) | Maximum undo file size |
| `undo.max_entries` | `10000` | Maximum undo entries per buffer |

When limits are reached, oldest entries are pruned.

## Commands

| Command | Description |
|---|---|
| `:undolist` | Show undo tree entries with timestamps |
| `:earlier {time}` | Restore buffer to state from `{time}` ago (e.g., `:earlier 5m`) |
| `:later {time}` | Move forward in undo history by `{time}` |

## Error handling

| Error | Behavior |
|---|---|
| Undo directory does not exist | Create it automatically |
| Undo file corrupted | Discard and start with empty undo history |
| File hash mismatch | Discard undo file |
| Disk full | Log warning, continue without persistent undo |

## Related

- Undo: [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- Session undo tree: [/docs/spec/features/session/undo_tree.md](/docs/spec/features/session/undo_tree.md)
