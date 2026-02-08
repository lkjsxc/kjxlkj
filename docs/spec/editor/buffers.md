# Buffers

Back: [/docs/spec/editor/README.md](/docs/spec/editor/README.md)

Buffers are core-owned, single-writer state.

## Requirements

- The core is the only writer of buffer content.
- Services may request reads (snapshots) and propose edits via typed operations.
- Buffer identity is stable and independent from filesystem paths.

## Core buffer state

A buffer is defined by:

| Field | Type | Description |
|---|---|---|
| `id` | `BufferId` | Stable unique identifier (monotonic counter). |
| `name` | `BufferName` | Display name (filename or `[No Name]`). |
| `path` | `Option<PathBuf>` | Filesystem path for file-backed buffers. |
| `content` | `Rope` | Text content stored as a rope data structure. |
| `modified` | `bool` | True if content differs from last-saved state. |
| `version` | `BufferVersion` | Monotonically increasing edit counter. |
| `encoding` | `Encoding` | Character encoding (default: UTF-8). |
| `line_ending` | `LineEnding` | `Lf` or `CrLf` (detected on load, default `Lf`). |
| `readonly` | `bool` | Prevents edits when true. |

## Text model: Rope (normative)

The buffer content MUST be stored in a rope data structure. The rope provides:

- O(log n) insert and delete at arbitrary positions.
- O(log n) line-index lookup (byte offset to line number and vice versa).
- O(1) amortized access to contiguous leaf chunks.
- Efficient snapshot cloning via structural sharing (copy-on-write).

### Recommended crate

Use the `ropey` crate (>= 1.6). It provides `Rope`, `RopeSlice`, line/char/byte indexing, and chunk iteration.

### Indexing layers

The rope stores raw bytes (UTF-8). Higher layers convert between index types:

| Layer | Unit | Use |
|---|---|---|
| Byte offset | `usize` | Rope internal addressing, LSP positions. |
| Char index | `usize` | Rope char-based API. |
| Grapheme index | `usize` | Cursor positions (see [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)). |
| Line number | `usize` | Zero-based line addressing. |

Conversion between grapheme index and byte/char offset uses `unicode-segmentation` iteration over the line's content.

### Edit operations

All edits go through a single `apply_edit` method:

| Operation | Parameters | Rope action |
|---|---|---|
| Insert | `(line, grapheme_offset, text)` | Convert to byte offset, `rope.insert(byte_offset, text)`. |
| Delete | `(start_line, start_grapheme, end_line, end_grapheme)` | Convert both to byte offsets, `rope.remove(start..end)`. |
| Replace | `(range, text)` | Delete then insert at start. |

Every edit increments `version` and sets `modified = true`.

## Snapshots

Rendering and services consume immutable buffer snapshots.

- Snapshots are versioned (monotonic `BufferVersion`).
- A snapshot clones the rope (cheap via structural sharing).
- Async results (syntax, diagnostics, git hunks) must be tagged with the version they were computed from.
- Core may drop stale results if the version has advanced.

### Snapshot contents

| Field | Type |
|---|---|
| `id` | `BufferId` |
| `version` | `BufferVersion` |
| `content` | `Rope` (clone) |
| `line_count` | `usize` |
| `path` | `Option<PathBuf>` |
| `modified` | `bool` |

## Buffer list

The editor maintains an ordered list of all open buffers.

| Command | Behavior |
|---|---|
| `:ls` / `:buffers` | List all buffers with id, name, modified flag, path. |
| `:b {n}` | Switch to buffer by id. |
| `:bn` / `:bp` | Next/previous buffer. |
| `:bd` | Delete buffer (refuses if modified unless `!`). |
| `:e {path}` | Open file into new buffer or switch to existing. |

## Large files

Large-file mode activates when a file exceeds 10 MB:

- Syntax highlighting is disabled.
- Undo history depth is limited.
- Line-count updates may be deferred.
- The FS service handles streaming reads.

## Related

- Text model internals: [/docs/technical/unicode.md](/docs/technical/unicode.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- FS service: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Syntax: [/docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)
- Diagnostics: [/docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)
- Undo: [/docs/spec/features/session/undo_tree.md](/docs/spec/features/session/undo_tree.md)
