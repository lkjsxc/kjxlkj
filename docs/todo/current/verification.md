# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate 0: Inventory and Drift

- [x] requirement matrix links canonical spec for high-risk domains
- [x] mismatch matrix classifies drift using `M1`..`M5`
- [x] limitations are synchronized with user-reported blockers

## Gate 1: Reachability

- [ ] each blocker fix is reachable from real key or command path
- [ ] each claimed fix changes observable runtime behavior
- [ ] no completion is based on type-only or dead code paths

## Gate 2: Deterministic Verification

- [ ] targeted regression tests for touched blockers pass
- [ ] mandatory live PTY E2E tests (`*R`) pass
- [ ] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes

## Gate 3: Reference Synchronization

- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with strongest evidence
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated for remaining gaps
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated for closed/deferred rows

## Gate 4: TODO Integrity

- [ ] every checked TODO item has linked evidence
- [ ] no checked TODO item conflicts with open limitations
- [ ] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) links all documentation files directly
