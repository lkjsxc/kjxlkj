# Modes: Command (Iteration 35)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement the Command mode state machine and its integration with Ex command-line UX and execution.

## Defining documents (direct, normative)

- Command mode spec:
  - [/docs/spec/modes/command.md](/docs/spec/modes/command.md)
- Command-line UX:
  - [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define how `:` enters command-line mode and how it exits.

### B. Minimal conformance slice

- [ ] Implement basic `:` entry, editing, execute, and exit with deterministic tests.
  - `:` enters command mode
  - Character input builds command
  - Backspace deletes characters
  - Enter executes command
  - Escape cancels and returns to Normal

### C. Full conformance

- [ ] Implement all Command mode behaviors and integrate completion/history fully.
  - [ ] Basic command editing
  - [ ] Command history (up/down arrows)
  - [ ] Tab completion

If any sub-item is intentionally deferred, apply the deferral protocol from [/docs/todo/current/README.md](/docs/todo/current/README.md) and avoid leaving a terminal “future” section with unchecked boxes.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
