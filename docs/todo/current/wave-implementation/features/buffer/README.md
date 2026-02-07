# Features: Buffer (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement buffer-related built-in features beyond the core buffer model.

## Defining documents (direct, normative)

- Buffer features index:
  - [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

## Coverage traversal

- Buffer features subtree:
  - [/docs/todo/doc-coverage/spec/features/buffer/README.md](/docs/todo/doc-coverage/spec/features/buffer/README.md)

## Checklist

- [ ] Placeholder scaffolding: define owning module(s) and snapshot/UI hooks.
- [ ] Minimal slice: implement one feature end-to-end with tests.
  - Alternate buffer (Ctrl-^, :b#, :b N)
  - Scratch buffers (:scratch)
- [ ] Full conformance: implement all documents in the subtree.
  - buffer_features.rs: BufferVariables, BufferLocalOptions, FileFormat, BufEvent, AutoCmd, AutoCmdRegistry
- [ ] Update conformance and limitations docs when behavior becomes user-visible. — done: conformance and limitations entries maintained with each batch

