# Export and Backup Contract

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Markdown Export

- Export job MUST create markdown files representing current projections.
- File paths SHOULD be deterministic by note title + ID.
- Job status MUST be queryable via `GET /admin/export/{job_id}`.

## SQL Backup

- Backup endpoint MUST trigger a PostgreSQL-consistent dump artifact.
- Backup operations MUST be audited and access-controlled.

## Job Semantics

- Export and backup endpoints MAY return async job IDs.
- Jobs MUST expose states: `queued`, `running`, `succeeded`, `failed`.

## Related

- API contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
