# HTTP Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Base path: `/api`

## Setup and Auth

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/setup/status` | discover whether first-run owner setup is still available |
| `POST` | `/setup/register` | first-run owner account bootstrap |
| `POST` | `/auth/login` | create authenticated session |
| `POST` | `/auth/logout` | revoke active session |
| `GET` | `/auth/session` | return current session identity |

## Users, Roles, Workspaces

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/users` | list users in tenant |
| `POST` | `/users` | create user account |
| `PATCH` | `/users/{id}/role` | change global role |
| `DELETE` | `/users/{id}` | disable or remove user |
| `GET` | `/workspaces` | list workspaces |
| `POST` | `/workspaces` | create workspace |
| `PATCH` | `/workspaces/{id}` | update workspace |
| `DELETE` | `/workspaces/{id}` | delete workspace |
| `GET` | `/workspaces/{id}/members` | list workspace members |
| `PUT` | `/workspaces/{id}/members/{user_id}` | upsert member role |

## Projects and Notes

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/projects` | list projects |
| `POST` | `/projects` | create project |
| `PATCH` | `/projects/{id}` | update project |
| `DELETE` | `/projects/{id}` | delete project |
| `POST` | `/notes` | create note stream |
| `POST` | `/notes/media` | create standalone media note from upload |
| `GET` | `/notes` | list notes |
| `GET` | `/notes/{id}` | fetch note projection |
| `PATCH` | `/notes/{id}` | apply note mutation with version check |
| `PATCH` | `/notes/{id}/title` | update note title with version check |
| `DELETE` | `/notes/{id}` | soft-delete note stream |
| `GET` | `/notes/{id}/history` | list event history |
| `POST` | `/notes/{id}/rollback` | rollback to selected version |

## Metadata, Links, Search

| Method | Path | Purpose |
|---|---|---|
| `PUT` | `/notes/{id}/metadata/{key}` | upsert typed metadata |
| `DELETE` | `/notes/{id}/metadata/{key}` | delete typed metadata key |
| `GET` | `/tags` | list tags |
| `PUT` | `/notes/{id}/tags` | replace tags for note |
| `GET` | `/notes/{id}/backlinks` | backlinks for note |
| `GET` | `/search` | full-text and filter search |

## Views, Optional Dashboards, Automation

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/views` | list saved views |
| `POST` | `/views` | create saved view |
| `PATCH` | `/views/{id}` | update saved view |
| `DELETE` | `/views/{id}` | delete saved view |
| `GET` | `/dashboards` | list workspace dashboard widgets |
| `POST` | `/dashboards/widgets` | create or update dashboard widget |
| `GET` | `/automation/rules` | list automation rules |
| `POST` | `/automation/rules` | create automation rule (includes librarian structuring rules) |
| `PATCH` | `/automation/rules/{id}` | update automation rule (includes provider/prompt contract) |
| `DELETE` | `/automation/rules/{id}` | delete automation rule |
| `POST` | `/automation/rules/{id}/launch` | manually launch an automation run for selected rule |
| `GET` | `/automation/runs` | list automation runs for workspace |
| `GET` | `/automation/runs/{id}` | automation run status/details (includes librarian operations) |
| `POST` | `/automation/runs/{id}/review` | persist apply/reject decisions and optional apply execution summary |

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

## Contract Rules

- Global roles and workspace roles MUST be evaluated on every user-scoped route.
- `GET /setup/status` MUST return deterministic `setup_available` boolean.
- Setup MUST lock after first owner account registration.
- `DELETE /notes/{id}` MUST return `204` on successful soft-delete.
- `DELETE /notes/{id}/metadata/{key}` MUST return `204`.
- Version conflicts MUST return `409` with current server version context.
- `POST /notes/{id}/attachments` MUST reject payloads greater than `500MB` with `413`.
- `GET /attachments/{id}` MUST return deterministic attachment payload metadata and content fields.
- export/backup launch endpoints MUST return `202` with created admin job payload.
- Librarian rules MUST validate provider mode (`openrouter` or `lmstudio`) and
  reject unknown providers with deterministic `422`.
- Librarian rules MUST validate `prompt_pack.manifest_path` and require all
  referenced stage JSON files to exist and be parseable.
- Librarian actions MUST use the attribute-less XML-like protocol from
  [librarian-xml.md](librarian-xml.md).

## Related

- Error model: [errors.md](errors.md)
- Payload types: [types.md](types.md)
