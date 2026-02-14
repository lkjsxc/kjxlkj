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

- [ ] implement workspace-scoped note create/list/get/update/title/delete -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [ ] implement note history and rollback APIs -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [ ] enforce optimistic versioning semantics -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)

## Verification Tasks

- [ ] run `API-NOTE-01..04` -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [ ] run version conflict boundary tests -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)

## Evidence Placeholder

- [ ] `Check: note lifecycle + history/rollback + version conflict integration coverage` -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [ ] `Result: pass` -> [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-02-wave-020-notes-core.md](/docs/log/audits/2026-02-13-stage-02-wave-020-notes-core.md)`
