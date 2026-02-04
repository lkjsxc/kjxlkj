# Ex Commands: Execution Model (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement Ex command execution, including `:execute`, `:normal`, and `:source` behavior and the shared execution pipeline.

## Defining documents (direct, normative)

- Execution index:
  - [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)
- Execution commands:
  - [/docs/spec/commands/execution/execute-command.md](/docs/spec/commands/execution/execute-command.md)
  - [/docs/spec/commands/execution/normal-command.md](/docs/spec/commands/execution/normal-command.md)
  - [/docs/spec/commands/execution/source-command.md](/docs/spec/commands/execution/source-command.md)

## Coverage traversal

- Execution subtree:
  - [/docs/todo/doc-coverage/spec/commands/execution/README.md](/docs/todo/doc-coverage/spec/commands/execution/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define the execution pipeline:
  - parse → resolve → execute → report
- [x] Define script/source file loading and error handling rules.

### B. Minimal conformance slice

- [ ] Implement `:source` for a minimal, deterministic script subset (as specified).
- [ ] Add tests proving deterministic execution ordering.

### C. Full conformance

- [ ] Implement all execution commands and semantics in the subtree.
- [ ] Ensure execution integrates with mappings/macros as specified.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

