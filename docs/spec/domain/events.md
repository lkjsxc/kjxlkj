# Event Sourcing Contract

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Event Store Rules

- `NoteEvent` rows are append-only.
- `WorkspaceEvent` rows are append-only.
- Event sequence (`seq`) is strictly increasing per stream identity.
- Events MUST record actor, timestamp, and payload.

## Snapshot Policy

- Snapshot MUST be recorded every 100 events per note stream.
- Snapshot MUST include full markdown and current metadata projection.
- Rebuild logic MUST replay events after last snapshot to current version.

## Transaction Rule

For accepted mutation:

1. validate version and authorization
2. append event
3. update affected projection(s)
4. commit transaction
5. publish event to WS subscribers

Steps 1-4 MUST be atomic.

## Related

- Notes contract: [notes.md](notes.md)
- Workspaces contract: [workspaces.md](workspaces.md)
- Automation contract: [automation.md](automation.md)
- Migrations: [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md)
