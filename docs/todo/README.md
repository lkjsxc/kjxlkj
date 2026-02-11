# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the executable reconstruction control plane for AI agents.

## Recursive Execution Order

- [ ] Step 1: [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)
- [ ] Step 2: [/docs/todo/checklists/00-entry-gate.md](/docs/todo/checklists/00-entry-gate.md)
- [ ] Step 3: [/docs/todo/checklists/01-critical-blockers.md](/docs/todo/checklists/01-critical-blockers.md)
- [ ] Step 4: [/docs/todo/checklists/02-implementation-architecture.md](/docs/todo/checklists/02-implementation-architecture.md)
- [ ] Step 5: [/docs/todo/checklists/03-test-implementation.md](/docs/todo/checklists/03-test-implementation.md)
- [ ] Step 6: [/docs/todo/checklists/04-verification-and-ledgers.md](/docs/todo/checklists/04-verification-and-ledgers.md)
- [ ] Step 7: [/docs/todo/checklists/05-release-readiness.md](/docs/todo/checklists/05-release-readiness.md)
- [ ] Step 8: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] Step 9: [/docs/todo/waves/stage-00-foundation-ingestion/README.md](/docs/todo/waves/stage-00-foundation-ingestion/README.md)
- [ ] Step 10: [/docs/todo/waves/stage-01-architecture-core/README.md](/docs/todo/waves/stage-01-architecture-core/README.md)
- [ ] Step 11: [/docs/todo/waves/stage-02-editing-and-modes/README.md](/docs/todo/waves/stage-02-editing-and-modes/README.md)
- [ ] Step 12: [/docs/todo/waves/stage-03-commands-and-ranges/README.md](/docs/todo/waves/stage-03-commands-and-ranges/README.md)
- [ ] Step 13: [/docs/todo/waves/stage-04-window-explorer-terminal/README.md](/docs/todo/waves/stage-04-window-explorer-terminal/README.md)
- [ ] Step 14: [/docs/todo/waves/stage-05-services-and-features/README.md](/docs/todo/waves/stage-05-services-and-features/README.md)
- [ ] Step 15: [/docs/todo/waves/stage-06-ui-ux-and-scripting/README.md](/docs/todo/waves/stage-06-ui-ux-and-scripting/README.md)
- [ ] Step 16: [/docs/todo/waves/stage-07-technical-hardening/README.md](/docs/todo/waves/stage-07-technical-hardening/README.md)
- [ ] Step 17: [/docs/todo/waves/stage-08-release-and-ops/README.md](/docs/todo/waves/stage-08-release-and-ops/README.md)
- [ ] Step 18: [/docs/todo/inventory/README.md](/docs/todo/inventory/README.md)

## Multi-Wave Program Contract

- [ ] the wave program contains dozens of waves and is mandatory, not optional
- [ ] every wave has direct document-read links that must be checked before closure
- [ ] reconstruction is invalid if any wave is skipped or reordered
- [ ] ledger updates must stay synchronized with wave completion state

## Non-Negotiable Rules

- [ ] never mark a checkbox complete without deterministic evidence
- [ ] treat user-reported runtime failures as higher priority than stale green claims
- [ ] close blockers in this order: correctness, reachability, verification quality
- [ ] update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) in the same change
- [ ] keep source topology near 12 direct children per directory and split files before 200 lines
- [ ] keep all internal links free of `../`

## Completion Gate

- [ ] all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed
- [ ] all blocker `*R` cases in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) pass with screen-state assertions
- [ ] all wave files in [/docs/todo/waves/](/docs/todo/waves/README.md) are completed in order
- [ ] TODO inventory links every documentation file directly
- [ ] no checked item conflicts with canonical policy/spec/reference docs
