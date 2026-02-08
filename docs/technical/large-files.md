# Large File Performance (Implementation)

Back: [/docs/technical/README.md](/docs/technical/README.md)
This document explains how kjxlkj stays responsive on large buffers in practice.

Normative requirements live in: [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

## Goal

Keep “edit a huge file” operations bounded by what the user can see (the viewport), not by total file size.

This is an implementation document: it describes the intended tactics used to satisfy the spec.

## What “large” means here

Use the spec’s definitions and stressors (bytes, line count, extremely long lines, heavy non-ASCII):

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

Implementation guidance: treat extremely long lines as “large” even when total file bytes are modest.

## Current strategy (high-level)

The architecture is “input → core → snapshot → render”. Large-file performance comes from ensuring the snapshot is cheap.

### 1. Viewport-bounded snapshots (core)

The core produces immutable snapshots for the renderer.

To avoid per-frame work proportional to total file size:

- a snapshot contains only a viewport-sized slice of lines (not “all buffer lines”)
- snapshot generation must not iterate the entire buffer
- allocations per snapshot are proportional to visible rows (plus small constant metadata)

This is the single most important invariant for large files.

### 2. Event-driven rendering (host)

The host render loop should avoid continuous redraws when idle.

In practice:

- render once on startup
- then render only after input events (keys) and terminal resize events

This prevents “idle CPU burn” on large files (where even a cheap snapshot still costs something).

### 3. Streaming file open (I/O)

Opening a file should avoid intermediate “full file string” allocations.

In practice:

- prefer reader → rope/text, rather than read-to-string → rope/text
- missing paths behave like “new file”: create an empty buffer whose path is the requested path

This reduces peak memory and time-to-first-interaction for large files.

## Complexity expectations (informal)

This section is descriptive, not normative.

| Operation | Expected scaling | Notes |
|---|---|---|
| Snapshot generation | O(viewport height) | must not enumerate all lines |
| Render | O(viewport width × height) | cell-based terminal output |
| Scroll by 1 line | O(viewport height) | new visible slice; depends on line access cost |
| Open file | O(file bytes) | single pass through input |

## Known stressors and current posture

Factors that stress large file handling.

### Extremely long lines

Even with viewport-bounded line counts, a single very long line can be expensive if the renderer must measure or slice deep into it.

Current posture:

- keep snapshot line count bounded
- treat long-line optimizations as future work (see below)

### Non-ASCII and wide graphemes

Display width computation is a renderer concern. The core should not precompute per-grapheme width for the entire buffer.

## What is intentionally not promised here

These are common “large file editor” behaviors, but they are not automatically assumed:

- no dedicated “large file mode” toggle
- no guaranteed feature degradation (syntax/LSP/undo caps) unless explicitly implemented
- no progress indicator or cancel during open unless explicitly implemented

If any of the above are added, they should be specified first in:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) and [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## How to verify (recommended)

Approaches for testing large file performance.

### Automated

- Add/keep regression tests that assert snapshot line materialization is viewport-sized.
- Add/keep regression tests for “idle render does not busy-loop” where feasible.

### Manual

- Open a very large file and confirm:
  - the editor becomes interactive quickly
  - CPU usage stays low when not typing or resizing
  - scrolling does not trigger “whole-file” pauses

## Future improvements (targets)

These are compatible with the current model and can be implemented incrementally:

- terminal diffing (dirty-region / cell-diff) to reduce writes
- incremental snapshot caching for unchanged lines within a stable viewport
- long-line virtualization (render only the visible slice of a single long line)
- optional progress + cancel for long reads

## Related

- Spec: [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Latency: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Memory: [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- Viewport: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
