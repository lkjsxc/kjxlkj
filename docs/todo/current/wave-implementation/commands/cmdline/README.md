# Ex Commands: Command-Line (Iteration 34)

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

- [ ] Define the command-line model as part of core state and snapshots.
- [ ] Define the history storage and persistence expectations (if any).
- [ ] Define completion interfaces and data sources.

### B. Minimal conformance slice

- [ ] Implement `:` entry/exit and minimal editing.
- [ ] Implement stable history navigation and acceptance tests.

### C. Full conformance

- [ ] Implement all command-line behaviors defined by the cmdline subtree.
- [ ] Ensure command-line window interactions integrate with window/viewports. — done: cmdline_window.rs (core-edit) with CmdlineWindowState, CmdlineViewport, follow_cmdline_cursor, render_cmdline_window

### D. Conformance updates

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

