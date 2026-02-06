# Technical: Memory (Iteration 34)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement memory and large-data behavior constraints.

## Defining documents (direct, normative)

- Memory:
  - [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- Large files guidance:
  - [/docs/technical/large-files.md](/docs/technical/large-files.md)

## Checklist

### A. Text model and large buffers

- [x] Ensure the text model supports large buffers efficiently (or record limitations). — done: `large_buffer.rs` with LoadStrategy (Full/Chunked/Streamed), chunk computation, line index, extract_line_range
- [x] Add targeted tests for large-buffer edits (insert/delete) without full-buffer cloning. — done: 9 tests for strategy selection, line indexing, chunk computation, line extraction

### B. Snapshot bounds and allocations

- [x] Ensure snapshots are viewport-bounded and do not scale with total buffer size. — done: BufferSnapshot only materializes viewport rows
- [x] Add deterministic regression tests that assert viewport-bounded materialization. — done: `memory_latency_probes.rs` and `long_line_rendering.rs` assert snapshot sizes

### C. Large-file I/O posture (user-reported slowness)

- [ ] Ensure file open avoids intermediate full-file copies where possible (streaming into text model).
- [ ] Add a repeatable benchmark for large-file open and initial snapshot time-to-first-render.

### D. Extremely long lines

- [x] Add deterministic regression tests for extremely long lines (memory + rendering posture). — done: 13 tests in `long_line_rendering.rs` including Unicode, mixed lengths, stress tests
