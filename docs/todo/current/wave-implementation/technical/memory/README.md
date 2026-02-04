# Technical: Memory (Iteration 33)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement memory and large-data behavior constraints.

## Defining documents (direct, normative)

- Memory:
  - [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- Large files guidance:
  - [/docs/technical/large-files.md](/docs/technical/large-files.md)

## Checklist

- [x] Ensure the text model supports large buffers efficiently (or record limitations).
- [x] Ensure snapshots are bounded and do not scale with terminal size unnecessarily.
- [x] Add regression tests for large file/long line scenarios (deterministic).

