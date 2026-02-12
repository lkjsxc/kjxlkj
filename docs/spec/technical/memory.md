# Memory and Allocation Policy

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
Define memory invariants that keep the editor stable and responsive as buffers and features grow.

This document is normative for memory behavior, but it intentionally focuses on structural guarantees rather than hard numeric limits.

## Definitions

| Term | Meaning |
|---|---|
| Resident memory | Long-lived allocations (buffers, indexes, undo state, caches). |
| Transient memory | Short-lived allocations in hot paths (per keystroke, per snapshot, per render). |
| Working set | Resident + expected transient usage under typical interaction. |
| Large buffer | As defined in [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md). |

## Core requirements (normative)

Memory usage invariants the implementation must satisfy.

### Avoid full-buffer copies in hot paths

The implementation MUST avoid allocating or copying the full buffer content as a `String` during routine interaction.

Examples of “routine interaction”:

- cursor motion
- scrolling
- Insert-mode typing
- snapshot generation
- rendering

Full-buffer materialization MAY exist only for explicit operations that semantically require it (e.g., “write entire buffer”), and SHOULD be avoided for large buffers when practical.

### Viewport-bounded snapshots MUST be memory-bounded

The snapshot is the core-to-render contract.

- Snapshot generation MUST be O(viewport) in allocation size.
- A snapshot MUST NOT contain “all buffer lines”.

This requirement is shared with (and motivated by):

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

### File open SHOULD avoid intermediate full copies

When opening a file from disk:

- The implementation SHOULD stream UTF-8 bytes into the text model (reader → text) to avoid an additional “full file string” allocation.
- Opening a missing path SHOULD create an empty buffer associated with that path (new-file semantics).

### Undo/redo MUST have a bounded growth model

Undo history MUST not require copying the full buffer for every edit step.

Allowed models include (non-exhaustive):

- persistent data structures with structural sharing
- edit-log replay with periodic checkpoints
- coalescing of fine-grained edits into larger, fewer steps

If an undo model can grow unbounded in the worst case, it MUST provide a deterministic cap strategy (prune/coalesce), and that cap MUST be recorded as a user-visible limitation if it changes observable behavior.

### Caches MUST be bounded and evictable

Any caches introduced by services (LSP, indexing, syntax, git, etc.) MUST:

- have explicit size bounds
- support deterministic eviction (e.g., LRU)
- be disable-able or more aggressively bounded for large buffers

### Observability SHOULD exist (target)

The system SHOULD provide a way (command, debug view, or log) to inspect coarse memory contributors:

- active buffer size characteristics (bytes, line count)
- undo history size (steps, estimated footprint)
- cache sizes (per service)

## Large/very large buffer posture (normative)

For large buffers, memory behavior MUST prioritize responsiveness over feature completeness:

- no per-frame allocations proportional to total buffer size
- no “hidden full scans” triggered by scrolling or typing

Any feature degradation used to preserve this MUST be explicit and recorded in conformance/limitations.

## Acceptance criteria (Given/When/Then)

1. Given a buffer with 1,000,000 lines, when a snapshot is produced, then the snapshot’s allocations MUST be bounded by viewport size (not total line count).
2. Given a large file, when the editor is idle, then transient allocations SHOULD remain near-zero (no continuous redraw loop that reallocates snapshot data).
3. Given repeated Insert-mode typing on a large buffer, when 1,000 characters are inserted, then memory usage MUST not increase by “one full-buffer copy” per character (no accidental per-keystroke full clones).

## Related

- Large files: [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Latency: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Testing: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
