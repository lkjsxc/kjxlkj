# Buffers
Buffers are core-owned, single-writer state.

## Requirements

- The core is the only writer of buffer content.
- Services may request reads (snapshots) and propose edits via typed operations.
- Buffer identity is stable and independent from filesystem paths.

## Core buffer state

A buffer is defined by:

- `BufferId` (stable)
- `BufferName` (display)
- optional filesystem path (file-backed)
- content (rope/piece table; implementation detail)
- modified flag
- encoding and line-ending policy

## Snapshots

Rendering and services consume immutable buffer snapshots.

- Snapshots are versioned (monotonic `BufferVersion`).
- Async results (syntax, diagnostics, git hunks) must be tagged with the version they were computed from.
- Core may drop stale results.

## Large files

spec supports large-file mode:

- Partial/streaming reads are handled by the FS service.
- Core keeps UI responsive by limiting expensive per-keystroke work.
- Syntax and indexing may degrade gracefully.

## Related

- FS service: [docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Syntax: [docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)
- Diagnostics/list UI: [docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)
