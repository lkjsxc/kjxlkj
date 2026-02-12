# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

`/docs/todo/` is the execution contract for rebuilding `kjxlkj` from docs.

## Start Gate

- [ ] read [/README.md](/README.md)
- [ ] read [/docs/README.md](/docs/README.md)
- [ ] read [/docs/policy/README.md](/docs/policy/README.md)
- [ ] read [/docs/spec/README.md](/docs/spec/README.md)
- [ ] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] read [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Immediate Blocker Pack

- [x] `LIM-RUNTIME-02` runtime bootstrap and crate topology reconstruction
- [x] `LIM-API-02` API implementation reconstruction
- [x] `LIM-WS-02` websocket realtime reconstruction
- [x] `LIM-UI-02` workspace suite frontend reconstruction
- [x] `LIM-RBAC-01` role-based authorization reconstruction
- [x] `LIM-AUTO-01` automation rule and run engine reconstruction
- [ ] `LIM-ISSUE-GUARD-02` regression guards for `IMP-*` and `USR-*` findings

## Recursive Wave Program

- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] execute stages and waves in order
- [ ] do not mark work complete without deterministic evidence
- [ ] synchronize reference ledgers with status changes

## Completion Gate

- [ ] all stage and wave checklists are complete in order
- [ ] all high-severity limitation rows are closed
- [ ] acceptance tests in technical spec pass
- [ ] release gate in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) is satisfied
