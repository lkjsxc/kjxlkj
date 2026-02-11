# Wave 022: Live E2E and Race Validation

Back: [/docs/todo/waves/stage-02-editing-and-modes/README.md](/docs/todo/waves/stage-02-editing-and-modes/README.md)

## Wave Identity

- [x] Wave ID: `W022`
- [x] Stage: `Stage 02: Editing and Modes`
- [x] Focus: Live E2E and Race Validation
- [x] Stage scope understood: Modal behavior, editing primitives, cursor semantics, and text transformation determinism.

## Tier-A Critical Docs (Highest Priority, Re-read)

- [x] [/README.md](/README.md)
- [x] [/docs/README.md](/docs/README.md)
- [x] [/docs/policy/README.md](/docs/policy/README.md)
- [x] [/docs/policy/INSTRUCT.md](/docs/policy/INSTRUCT.md)
- [x] [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [x] [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [x] [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- [x] [/docs/spec/README.md](/docs/spec/README.md)
- [x] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [x] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [x] [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
- [x] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [x] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [x] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [x] [/docs/reference/README.md](/docs/reference/README.md)
- [x] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [x] [/docs/reference/CI.md](/docs/reference/CI.md)
- [x] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
- [x] [/docs/todo/README.md](/docs/todo/README.md)
- [x] [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Tier-B Stage Docs (Stage Priority, Re-read)

- [x] [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- [x] [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- [x] [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- [x] [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- [x] [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [x] [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- [x] [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

## Tier-C Wave Direct Docs (Coverage Slice)

- [x] [/docs/spec/editing/registers/special-registers.md](/docs/spec/editing/registers/special-registers.md)
- [x] [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- [x] [/docs/spec/editing/search/grep-integration.md](/docs/spec/editing/search/grep-integration.md)
- [x] [/docs/spec/editing/search/live-grep.md](/docs/spec/editing/search/live-grep.md)
- [x] [/docs/spec/editing/search/search-commands.md](/docs/spec/editing/search/search-commands.md)
- [x] [/docs/spec/editing/search/search-count.md](/docs/spec/editing/search/search-count.md)
- [x] [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)

## Implementation Tasks

- [x] extract and normalize all normative requirements from Tier-A/B/C docs
- [x] map requirements to concrete modules and public contracts
- [x] define deterministic command/key entry paths for touched behaviors
- [x] implement reachable user-visible behavior before convenience paths
- [x] enforce explicit error handling and deterministic fallback paths
- [x] preserve topology and file-size constraints during implementation
- [x] avoid stub-only or dead-path completion claims
- [x] split modules/files early when growth risk appears
- [x] keep behavior reproducible across repeated runs and scripts
- [x] document unresolved contradictions as explicit drift rows

## Verification Tasks

- [x] add or update failing regression tests for touched requirements
- [x] run unit and integration suites for touched modules
- [x] run required `*R` E2E cases for touched blocker surfaces
- [x] dump state after each key input in blocker E2E tests
- [x] assert screen output rows, focus, cursor/caret, and pane geometry
- [x] run at least one boundary/race scenario relevant to this wave
- [x] capture deterministic failure artifacts for failing cases
- [x] rerun key scripts to verify deterministic replay equivalence
- [x] record exact command outputs and pass/fail signals for evidence

## Ledger Sync Tasks

- [x] update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) with strongest evidence only
- [x] update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) for open/closed rows
- [x] update [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) row statuses and classes
- [x] ensure wave completion checkboxes match ledger truth
- [x] ensure no stale green claims remain after contradictory evidence

## Wave Exit

- [x] all Tier-A/B/C docs above were read directly
- [x] implementation tasks are complete or explicitly deferred with rationale
- [x] verification tasks are complete with deterministic evidence
- [x] ledger sync tasks are complete in the same logical closure change
- [x] continue: [/docs/todo/waves/stage-02-editing-and-modes/wave-023.md](/docs/todo/waves/stage-02-editing-and-modes/wave-023.md)
