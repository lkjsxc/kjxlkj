# Wave 020: Notes CRUD and Projection Baseline

Back: [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] implement workspace-scoped note create/list/get/update/title/delete
- [x] implement note history and rollback APIs
- [x] enforce optimistic versioning semantics

## Verification Tasks

- [x] run `API-NOTE-01..04`
- [x] run version conflict boundary tests

## Evidence Placeholder

- [x] `Check: note lifecycle + history/rollback + version conflict integration coverage`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-02-wave-020-notes-core.md](/docs/log/audits/2026-02-13-stage-02-wave-020-notes-core.md)`
