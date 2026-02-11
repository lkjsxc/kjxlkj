# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate -1: Standby Baseline

- [x] docs-only baseline is active
- [x] source/workspace artifacts are absent by design
- [x] reconstructed-profile CI workflow is absent by design
- [x] conformance/limitations/drift are synchronized to standby state

## Gate 0: Inventory and Drift

- [x] requirement matrix links canonical spec for high-risk domains
- [x] mismatch matrix classifies drift using `M1`..`M5`
- [x] limitations are synchronized with observed runtime evidence

## Gate 1: Reachability

- [x] each blocker fix is reachable from real key or command path
- [x] each claimed fix changes observable runtime behavior
- [x] no completion is based on type-only or dead code paths

## Gate 2: Deterministic Verification

- [x] targeted regression tests for touched blockers pass
- [x] mandatory live PTY E2E tests (`*R`) pass
- [x] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes

## Gate 3: Reference Synchronization

- [x] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with strongest evidence
- [x] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated for remaining gaps
- [x] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated for closed/deferred rows

## Gate 4: TODO Integrity

- [x] every checked TODO item has linked evidence
- [x] no checked TODO item conflicts with open limitations
- [x] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) links all documentation files directly
