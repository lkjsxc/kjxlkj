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

- [x] read [/README.md](/README.md)
- [x] read [/docs/README.md](/docs/README.md)
- [x] read [/docs/policy/README.md](/docs/policy/README.md)
- [x] read [/docs/spec/README.md](/docs/spec/README.md)
- [x] read [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [x] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [x] read [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Immediate Blocker Pack

- [x] `LIM-RUNTIME-03` runtime bootstrap and supervision reconstruction
- [x] `LIM-API-03` API implementation reconstruction
- [x] `LIM-WS-03` websocket realtime reconstruction
- [x] `LIM-UI-03` workspace suite frontend reconstruction
- [x] `LIM-RBAC-02` role-based authorization reconstruction
- [x] `LIM-AUTO-02` automation rule and run engine reconstruction
- [x] `LIM-LIB-02` librarian AI structuring engine reconstruction
- [x] `LIM-ISSUE-GUARD-03` regression guards for `IMP-*` and `USR-*` findings
- [x] `LIM-UX-03` UX requirement execution for `UX-*` matrix and `REG-UX-*` pack
- [x] `LIM-LIB-GUARD-02` parser/provider regression guards

## Findings and UX Closure Pack

- [x] all `IMP-*` findings mapped in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) have passing `REG-IMP-*` evidence
- [x] all `USR-*` findings mapped in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) have passing `REG-USR-*` evidence
- [x] all `UX-*` requirements in [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) have deterministic acceptance proof
- [x] compact layout and accessibility regressions (`E2E-19..22`) are passing

## Recursive Wave Program

- [x] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [x] execute stages and waves in order (Stage 00 through Stage 09)
- [x] do not mark work complete without deterministic evidence
- [x] synchronize reference ledgers with status changes

## Completion Gate

- [x] all stage and wave checklists are complete in order
- [x] all high-severity limitation rows are closed
- [x] acceptance tests in technical spec pass
- [x] release gate in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) is satisfied
