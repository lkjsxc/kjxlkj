# Export and Backup Contract

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Markdown Export

- Export launch endpoint (`POST /admin/export/markdown`) MUST create an admin job
  record and return `202`.
- Job payload MUST include deterministic `artifact_path`.
- Job status MUST be queryable via `GET /admin/export/{job_id}`.

## SQL Backup

- Backup launch endpoint (`POST /admin/backup/sql`) MUST create an admin job
  record and return `202`.
- Backup operations MUST be access-controlled.

## Job Semantics

- Export and backup endpoints MUST return admin job IDs.
- Current baseline job status is deterministic `completed`.

## Related

- API contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
