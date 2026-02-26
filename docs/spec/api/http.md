# HTTP Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Base path: `/api`

## Transport Invariants

- Every response MUST carry `x-request-id`.
- If client sends `x-request-id`, server MUST echo it; otherwise server MUST generate one.
- Every error response MUST follow [/docs/spec/api/errors.md](/docs/spec/api/errors.md).
- Mutating requests (`POST`, `PATCH`, `DELETE`) MUST enforce auth and CSRF for browser sessions.
- Mutating requests SHOULD accept `Idempotency-Key` header for retry safety.

## Auth and Session

| Method | Path | Auth | CSRF | Idempotency | Purpose |
|---|---|---|---|---|---|
| `POST` | `/setup/register` | no | yes | required | first-run owner bootstrap |
| `POST` | `/auth/login` | no | yes | optional | create session |
| `POST` | `/auth/logout` | yes | yes | optional | revoke current session |
| `GET` | `/auth/session` | optional | no | n/a | current session identity |

## Core Workspace and Note APIs

| Method | Path | Auth | CSRF | Idempotency | Purpose |
|---|---|---|---|---|---|
| `GET` | `/workspaces` | yes | no | n/a | list workspaces |
| `POST` | `/workspaces` | yes | yes | required | create workspace |
| `GET` | `/notes` | yes | no | n/a | list notes |
| `POST` | `/notes` | yes | yes | required | create note |
| `GET` | `/notes/{id}` | yes | no | n/a | read note projection |
| `PATCH` | `/notes/{id}` | yes | yes | required | patch note with version check |
| `PATCH` | `/notes/{id}/title` | yes | yes | required | update title with version check |
| `DELETE` | `/notes/{id}` | yes | yes | required | soft-delete note |
| `GET` | `/notes/{id}/history` | yes | no | n/a | event history |

## Search and Link APIs

| Method | Path | Auth | CSRF | Purpose |
|---|---|---|---|---|
| `GET` | `/search` | yes | no | hybrid lexical+semantic note search |
| `GET` | `/notes/{id}/backlinks` | yes | no | backlink projections |

`GET /search` MUST support:

- `q` (required)
- `mode` (`hybrid`, `lexical`, `semantic`)
- `workspace_id` (required)
- `project_id` (optional)
- `limit` (optional)

## Automation and Agent APIs

| Method | Path | Auth | CSRF | Idempotency | Purpose |
|---|---|---|---|---|---|
| `GET` | `/automation/rules` | yes | no | n/a | list rules |
| `POST` | `/automation/rules` | yes | yes | required | create rule |
| `PATCH` | `/automation/rules/{id}` | yes | yes | required | update rule |
| `POST` | `/automation/rules/{id}/launch` | yes | yes | required | launch run |
| `GET` | `/automation/runs` | yes | no | n/a | list runs |
| `GET` | `/automation/runs/{id}` | yes | no | n/a | run status and audit |
| `POST` | `/automation/runs/{id}/review` | yes | yes | required | apply/reject operation decisions |

Agent rule payloads MUST support `action_json.kind = "kjxlkj_agent"`.

## Contract Rules

- `POST /notes` without title MUST assign datetime title.
- `note_id` MUST remain immutable and separate from `title`.
- Version conflicts MUST return `409` with current version details.
- Unknown search `mode` values MUST return deterministic `422`.
- Agent YOLO mode MUST still honor permission and workspace boundaries.
- Search route MUST enforce per-user rate limit and return `429` + `Retry-After` on breach.
- Duplicate `Idempotency-Key` on mutating routes MUST return original success identity.
- Unauthorized and forbidden outcomes MUST be deterministic (`401` vs `403`).

## Retry and Degradation Semantics

- Clients MAY safely retry idempotent mutating calls with the same `Idempotency-Key`.
- If embedding provider is degraded, search MUST return lexical fallback plus diagnostics.
- Upstream provider failures MUST return stable machine codes and retryability flags.

## Related

- Types: [types.md](types.md)
- Errors: [errors.md](errors.md)
- Domain notes: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Domain search: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
- Security: [/docs/spec/security/README.md](/docs/spec/security/README.md)
