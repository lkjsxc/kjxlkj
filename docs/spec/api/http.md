# HTTP Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Base path: `/api`

## Auth and Session

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/setup/register` | first-run owner bootstrap |
| `POST` | `/auth/login` | create session |
| `POST` | `/auth/logout` | revoke current session |
| `GET` | `/auth/session` | current session identity |

## Core Workspace and Note APIs

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/workspaces` | list workspaces |
| `POST` | `/workspaces` | create workspace |
| `GET` | `/notes` | list notes |
| `POST` | `/notes` | create note |
| `GET` | `/notes/{id}` | read note projection |
| `PATCH` | `/notes/{id}` | patch note with version check |
| `PATCH` | `/notes/{id}/title` | update title with version check |
| `DELETE` | `/notes/{id}` | soft-delete note |
| `GET` | `/notes/{id}/history` | event history |

## Search and Link APIs

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/search` | hybrid lexical+semantic note search |
| `GET` | `/notes/{id}/backlinks` | backlink projections |

`GET /search` MUST support:

- `q` (required)
- `mode` (`hybrid`, `lexical`, `semantic`)
- `workspace_id` (required)
- `project_id` (optional)
- `limit` (optional)

## Automation and Agent APIs

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/automation/rules` | list rules |
| `POST` | `/automation/rules` | create rule |
| `PATCH` | `/automation/rules/{id}` | update rule |
| `POST` | `/automation/rules/{id}/launch` | launch run |
| `GET` | `/automation/runs` | list runs |
| `GET` | `/automation/runs/{id}` | run status and audit |
| `POST` | `/automation/runs/{id}/review` | apply/reject operation decisions |

Agent rule payloads MUST support `action_json.kind = "kjxlkj_agent"`.

## Contract Rules

- `POST /notes` without title MUST assign datetime title.
- `note_id` MUST remain immutable and separate from `title`.
- Version conflicts MUST return `409` with current version details.
- Unknown search `mode` values MUST return deterministic `422`.
- Agent YOLO mode MUST still honor permission and workspace boundaries.

## Related

- Types: [types.md](types.md)
- Errors: [errors.md](errors.md)
- Domain notes: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Domain search: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
