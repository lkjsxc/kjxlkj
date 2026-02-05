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

- [x] Ensure the text model supports large buffers efficiently (or record limitations).
- [x] Add targeted tests for large-buffer edits (insert/delete) without full-buffer cloning.

### B. Snapshot bounds and allocations

- [x] Ensure snapshots are viewport-bounded and do not scale with total buffer size.
- [x] Add deterministic regression tests that assert viewport-bounded materialization.

### C. Large-file I/O posture (user-reported slowness)

- [x] Ensure file open avoids intermediate full-file copies where possible (streaming into text model).
- [ ] Add a repeatable benchmark for large-file open and initial snapshot time-to-first-render.

### D. Extremely long lines

- [x] Add deterministic regression tests for extremely long lines (memory + rendering posture).
