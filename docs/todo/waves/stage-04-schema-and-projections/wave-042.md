# Wave 042: Export, Backup, and Job Observability

Back: [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

## Relevant Documents

- [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S04-W042-01: implement export and SQL backup launch contracts from [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- [x] restructure-step S04-W042-02: implement job status and artifact retrieval semantics from [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [x] restructure-step S04-W042-03: enforce job authorization and failure semantics from [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [x] restructure-step S04-W042-04: enforce ops observability and recovery hooks from [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
- [x] restructure-step S04-W042-05: keep deployment health endpoints aligned with [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)

## Verification Hooks

- [x] restructure-step S04-W042-V01: run `OPS-01` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S04-W042-V02: sync job/export status in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
