# Ex Commands: Essential (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement the essential Ex command surface and shared rules that apply to all commands.

## Defining documents (direct, normative)

- Essential commands:
  - [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)
- Command execution model (shared rules):
  - [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

## Coverage traversal

- Essential file coverage is part of the commands subtree:
  - [/docs/todo/doc-coverage/spec/commands/README.md](/docs/todo/doc-coverage/spec/commands/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define shared command parsing and execution interfaces.
- [ ] Define shared error and message reporting rules.

### B. Minimal conformance slice

- [ ] Implement a minimal set of essential commands required by:
  - core file open/write workflows
  - safe quit flows
- [ ] Add deterministic tests for parsing and error cases.

### C. Full conformance

- [ ] Implement all essential commands exactly as specified.
- [ ] Ensure shared rules are applied consistently across all command families.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

