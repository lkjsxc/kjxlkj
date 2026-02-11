# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate 0: Inventory and Drift

- [ ] requirement matrix links canonical high-risk specs
- [ ] mismatch matrix classifies drift using `M1`..`M5`
- [ ] limitations align with strongest runtime evidence

## Gate 1: Reachability

- [ ] each blocker fix is reachable from real key or command paths
- [ ] each claimed fix changes visible runtime behavior
- [ ] no closure is based on dead or type-only paths

## Gate 2: Deterministic Verification

- [ ] targeted regression tests for touched blockers pass
- [ ] matching live PTY E2E tests (`*R`) pass
- [ ] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes

## Gate 3: Reference Synchronization

- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with strongest evidence
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated with open/closed rows
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated with row status

## Gate 4: TODO Integrity

- [ ] every checked TODO item has direct evidence links
- [ ] no checked TODO item conflicts with open limitations
- [ ] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) links all docs directly
