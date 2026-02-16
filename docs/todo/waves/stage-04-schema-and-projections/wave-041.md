# Wave 041: Automation Rule and Run State Machine

Back: [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

## Relevant Documents

- [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)

## Restructure Steps

- [x] restructure-step S04-W041-01: implement automation rule CRUD semantics from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) [doc-link](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-02: implement run state machine transitions from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) [doc-link](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-03: enforce per-trigger idempotency and replay-safe run creation from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) [doc-link](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-04: align HTTP payloads and errors with [/docs/spec/api/types.md](/docs/spec/api/types.md) and [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/types.md)
- [x] restructure-step S04-W041-05: emit deterministic automation events per [/docs/spec/domain/events.md](/docs/spec/domain/events.md) [doc-link](/docs/spec/domain/events.md)

## Verification Hooks

- [x] restructure-step S04-W041-V01: run `API-AUTO-01` and `API-AUTO-02` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S04-W041-V02: sync automation status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
