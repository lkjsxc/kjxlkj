# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the executable reconstruction checklist for AI agents.

## Recursive Execution Order

- [ ] Step 1: [checklists/README.md](checklists/README.md)
- [ ] Step 2: [checklists/00-entry-gate.md](checklists/00-entry-gate.md)
- [ ] Step 3: [checklists/01-critical-blockers.md](checklists/01-critical-blockers.md)
- [ ] Step 4: [checklists/02-implementation-architecture.md](checklists/02-implementation-architecture.md)
- [ ] Step 5: [checklists/03-test-implementation.md](checklists/03-test-implementation.md)
- [ ] Step 6: [checklists/04-verification-and-ledgers.md](checklists/04-verification-and-ledgers.md)
- [ ] Step 7: [checklists/05-release-readiness.md](checklists/05-release-readiness.md)
- [ ] Step 8: [inventory/README.md](inventory/README.md)

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
- [ ] TODO inventory links every documentation file directly
- [ ] no checked item conflicts with canonical policy/spec/reference docs
