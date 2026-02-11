# Wave 066: State Model and Data Flow Design

Back: [/docs/todo/waves/stage-08-release-and-ops/README.md](/docs/todo/waves/stage-08-release-and-ops/README.md)

## Wave Identity

- [ ] Wave ID: `W066`
- [ ] Stage: `Stage 08: Release and Ops`
- [ ] Focus: State Model and Data Flow Design
- [ ] Scope statement is understood: End-to-end release evidence, conformance closure, and operational readiness.

## Mandatory Document Reads (Direct Links)

- [ ] [/docs/spec/ux/keybindings/windows-tabs.md](/docs/spec/ux/keybindings/windows-tabs.md)
- [ ] [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
- [ ] [/docs/spec/ux/keyboard-layouts.md](/docs/spec/ux/keyboard-layouts.md)
- [ ] [/docs/spec/ux/layout.md](/docs/spec/ux/layout.md)
- [ ] [/docs/spec/ux/theming.md](/docs/spec/ux/theming.md)
- [ ] [/docs/technical/README.md](/docs/technical/README.md)

## Requirement Normalization

- [ ] extract normative `MUST` and `MUST NOT` statements from all wave docs
- [ ] record ambiguous terms and normalize into deterministic language
- [ ] map each requirement to expected user-visible behavior
- [ ] map each requirement to expected state transition
- [ ] classify requirement risk: correctness, reachability, or verification
- [ ] identify required command/key entry paths
- [ ] identify required data structures and invariants
- [ ] identify failure modes and explicit negative behavior
- [ ] identify persistence/session expectations where relevant
- [ ] identify cross-domain dependencies and required order
- [ ] identify required diagnostics and failure artifacts
- [ ] identify mandatory test IDs from technical specs
- [ ] identify any policy constraints impacting implementation shape
- [ ] identify required ledger updates for closure evidence
- [ ] record unresolved contradictions for later drift entries
- [ ] confirm wave scope is minimal but complete for assigned docs

## Implementation Decomposition

- [ ] define concrete module boundaries before coding
- [ ] keep each touched source directory near 12 direct children
- [ ] split files before they exceed 200 lines
- [ ] separate state mutation from IO side effects
- [ ] separate parsing from dispatch and rendering paths
- [ ] define deterministic action routing for each entry path
- [ ] wire real user-reachable paths before auxiliary helpers
- [ ] implement explicit error paths for invalid or partial states
- [ ] preserve backward-compatible behavior where required by spec
- [ ] make behavior reproducible across repeated runs
- [ ] add or update structured diagnostics for failure triage
- [ ] avoid hidden global state and implicit coupling
- [ ] document any intentional defer decisions in ledgers
- [ ] confirm each requirement has an implementation hook or explicit TODO
- [ ] verify no placeholder/stub-only paths are treated as complete
- [ ] prepare small cohesive commits scoped to one logical closure unit

## Unit and Integration Verification

- [ ] design failing regression tests before or with the fix
- [ ] cover boundary conditions and state transitions
- [ ] cover invalid inputs and expected error behavior
- [ ] cover deterministic replay and idempotent operations
- [ ] cover persistence/restore where applicable
- [ ] cover mixed-feature interaction where applicable
- [ ] cover Unicode or width-sensitive behavior when relevant
- [ ] cover race-prone interactions with deterministic deadlines
- [ ] attach each test to requirement IDs and wave ID
- [ ] verify test names and IDs remain stable and searchable
- [ ] remove redundant low-signal tests if superseded by stronger coverage
- [ ] document residual gaps explicitly if any test cannot be added now
- [ ] confirm no checkbox is checked without direct test evidence
- [ ] ensure test output artifacts are sufficient for debugging

## Live E2E and Boundary Validation

- [ ] run user-like PTY flows for all touched blocker behaviors
- [ ] dump state after each key input for touched E2E cases
- [ ] assert screen output rows against explicit expectations
- [ ] assert cursor/caret coordinates and focused pane type
- [ ] assert pane geometry/layout summary consistency
- [ ] assert raw input to normalized action mapping correctness
- [ ] assert no hidden regressions in mixed-feature scripts
- [ ] run at least one stress or race scenario for touched domains
- [ ] capture bounded failure artifacts for every failing run
- [ ] confirm blocker closure is not based on trace-only evidence
- [ ] rerun same script twice and verify deterministic outcome
- [ ] record exact commands and pass/fail signals in evidence notes
- [ ] update E2E matrix references if IDs or scope changed
- [ ] leave unchecked if any required E2E scenario remains unproven

## Drift, Ledger, and Traceability Sync

- [ ] update limitation rows for newly discovered gaps
- [ ] update drift rows with status and mismatch class
- [ ] update conformance claims to strongest current evidence only
- [ ] ensure requirement IDs map to code paths and test paths
- [ ] ensure evidence links are deterministic and reproducible
- [ ] ensure TODO checkboxes align with ledger status
- [ ] ensure no stale green claims remain after contradictory evidence
- [ ] record deferred items with rationale and concrete next action
- [ ] cross-check with policy/spec precedence before closure
- [ ] confirm this wave produced no orphaned documentation links

## Wave Exit Gate

- [ ] all mandatory document reads are completed
- [ ] all in-wave requirements are implemented or explicitly deferred
- [ ] all mandatory tests for in-wave requirements pass
- [ ] all mandatory E2E for in-wave blockers pass with screen assertions
- [ ] reference ledgers are synchronized in same logical change
- [ ] inventory links remain complete and valid
- [ ] no file-size or fan-out policy violations were introduced
- [ ] remaining risks are documented explicitly with follow-up IDs
- [ ] all evidence commands and proof snippets are recorded
- [ ] next wave handoff is concrete and deterministic

## Next Wave

- [ ] continue: [/docs/todo/waves/stage-08-release-and-ops/wave-067.md](/docs/todo/waves/stage-08-release-and-ops/wave-067.md)
