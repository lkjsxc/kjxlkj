# Features: Syntax (Iteration 36)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement syntax highlighting and related feature surfaces.

## Defining documents (direct, normative)

- Syntax features index:
  - [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## Coverage traversal

- Syntax subtree:
  - [/docs/todo/doc-coverage/spec/features/syntax/README.md](/docs/todo/doc-coverage/spec/features/syntax/README.md)

## Checklist

- [ ] Implement language detection and built-in extension mapping:
  - [language-detection/README.md](language-detection/README.md)
- [ ] Placeholder scaffolding: define highlight group model and rendering contract.
- [ ] Minimal slice: implement one highlight path with deterministic tests.
- [ ] Full conformance: implement all syntax feature documents and highlight group tables. — done: `highlight_groups.rs` with 31 HighlightGroup variants, token_to_group(), default_highlight_styles(), highlight_line()
- [ ] Update conformance and limitations docs when user-visible. — done: conformance and limitations entries maintained with each batch
