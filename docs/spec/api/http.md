# HTTP Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Base path: `/api/v1`

## Setup and Auth

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/setup/register` | first-run account bootstrap |
| `POST` | `/auth/login` | create authenticated session |
| `POST` | `/auth/logout` | revoke active session |
| `GET` | `/auth/session` | return current session identity |

## Notes and History

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/notes` | create note stream |
| `GET` | `/notes` | list notes |
| `GET` | `/notes/{id}` | fetch note projection |
| `PATCH` | `/notes/{id}` | apply note mutation with version check |
| `DELETE` | `/notes/{id}` | soft-delete note stream |
| `GET` | `/notes/{id}/history` | list event history |
| `POST` | `/notes/{id}/rollback` | rollback to selected version |

## Records, Tags, Links, Search

| Method | Path | Purpose |
|---|---|---|
| `PUT` | `/notes/{id}/metadata/{key}` | upsert typed metadata |
| `DELETE` | `/notes/{id}/metadata/{key}` | delete typed metadata key |
| `GET` | `/tags` | list tags |
| `PUT` | `/notes/{id}/tags` | replace tags for note |
| `GET` | `/notes/{id}/backlinks` | backlinks for note |
| `GET` | `/search` | full-text and filter search |

## Attachments and Admin Operations

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/notes/{id}/attachments` | upload chunked attachment |
| `GET` | `/attachments/{id}` | download attachment |
| `DELETE` | `/attachments/{id}` | delete attachment |
| `POST` | `/admin/export/markdown` | launch markdown export job |
| `GET` | `/admin/export/{job_id}` | export job status/artifact |
| `POST` | `/admin/backup/sql` | launch SQL backup job |

## Ops

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/healthz` | liveness check |
| `GET` | `/readyz` | readiness check (DB + migrations) |

## Related

- Error model: [errors.md](errors.md)
- Payload types: [types.md](types.md)
