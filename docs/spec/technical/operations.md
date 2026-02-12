# Operations Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Health and Readiness

- `/healthz` checks process liveness.
- `/readyz` checks DB connectivity and migration compatibility.

## Backup and Recovery

| Operation | Requirement |
|---|---|
| SQL backup | callable via admin API and auditable |
| restore drill | MUST prove functional parity after restore |
| markdown export | MUST emit deterministic note artifacts |

## Observability

- structured logs MUST include request ID and principal/session context
- error logs MUST include stable error code
- critical jobs MUST emit start/finish/failure events

## Related

- Export model: [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- CI/Release: [/docs/reference/CI.md](/docs/reference/CI.md)
