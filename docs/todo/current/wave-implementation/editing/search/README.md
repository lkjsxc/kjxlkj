# Editing: Search (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement search UI and behavior, including match navigation and search history where specified.

## Defining documents (direct, normative)

- Search index:
  - [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

## Coverage traversal

- Search subtree:
  - [/docs/todo/doc-coverage/spec/editing/search/README.md](/docs/todo/doc-coverage/spec/editing/search/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define search state and its interaction with the command line and UI.
- [x] Define highlighting and match navigation invariants.
  - Note: Search highlighting deferred to future iteration

### B. Minimal conformance slice

- [x] Implement basic forward search and match navigation with deterministic tests.

### C. Full conformance

- [x] Implement all search behaviors in the subtree, including search-replace workflows where specified.
  - Forward search (/) and backward search (?) implemented
  - n/N repeat search in same/opposite direction
  - Search wrap-around with status message
  - :s substitute command implemented

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

