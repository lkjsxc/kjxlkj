# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the execution contract for rebuilding `kjxlkj` from docs.

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Start Gate

- [x] read [/README.md](/README.md)
- [x] read [/docs/README.md](/docs/README.md)
- [x] read [/docs/policy/README.md](/docs/policy/README.md)
- [x] read [/docs/spec/README.md](/docs/spec/README.md)
- [x] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [x] read [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Immediate Blocker Pack

- [ ] `LIM-BOOT-01` workspace/runtime reconstruction
- [ ] `LIM-DOCKER-01` single-container compose artifact reconstruction
- [ ] `LIM-API-01` API reconstruction
- [ ] `LIM-WS-01` websocket reconstruction
- [ ] `LIM-UI-01` UX shell/editor reconstruction
- [ ] `LIM-LIB-01` librarian runtime reconstruction
- [ ] `LIM-ISSUE-GUARD-01` `IMP-*`/`USR-*` regression guard pack

## Recursive Wave Program

- [x] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] execute stages and waves in order (Stage 00 through Stage 09)
- [ ] do not mark work complete without deterministic evidence
- [ ] synchronize reference ledgers with every status change

## Completion Gate

- [ ] all stage and wave checklists are complete in order
- [ ] no open high-severity limitation remains
- [ ] acceptance tests in technical spec pass
- [ ] single-container compose startup passes (`docker compose up --build`)
- [ ] release gate in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) is satisfied
