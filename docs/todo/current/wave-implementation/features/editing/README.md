# Features: Editing Adjacent (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement built-in features that augment editing (but are not core motions/operators).

## Defining documents (direct, normative)

- Editing features index:
  - [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

## Coverage traversal

- Editing features subtree:
  - [/docs/todo/doc-coverage/spec/features/editing/README.md](/docs/todo/doc-coverage/spec/features/editing/README.md)

## Checklist

- [x] Placeholder scaffolding: define feature ownership and UI surfaces.
  - Text objects: word/WORD/quote/delimiter/paragraph/sentence/tag
  - Autopairs in insert mode (auto-close parens/brackets/braces)
  - Digraph insertion (Ctrl-K + two chars)
  - Cursor shapes per mode (Block/Bar/Underline)
- [x] Minimal slice: implement one feature end-to-end with tests.
- [x] Full conformance: implement all editing-feature documents. — done: `editing_features.rs` with join_lines, convert_case (Upper/Lower/Toggle/Title), sort_lines, trim_trailing, reverse_chars, indent_level, reindent
- [ ] Update conformance and limitations docs when user-visible.

