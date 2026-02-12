# Migration Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Tooling

- Database schema migrations MUST use SQLx migration files.
- Migrations MUST be ordered and deterministic.

## Schema Domains

Migrations MUST cover:

- users and sessions
- note streams and note events
- note projections and snapshots
- tags and backlinks projections
- attachments and chunks

## Safety Rules

- forward migrations MUST be idempotent in deployment scripts
- rollback scripts SHOULD exist when feasible
- migration failures MUST fail startup readiness

## Related

- Runtime startup: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Deployment: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
