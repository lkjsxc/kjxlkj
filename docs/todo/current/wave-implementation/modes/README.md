# Implementation: Modes (Iteration 33)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement modal editing semantics and mode transitions:

- Normal, Insert, Visual, Replace, Command
- mode entry/exit rules
- mode-specific cursor models and selections
- mode-specific key interpretation and counts

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Normal mode | [normal/README.md](normal/README.md) |
| Insert mode | [insert/README.md](insert/README.md) |
| Visual mode | [visual/README.md](visual/README.md) |
| Replace mode | [replace/README.md](replace/README.md) |
| Command mode | [command/README.md](command/README.md) |
| Mode transitions | [transitions/README.md](transitions/README.md) |
| Mode configuration | [configuration/README.md](configuration/README.md) |

## Read first (direct, normative)

- Modes index:
  - [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- Individual modes:
  - [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
  - [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
  - [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
  - [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
  - [/docs/spec/modes/command.md](/docs/spec/modes/command.md)
  - [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- UX keybinding reference:
  - [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)

## Coverage traversal

- Modes subtree:
  - [/docs/todo/doc-coverage/spec/modes/README.md](/docs/todo/doc-coverage/spec/modes/README.md)

## Placeholder scaffolding (sub-wave)

- [ ] Define a single, explicit mode state machine with:
  - mode enum/state
  - pending operator/motion state (when applicable)
  - count prefix model
  - per-mode cursor/selection representation
- [ ] Define command-line entry and editing model boundaries.

## Minimal conformance slice (sub-wave)

- [x] Implement a minimal, fully tested set of mode transitions:
  - Normal ↔ Insert
  - Normal ↔ Visual (charwise)
  - Normal ↔ Command
  - Normal ↔ Replace (as defined, not “Insert-like” unless specified)
- [x] Ensure cursor and selection invariants hold across transitions.

## Full conformance (sub-wave)

- [ ] Implement full Insert/Replace/Visual semantics per spec, including sub-features under Insert mode.
- [ ] Implement configuration impacts on modes (where specified).
- [ ] Ensure mode behavior matches UX keybindings tables.

## Tests (normative outputs)

- [ ] Add tests for:
  - mode transition graphs
  - pending state cancellation rules
  - selection model correctness for Visual modes

## Conformance and limitations (required updates)

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
