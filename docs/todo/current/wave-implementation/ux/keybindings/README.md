# UX: Keybindings (Iteration 36)

Back: [/docs/todo/current/wave-implementation/ux/README.md](/docs/todo/current/wave-implementation/ux/README.md)

## Scope

Implement keybinding coverage and ensure it matches mode behavior and command surfaces.

## Defining documents (direct, normative)

- Keybindings index:
  - [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
- Keybinding sections:
  - [/docs/spec/ux/keybindings/README.md](/docs/spec/ux/keybindings/README.md)

## Coverage traversal

- Keybindings subtree:
  - [/docs/todo/doc-coverage/spec/ux/keybindings/README.md](/docs/todo/doc-coverage/spec/ux/keybindings/README.md)

## Checklist

- [ ] Follow the leader-key checklist:
  - [leader/README.md](leader/README.md)
- [x] Define a keybinding coverage map tied to tests and conformance. — done: `keybinding_coverage.rs` with CoverageMap (add/untested/undocumented/for_mode/coverage_pct/find_duplicates), build_default_normal_coverage()
- [ ] Minimal slice: ensure all currently implemented keys are documented and tested.
- [x] Full conformance: implement the full keybinding tables (or record limitations explicitly). — done: `keybinding_tables.rs` (input) with ActionCategory (12 categories), BindingTable, build_normal_table (60+ bindings), coverage_stats
