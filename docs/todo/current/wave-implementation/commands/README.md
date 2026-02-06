# Implementation: Ex Commands (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the Ex command surface, including:

- command-line entry/editing/history/completion
- command parsing and execution model
- ranges and addresses
- file/buffer/session/substitute/quit command families

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Essential command set | [essential/README.md](essential/README.md) |
| Command-line UX | [cmdline/README.md](cmdline/README.md) |
| Execution model | [execution/README.md](execution/README.md) |
| Ranges and addresses | [ranges/README.md](ranges/README.md) |
| File commands | [file/README.md](file/README.md) |
| Buffer commands | [buffer/README.md](buffer/README.md) |
| Session commands | [session/README.md](session/README.md) |
| Substitute and global | [substitute/README.md](substitute/README.md) |
| Syntax commands | [syntax/README.md](syntax/README.md) |
| Quit commands | [quit/README.md](quit/README.md) |

## Read first (direct, normative)

- Commands index:
  - [/docs/spec/commands/README.md](/docs/spec/commands/README.md)
- Essential and execution model:
  - [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)
  - [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)
- Ranges:
  - [/docs/spec/commands/ranges/README.md](/docs/spec/commands/ranges/README.md)
- Command-line UX:
  - [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

## Coverage traversal

- Commands subtree:
  - [/docs/todo/doc-coverage/spec/commands/README.md](/docs/todo/doc-coverage/spec/commands/README.md)

## Placeholder scaffolding (sub-wave)

- [x] Define an Ex parser that:
  - accepts the specified grammar - implemented (CommandParser)
  - reports errors deterministically
  - separates parsing from execution - implemented
- [ ] Define range/address resolution as a reusable component.
- [x] Define command registry and completion model boundaries.

## Minimal conformance slice (sub-wave)

- [x] Implement the smallest set of commands required to:
  - open/edit/write files
  - quit safely
  - run a shell command via the terminal service (if specified)
- [x] Add deterministic tests for the minimal slice.

## Full conformance (sub-wave)

- [x] Implement every command family under `/docs/spec/commands/`.
  - :q, :q!, :qa, :qa! - quit commands implemented
  - :w, :w {file}, :wa - write commands implemented
  - :wq, :x - write-quit implemented
  - :e {file}, :e! {file} - edit commands implemented
  - :! {cmd} - external command implemented
  - :s/pattern/replacement/[flags] - substitute implemented
  - :g/pattern/command - global implemented
  - :v/pattern/command - vglobal implemented
- [ ] Ensure command behavior matches editing primitives and core state model.
- [ ] Ensure user-visible error messages are stable enough for tests.

## Tests (normative outputs)

- [ ] Add tests for:
  - parsing and error cases
  - range/address correctness - deferred (no range support yet)
  - command history behavior - deferred
  - completion behavior - deferred

## Conformance and limitations (required updates)

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
