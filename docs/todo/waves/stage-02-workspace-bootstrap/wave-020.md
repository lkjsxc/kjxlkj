# Wave 020: Notes Lifecycle, Versions, and History

Back: [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

## Relevant Documents

- [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [/docs/spec/domain/note-types.md](/docs/spec/domain/note-types.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Restructure Steps

- [x] restructure-step S02-W020-01: implement note create/list/get/update/title/delete paths from [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [x] restructure-step S02-W020-02: enforce note-kind and access-scope invariants from [/docs/spec/domain/note-types.md](/docs/spec/domain/note-types.md)
- [x] restructure-step S02-W020-03: implement history and rollback semantics from [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [x] restructure-step S02-W020-04: enforce optimistic version conflicts and payload types from [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [x] restructure-step S02-W020-05: enforce deterministic error responses from [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Verification Hooks

- [x] restructure-step S02-W020-V01: run `API-NOTE-*` checks defined in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S02-W020-V02: sync notes-core status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
