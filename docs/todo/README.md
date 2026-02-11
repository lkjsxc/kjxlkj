# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the authoritative reconstruction execution contract.

Only this file and `/docs/todo/waves/` are part of the TODO system.

## Start Here

- [ ] read this file fully before opening any wave file
- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] follow stages and waves recursively in strict order
- [ ] do not skip a wave with unchecked required items
- [ ] complete implementation only through wave-driven closure

## Critical Documents (Always Re-read)

- [ ] [/README.md](/README.md)
- [ ] [/docs/README.md](/docs/README.md)
- [ ] [/docs/policy/README.md](/docs/policy/README.md)
- [ ] [/docs/policy/INSTRUCT.md](/docs/policy/INSTRUCT.md)
- [ ] [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [ ] [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [ ] [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- [ ] [/docs/spec/README.md](/docs/spec/README.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
- [ ] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] [/docs/reference/README.md](/docs/reference/README.md)
- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
- [ ] [/docs/todo/README.md](/docs/todo/README.md)
- [ ] [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Recursive Rule

- [ ] each stage README points to ordered wave files
- [ ] each wave points to the exact next wave or next stage README
- [ ] if a wave cannot be closed, remain in that wave and resolve blockers first

## Completion Rule

- [ ] all waves in all stages are checked in order
- [ ] all high-severity limitations are closed
- [ ] all required `*R` E2E cases pass with screen-state assertions
- [ ] conformance, limitations, and drift ledgers are synchronized
- [ ] every documentation file has direct links inside TODO wave docs
