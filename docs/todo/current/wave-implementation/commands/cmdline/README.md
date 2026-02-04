# Ex Commands: Command-Line (Iteration 33)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement command-line entry, editing, history, completion, and command-line window behavior.

## Defining documents (direct, normative)

- Command-line index:
  - [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)
- Core command-line behaviors:
  - [/docs/spec/commands/cmdline/cmdline-entry.md](/docs/spec/commands/cmdline/cmdline-entry.md)
  - [/docs/spec/commands/cmdline/cmdline-editing.md](/docs/spec/commands/cmdline/cmdline-editing.md)
  - [/docs/spec/commands/cmdline/cmdline-history.md](/docs/spec/commands/cmdline/cmdline-history.md)
  - [/docs/spec/commands/cmdline/completion.md](/docs/spec/commands/cmdline/completion.md)
  - [/docs/spec/commands/cmdline/cmdline-window.md](/docs/spec/commands/cmdline/cmdline-window.md)

## Coverage traversal

- Command-line subtree:
  - [/docs/todo/doc-coverage/spec/commands/cmdline/README.md](/docs/todo/doc-coverage/spec/commands/cmdline/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define the command-line model as part of core state and snapshots.
- [x] Define the history storage and persistence expectations (if any).
- [x] Define completion interfaces and data sources.

### B. Minimal conformance slice

- [x] Implement `:` entry/exit and minimal editing.
- [x] Implement stable history navigation and acceptance tests.

### C. Full conformance

- [x] Implement all command-line behaviors defined by the cmdline subtree.
- [x] Ensure command-line window interactions integrate with window/viewports.

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

