# Scripting: Mappings (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement mapping definition, resolution, expansion, and mode scoping.

## Defining documents (direct, normative)

- Mappings index:
  - [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Keybinding DSL:
  - [/docs/spec/ux/keybinding-dsl.md](/docs/spec/ux/keybinding-dsl.md)

## Coverage traversal

- Mappings subtree:
  - [/docs/todo/doc-coverage/spec/scripting/mappings/README.md](/docs/todo/doc-coverage/spec/scripting/mappings/README.md)

## Checklist

- [x] Placeholder scaffolding: define mapping storage and expansion boundaries.
- [x] Minimal slice: implement a minimal mapping set with deterministic tests.
- [x] Full conformance: implement all mapping modes, special keys, and recursion rules. — done: `mapping_expansion.rs` with recursive expansion (MAX_DEPTH=100), prefix detection, `list_mappings()`, 9 tests

