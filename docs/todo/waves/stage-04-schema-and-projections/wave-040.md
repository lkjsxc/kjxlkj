# Wave 040: Migration and Projection Integrity

Back: [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

## Relevant Documents

- [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md)
- [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S04-W040-01: implement migration ordering and compatibility policy from [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md)
- [x] restructure-step S04-W040-02: enforce event append and projection update integrity from [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [x] restructure-step S04-W040-03: enforce workspace and note projection boundaries from [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md) and [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- [x] restructure-step S04-W040-04: align projection payload contracts with [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [x] restructure-step S04-W040-05: lock migration rollback expectations from [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md)

## Verification Hooks

- [x] restructure-step S04-W040-V01: run migration/projection checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S04-W040-V02: sync migration status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
