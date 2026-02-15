# Migration Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Tooling

- Migrations MUST be deterministic and ordered.
- Migrations MUST include forward-apply scripts.

## Required Schema Domains

- notes, events, projections
- search lexical indexes and vector embedding storage
- automation rules and runs
- agent KV memory store and state tables

## Safety Rules

- migration failures MUST fail startup readiness
- forward migrations MUST be idempotent in deployment scripts
- rollback scripts SHOULD exist for high-risk changes

## Related

- Search: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
- Agent: [librarian-agent.md](librarian-agent.md)
