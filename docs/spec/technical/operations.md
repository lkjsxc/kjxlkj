# Operations Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Health and Readiness

- `/api/healthz` checks process liveness.
- `/api/readyz` checks DB connectivity and migration compatibility.

## Backup and Recovery

| Operation | Requirement |
|---|---|
| SQL backup | callable via admin API and auditable |
| restore drill | must prove functional parity after restore |
| embedding reindex | resumable and idempotent |

## Observability

- structured logs MUST include `request_id`, `user_id`, `workspace_id`.
- error logs MUST include stable error code.
- agent runs MUST log prompt hash, parser version, and operation counts.
- full conversation transcript storage MUST stay disabled by default.

## Related

- Agent contract: [librarian-agent.md](librarian-agent.md)
- Security: [/docs/spec/security/README.md](/docs/spec/security/README.md)
