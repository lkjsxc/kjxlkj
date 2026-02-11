# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate 0: Inventory and Drift

- [x] requirement matrix links canonical high-risk specs
- [x] mismatch matrix classifies drift using `M1`..`M5`
- [x] limitations align with strongest runtime evidence

## Gate 1: Reachability

- [x] each blocker fix is reachable from real key or command paths
- [x] each claimed fix changes visible runtime behavior
- [x] no closure is based on dead or type-only paths

## Gate 2: Deterministic Verification

- [x] targeted regression tests for touched blockers pass
- [x] matching live PTY E2E tests (`*R`) pass
- [x] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes

## Gate 3: Reference Synchronization

- [x] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with strongest evidence
- [x] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated with open/closed rows
- [x] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated with row status

## Gate 4: TODO Integrity

- [x] every checked TODO item has direct evidence links
- [x] no checked TODO item conflicts with open limitations
- [x] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) links all docs directly
