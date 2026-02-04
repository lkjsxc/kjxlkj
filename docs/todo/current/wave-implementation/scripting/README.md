# Implementation: Scripting and Automation (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the scripting surface defined by the spec (without introducing plugins):

- mappings and keybinding DSL
- user commands and command-line completion hooks
- event automation and debouncing rules
- script files (configuration and automation)

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Mappings | [mappings/README.md](mappings/README.md) |
| Command-line completion hooks | [cmdline-completion/README.md](cmdline-completion/README.md) |
| Event automation | [event-automation/README.md](event-automation/README.md) |
| Script files | [script-files/README.md](script-files/README.md) |
| Timing and debounce | [timing-debounce/README.md](timing-debounce/README.md) |
| User commands | [user-commands/README.md](user-commands/README.md) |
| User functions | [user-functions/README.md](user-functions/README.md) |

## Read first (direct, normative)

- Scripting index:
  - [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md)
- Mappings:
  - [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Keybinding DSL:
  - [/docs/spec/ux/keybinding-dsl.md](/docs/spec/ux/keybinding-dsl.md)

## Coverage traversal

- Scripting subtree:
  - [/docs/todo/doc-coverage/spec/scripting/README.md](/docs/todo/doc-coverage/spec/scripting/README.md)

## Placeholder scaffolding (sub-wave)

- [x] Define the mapping data model and resolution order.
- [x] Define the boundary between:
  - raw input decoding
  - mapping expansion
  - core intent emission

## Minimal conformance slice (sub-wave)

- [x] Implement a minimal subset of mappings that is:
  - deterministic
  - test-backed
  - sufficient to cover core navigation/editing paths

## Full conformance (sub-wave)

- [x] Implement the full mapping and automation spec subtree.
- [x] Ensure recursion/plug mappings are handled exactly as specified (or recorded as limitations).

## Tests (normative outputs)

- [x] Add tests for:
  - mapping precedence and mode scoping
  - recursion limits and safety rules
  - timing/debounce determinism

## Conformance and limitations (required updates)

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
