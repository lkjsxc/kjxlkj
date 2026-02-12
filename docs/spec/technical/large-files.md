# Large File Performance

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
Normative requirements that keep the editor responsive and memory-efficient on large buffers.

## Definitions

“Large” is contextual. This spec uses both byte size and structural size:

| Category | Approx size | Typical shape |
|----------|-------------|---------------|
| Small | <1 MiB | interactive without special handling |
| Medium | 1–10 MiB | common log/source files |
| Large | 10–100 MiB | may require feature degradation |
| Very large | >100 MiB | optional support; strict degradation expected |

Structural stressors (can matter more than bytes):

- extremely long lines
- very high line counts
- heavy non-ASCII / wide-grapheme content

## Core performance invariants (normative)

Performance guarantees for large file operations.

### Snapshot cost MUST be viewport-bounded

The core produces immutable snapshots for rendering.

To avoid O(file) work on every frame:

- Snapshot generation MUST NOT iterate over the entire buffer to build render data.
- Snapshot generation MUST be O(viewport) in both time and allocation:
  - at most the visible text rows (plus a small constant margin, if required)
  - plus metadata (cursor, mode, filename, line count)

### Render must not require full-buffer cloning

- The renderer MUST be able to render a frame using only:
  - viewport geometry
  - cursor position
  - a viewport-sized slice of lines
  - status/command line metadata
- The renderer MUST NOT require “all buffer lines” to be present inside a snapshot.

### Idle rendering SHOULD be avoided

When no input and no UI-driven animation is occurring:

- The host/render loop SHOULD avoid rebuilding snapshots and redrawing frames while idle.

If a periodic tick is required (cursor blink, timers), it MUST be bounded and MUST NOT re-materialize full-buffer data.

## File opening requirements (normative)

Behavior on opening large files.

### Streaming open SHOULD avoid intermediate full copies

When opening a file from disk:

- The implementation SHOULD stream UTF-8 content directly into the rope/text model (reader → rope) to avoid an additional “full file string” allocation.
- Opening a missing path SHOULD create an empty buffer with that path (new-file semantics).

### User feedback (target)

For large/very large files, the implementation SHOULD provide user-visible feedback:

- progress indicator for long reads
- ability to cancel

## Feature degradation model (target)

When a buffer is classified as large/very large, the implementation MAY degrade features to preserve responsiveness.

If degradation is applied, it MUST be explicit and deterministic (same inputs → same degradation decision):

| Feature area | Allowed degradation examples |
|--------------|-----------------------------|
| Syntax/UI | disable highlighting, reduce decorations |
| LSP | disable or cap request sizes |
| Search | disable counts, cap match lists, prefer incremental |
| Undo | cap history or coalesce more aggressively |

## Acceptance criteria (Given/When/Then)

1. Given a buffer with 1,000,000 lines, when a snapshot is produced, then the snapshot MUST contain only a viewport-sized slice of lines (not the full buffer).
2. Given a large file, when the editor is idle, then CPU usage SHOULD remain low (no continuous full redraw loop).
3. Given a large file, when the user scrolls, then the editor MUST remain responsive (no work proportional to total line count per scroll step).

## Related

- Latency/order guarantees: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Memory policy: [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
