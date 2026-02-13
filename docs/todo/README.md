# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
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
- [ ] read [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] read [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Immediate Blocker Pack

- [ ] `LIM-RUNTIME-03` runtime bootstrap and supervision reconstruction
- [ ] `LIM-API-03` API implementation reconstruction
- [ ] `LIM-WS-03` websocket realtime reconstruction
- [ ] `LIM-UI-03` workspace suite frontend reconstruction
- [ ] `LIM-RBAC-02` role-based authorization reconstruction
- [ ] `LIM-AUTO-02` automation rule and run engine reconstruction
- [ ] `LIM-LIB-02` librarian AI structuring engine reconstruction
- [ ] `LIM-ISSUE-GUARD-03` regression guards for `IMP-*` and `USR-*` findings
- [ ] `LIM-UX-03` UX requirement execution for `UX-*` matrix and `REG-UX-*` pack
- [ ] `LIM-LIB-GUARD-02` parser/provider regression guards

## Findings and UX Closure Pack

- [ ] all `IMP-*` findings mapped in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) have passing `REG-IMP-*` evidence
- [ ] all `USR-*` findings mapped in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) have passing `REG-USR-*` evidence
- [ ] all `UX-*` requirements in [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) have deterministic acceptance proof
- [ ] compact layout and accessibility regressions (`E2E-19..22`) are passing

## Recursive Wave Program

- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] execute stages and waves in order (Stage 00 through Stage 09)
- [ ] do not mark work complete without deterministic evidence
- [ ] synchronize reference ledgers with status changes

## Completion Gate

- [ ] all stage and wave checklists are complete in order
- [ ] all high-severity limitation rows are closed
- [ ] acceptance tests in technical spec pass
- [ ] release gate in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) is satisfied
