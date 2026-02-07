# Editing: Motions (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement motion parsing and motion behavior for navigation and operator targets.

## Defining documents (direct, normative)

- Motions index:
  - [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

## Coverage traversal

- Motions subtree:
  - [/docs/todo/doc-coverage/spec/editing/motions/README.md](/docs/todo/doc-coverage/spec/editing/motions/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Implement motion grammar parsing and motion typing (inclusive/exclusive, linewise/charwise).
- [ ] Define count application rules and repeat mechanics where specified.

### B. Minimal conformance slice

- [ ] Implement a small set of core motions with deterministic tests.

### C. Full conformance

- [ ] Implement all motion documents in the subtree, including:
  - scrolling and viewport-sensitive motions
  - search motions (/, ?, n, N)
  - jump/mark motions (`, ', g;, g,, Ctrl-o, Ctrl-i)
  - sentence/paragraph motions ((, ), {, })
  - match bracket motion (%)

### D. Conformance updates

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

