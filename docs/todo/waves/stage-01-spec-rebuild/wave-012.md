# Wave 012: RBAC, Workspace Membership, and Project Access

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- [/docs/spec/domain/projects.md](/docs/spec/domain/projects.md)
- [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)

## Restructure Steps

- [x] restructure-step S01-W012-01: implement workspace membership semantics from [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md) [doc-link](/docs/spec/domain/workspaces.md)
- [x] restructure-step S01-W012-02: implement project access boundaries from [/docs/spec/domain/projects.md](/docs/spec/domain/projects.md) [doc-link](/docs/spec/domain/projects.md)
- [x] restructure-step S01-W012-03: enforce role matrix from [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) [doc-link](/docs/spec/domain/permissions.md)
- [x] restructure-step S01-W012-04: enforce deterministic forbidden/error payloads from [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/errors.md)
- [x] restructure-step S01-W012-05: emit role/membership mutation events matching [/docs/spec/domain/events.md](/docs/spec/domain/events.md) [doc-link](/docs/spec/domain/events.md)

## Verification Hooks

- [x] restructure-step S01-W012-V01: run permission matrix and forbidden-path checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S01-W012-V02: update mismatch status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
