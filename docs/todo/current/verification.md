# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate -1: Standby Baseline (Completed)

- [x] docs-only baseline is active
- [x] source/workspace artifacts are absent by design
- [x] conformance and limitations are synchronized to standby state
- [x] TODO remains in unchecked implementation mode

## Gate 0: Requirement Inventory

- [x] requirement matrix includes every normative spec area
- [x] each requirement has stable ID and spec link
- [x] each requirement has explicit status

## Gate 1: Reachability

- [x] each completed feature is reachable from real key or command path
- [x] each completed feature produces observable behavior change
- [x] no completion is based on type-only or dead code path

## Gate 2: Deterministic Verification

- [x] targeted tests pass for touched requirements
- [x] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes
- [x] verification evidence is recorded in canonical ledgers

## Gate 3: Reference Synchronization

- [x] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with evidence-backed claims
- [x] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated for remaining gaps
- [x] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated for closed/deferred mismatches

## Gate 4: TODO Integrity

- [x] every checked TODO item has linked evidence
- [x] no checked TODO item conflicts with open limitations
- [x] doc coverage includes direct links to every documentation file
