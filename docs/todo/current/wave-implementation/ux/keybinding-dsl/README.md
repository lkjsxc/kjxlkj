# UX: Keybinding DSL (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ux/README.md](/docs/todo/current/wave-implementation/ux/README.md)

## Scope

Implement the keybinding DSL and its interpretation rules.

## Defining documents (direct, normative)

- Keybinding DSL:
  - [/docs/spec/ux/keybinding-dsl.md](/docs/spec/ux/keybinding-dsl.md)

## Checklist

- [ ] Placeholder scaffolding: define the DSL parse model and validation rules.
  - keybinding_dsl.rs: KeyChord struct, parse_angle(), resolve_special()
- [ ] Minimal slice: implement a small DSL subset with deterministic tests.
  - parse_key_sequence() for <C-x>, <M-a>, <leader>, <CR>, <Esc>, combined modifiers, validate_key_sequence()
- [ ] Full conformance: implement the entire DSL as specified.
  - keybinding_dsl.rs: SpecialKey (16 keys + F1-F12), Modifiers (ctrl/alt/shift/meta), KeySpec, KeyChord, parse_key_notation, parse_key_sequence

