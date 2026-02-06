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

- [x] Implement the full mapping and automation spec subtree. — done: mapping_expansion.rs (recursive expansion), event_automation.rs (event dispatch), script_loader.rs (file parsing)
- [x] Ensure recursion/plug mappings are handled exactly as specified (or recorded as limitations). — done: `mapping_expansion.rs` MAX_DEPTH=100 recursion guard, prefix match

## Tests (normative outputs)

- [x] Add tests for:
  - mapping precedence and mode scoping — done: `mapping_expansion.rs` mode-scoped entries, 9 tests
  - recursion limits and safety rules — done: `expand_recursive()` MAX_DEPTH guard test
  - timing/debounce determinism — done: `debounce_exec.rs` FakeClock, 7 tests

## Conformance and limitations (required updates)

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
