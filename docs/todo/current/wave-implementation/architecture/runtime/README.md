# Architecture: Runtime Ordering (Iteration 34)

Back: [/docs/todo/current/wave-implementation/architecture/README.md](/docs/todo/current/wave-implementation/architecture/README.md)

## Scope

Implement the async-first runtime model:

- host/core/render task boundaries
- snapshot production and render consumption
- service supervision and IO isolation

## Defining documents (direct, normative)

- Runtime model:
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## Checklist

- [x] Placeholder scaffolding: define the task graph and message channels.
- [x] Minimal slice: implement one event loop path with deterministic tests.
- [ ] Full conformance: implement all ordering rules and invariants in the runtime spec.
- [ ] Add tests proving ordering and failure recovery rules.

