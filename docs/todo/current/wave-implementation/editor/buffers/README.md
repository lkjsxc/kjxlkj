# Editor: Buffers (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editor/README.md](/docs/todo/current/wave-implementation/editor/README.md)

## Scope

Implement the buffer model and buffer lifecycle invariants.

## Defining documents (direct, normative)

- Buffers:
  - [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)

## Checklist

- [x] Placeholder scaffolding: define buffer identity, metadata, and text ownership.
- [x] Minimal slice: implement open/edit/write for one buffer with tests.
- [x] Full conformance: implement all buffer behaviors defined by the spec and command subtrees.
  - buffer_full.rs: BufferType (Normal/Scratch/Help/QuickFix/Terminal/Prompt/Popup), BufferFlags, BufferInfo, AlternateTracker (switch_to/swap), filter_listed, find_by_name, modified_count
- [x] Update conformance and limitations docs when user-visible. — done: conformance and limitations entries maintained with each batch
