# Implementation: UX and Keybindings (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement user-facing behavior and interactions that tie subsystems together:

- keybinding coverage and discoverability
- layout expectations and navigation flows
- accessibility requirements
- theming and keyboard-only constraints

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Keybindings reference | [keybindings/README.md](keybindings/README.md) |
| Keybinding DSL | [keybinding-dsl/README.md](keybinding-dsl/README.md) |
| Layout | [layout/README.md](layout/README.md) |
| Accessibility | [accessibility/README.md](accessibility/README.md) |
| Theming | [theming/README.md](theming/README.md) |
| Keyboard layouts | [keyboard-layouts/README.md](keyboard-layouts/README.md) |

## Read first (direct, normative)

- UX index:
  - [/docs/spec/ux/README.md](/docs/spec/ux/README.md)
- Keybindings:
  - [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
  - [/docs/spec/ux/keybinding-dsl.md](/docs/spec/ux/keybinding-dsl.md)
- Accessibility:
  - [/docs/spec/ux/accessibility.md](/docs/spec/ux/accessibility.md)
- Layout:
  - [/docs/spec/ux/layout.md](/docs/spec/ux/layout.md)

## Coverage traversal

- UX subtree:
  - [/docs/todo/doc-coverage/spec/ux/README.md](/docs/todo/doc-coverage/spec/ux/README.md)

## Placeholder scaffolding (sub-wave)

- [x] Create an explicit keybinding coverage map that: — done: ux_coverage.rs (input) with CoverageEntry, CoverageSummary, build_normal_coverage, build_insert_coverage
  - lists the implemented keys
  - links each key to the owning spec document(s)
  - links each key to tests that prove the behavior

## Minimal conformance slice (sub-wave)

- [x] Ensure the minimal core editing surface is fully covered by: — done: ux_coverage.rs (input) with compute_summary showing tested/documented counts
  - keybindings
  - documentation pointers
  - tests

## Full conformance (sub-wave)

- [x] Implement all keybindings and UX behavior defined by the UX subtree. — done: keybinding_tables.rs (60+ bindings), mode_keybindings.rs, leader_keys.rs, ux_coverage.rs
- [x] Ensure keyboard-only invariant is never violated. — done: ux_coverage.rs keyboard_only_check, layout_acceptance.rs invariants
- [x] Ensure accessibility requirements are met (or explicitly limited). — done: `accessibility.rs` WCAG contrast, focus visibility, ARIA hints, 7 tests

## Tests (normative outputs)

- [x] Add tests for: — done: ux_coverage.rs, keybinding_tables.rs, mode_keybindings.rs, leader_keys.rs all have tests
  - keybinding → action mapping determinism
  - mode-scoped keybinding behavior
  - accessibility-related UI invariants (focus, contrast, cursor visibility)

## Conformance and limitations (required updates)

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
