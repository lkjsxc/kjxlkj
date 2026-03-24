# Write Authentication Contract

## Record API Rule

Record API write endpoints require header `x-admin-token`.

## Session Auth Rule

HTML setup/login/admin flow uses cookie sessions:

- `POST /login` sets `session_id` cookie after valid password.
- `POST /logout` clears `session_id`.
- `/admin` requires a valid non-expired `session_id` and redirects to `/login` when absent.
- Admin credentials and session rows are persisted in PostgreSQL (`DATABASE_URL`).
- Runtime startup fails when database connectivity is unavailable.

## Validation

- Expected token value comes from `ADMIN_TOKEN` environment variable.
- Missing or mismatched token returns `401` JSON error on record write endpoints.
- Session cookie is `HttpOnly`, `Path=/`, and cleared with `Max-Age=0` on logout.
- Record read endpoints never require token.

## Security Boundaries

- Token is compared exactly as UTF-8 string.
- Token is never echoed in response payloads.
- Session ID is random UUID and never emitted in JSON responses.
