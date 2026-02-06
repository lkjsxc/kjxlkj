# UI: Views (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement view composition (editor, command line, popups, terminal panes).

## Defining documents (direct, normative)

- UI views:
  - [/docs/spec/ui/views.md](/docs/spec/ui/views.md)

## Checklist

- [x] Placeholder scaffolding: define view layout and focus model.
  - view_tree.rs: FocusTarget, ViewNode, ViewTree with focus stack
- [x] Minimal slice: implement editor view + command line view with tests.
  - from_splits() builds tab/buffer/status/cmdline layout, find() by ComponentId
- [ ] Full conformance: implement all view types and composition rules.

