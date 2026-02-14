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

- [x] restructure-step S04-W041-01: implement automation rule CRUD semantics from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-02: implement run state machine transitions from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-03: enforce per-trigger idempotency and replay-safe run creation from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [x] restructure-step S04-W041-04: align HTTP payloads and errors with [/docs/spec/api/types.md](/docs/spec/api/types.md) and [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [x] restructure-step S04-W041-05: emit deterministic automation events per [/docs/spec/domain/events.md](/docs/spec/domain/events.md)

## Verification Hooks

- [x] restructure-step S04-W041-V01: run `API-AUTO-01` and `API-AUTO-02` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S04-W041-V02: sync automation status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
