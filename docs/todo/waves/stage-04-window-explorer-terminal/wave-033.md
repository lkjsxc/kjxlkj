# Wave 033: Requirement Extraction and Normalization

Back: [/docs/todo/waves/stage-04-window-explorer-terminal/README.md](/docs/todo/waves/stage-04-window-explorer-terminal/README.md)

## Wave Identity

- [ ] Wave ID: `W033`
- [ ] Stage: `Stage 04: Window, Explorer, Terminal`
- [ ] Focus: Requirement Extraction and Normalization
- [ ] Stage scope understood: Split lifecycle, mixed-pane focus, explorer behavior, and terminal integration.

## Tier-A Critical Docs (Highest Priority, Re-read)

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

## Tier-B Stage Docs (Stage Priority, Re-read)

- [ ] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- [ ] [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- [ ] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- [ ] [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)
- [ ] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- [ ] [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)

## Tier-C Wave Direct Docs (Coverage Slice)

- [ ] [/docs/spec/features/git/git.md](/docs/spec/features/git/git.md)
- [ ] [/docs/spec/features/git/gitsigns.md](/docs/spec/features/git/gitsigns.md)
- [ ] [/docs/spec/features/git/merge-conflicts.md](/docs/spec/features/git/merge-conflicts.md)
- [ ] [/docs/spec/features/git/vimdiff.md](/docs/spec/features/git/vimdiff.md)
- [ ] [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- [ ] [/docs/spec/features/lsp/code-actions.md](/docs/spec/features/lsp/code-actions.md)
- [ ] [/docs/spec/features/lsp/code-lens.md](/docs/spec/features/lsp/code-lens.md)

## Implementation Tasks

- [ ] extract and normalize all normative requirements from Tier-A/B/C docs
- [ ] map requirements to concrete modules and public contracts
- [ ] define deterministic command/key entry paths for touched behaviors
- [ ] implement reachable user-visible behavior before convenience paths
- [ ] enforce explicit error handling and deterministic fallback paths
- [ ] preserve topology and file-size constraints during implementation
- [ ] avoid stub-only or dead-path completion claims
- [ ] split modules/files early when growth risk appears
- [ ] keep behavior reproducible across repeated runs and scripts
- [ ] document unresolved contradictions as explicit drift rows

## Verification Tasks

- [ ] add or update failing regression tests for touched requirements
- [ ] run unit and integration suites for touched modules
- [ ] run required `*R` E2E cases for touched blocker surfaces
- [ ] dump state after each key input in blocker E2E tests
- [ ] assert screen output rows, focus, cursor/caret, and pane geometry
- [ ] run at least one boundary/race scenario relevant to this wave
- [ ] capture deterministic failure artifacts for failing cases
- [ ] rerun key scripts to verify deterministic replay equivalence
- [ ] record exact command outputs and pass/fail signals for evidence

## Ledger Sync Tasks

- [ ] update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) with strongest evidence only
- [ ] update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) for open/closed rows
- [ ] update [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) row statuses and classes
- [ ] ensure wave completion checkboxes match ledger truth
- [ ] ensure no stale green claims remain after contradictory evidence

## Wave Exit

- [ ] all Tier-A/B/C docs above were read directly
- [ ] implementation tasks are complete or explicitly deferred with rationale
- [ ] verification tasks are complete with deterministic evidence
- [ ] ledger sync tasks are complete in the same logical closure change
- [ ] continue: [/docs/todo/waves/stage-04-window-explorer-terminal/wave-034.md](/docs/todo/waves/stage-04-window-explorer-terminal/wave-034.md)
