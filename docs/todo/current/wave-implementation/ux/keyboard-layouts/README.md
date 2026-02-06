# UX: Keyboard Layouts (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ux/README.md](/docs/todo/current/wave-implementation/ux/README.md)

## Scope

Implement keyboard-layout related behaviors and constraints (where specified).

## Defining documents (direct, normative)

- Keyboard layouts:
  - [/docs/spec/ux/keyboard-layouts.md](/docs/spec/ux/keyboard-layouts.md)

## Checklist

- [x] Define how key decoding handles non-US layouts (or record limitations). — done: `keyboard_layout.rs` with LayoutRemapper for QWERTY/Dvorak/Colemak/Workman, `parse_layout()`
- [x] Add tests covering layout-dependent key decoding behavior. — done: 7 tests for remap, preserve_hjkl, custom remaps

