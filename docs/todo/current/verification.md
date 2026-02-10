# TODO Verification Gates

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate -1: Standby Baseline (Completed)

- [x] docs-only baseline is active
- [x] source/workspace artifacts are absent by design
- [x] conformance and limitations are synchronized to standby state
- [x] TODO remains in unchecked implementation mode

## Gate 0: Requirement Inventory

- [ ] requirement matrix includes every normative spec area
- [ ] each requirement has stable ID and spec link
- [ ] each requirement has explicit status

## Gate 1: Reachability

- [ ] each completed feature is reachable from real key or command path
- [ ] each completed feature produces observable behavior change
- [ ] no completion is based on type-only or dead code path

## Gate 2: Deterministic Verification

- [ ] targeted tests pass for touched requirements
- [ ] profile-appropriate full gate from [/docs/reference/CI.md](/docs/reference/CI.md) passes
- [ ] verification evidence is recorded in canonical ledgers

## Gate 3: Reference Synchronization

- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) updated with evidence-backed claims
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) updated for remaining gaps
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) updated for closed/deferred mismatches

## Gate 4: TODO Integrity

- [ ] every checked TODO item has linked evidence
- [ ] no checked TODO item conflicts with open limitations
- [ ] doc coverage includes direct links to every documentation file
