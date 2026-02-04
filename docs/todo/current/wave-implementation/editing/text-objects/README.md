# Editing: Text Objects (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement text object selection semantics and interaction with operators and Visual mode.

## Defining documents (direct, normative)

- Text objects index:
  - [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

## Coverage traversal

- Text objects subtree:
  - [/docs/todo/doc-coverage/spec/editing/text-objects/README.md](/docs/todo/doc-coverage/spec/editing/text-objects/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define text object parsing and selection models.
- [ ] Define inclusive/exclusive and linewise behaviors when specified.

### B. Minimal conformance slice

- [ ] Implement a minimal text object set with deterministic tests.

### C. Full conformance

- [ ] Implement all text object documents in the subtree.
  - Word objects: iw, aw, iW, aW
  - Quote objects: i", a", i', a', i`, a`
  - Block objects: i(, a(, i[, a[, i{, a{, i<, a<, ib, ab, iB, aB
  - Tag objects: it, at

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

