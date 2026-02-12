# Operations Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Health and Readiness

- `/api/healthz` checks process liveness.
- `/api/readyz` checks DB connectivity and migration compatibility.

## Backup and Recovery

| Operation | Requirement |
|---|---|
| SQL backup | callable via admin API and auditable |
| restore drill | MUST prove functional parity after restore |
| markdown export | MUST emit deterministic workspace-scoped artifacts |

## Observability

- structured logs MUST include `request_id`, `user_id`, `workspace_id`, and
 principal/session context
- error logs MUST include stable error code
- critical jobs (export/backup/automation) MUST emit start/finish/failure events

## Related

- Export model: [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- Automation model: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- CI/Release: [/docs/reference/CI.md](/docs/reference/CI.md)
