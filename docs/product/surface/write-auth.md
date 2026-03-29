# Write Authentication Contract

## Session Auth Rule

All write operations require a valid admin session:

- `POST /login` sets `session_id` cookie after valid password.
- `POST /logout` clears `session_id` and redirects to `/login`.
- Write endpoints (`POST`, `PUT`, `DELETE` on `/records/*`) require a valid non-expired `session_id`.
- Admin credentials and session rows are persisted in PostgreSQL (`DATABASE_URL`).
- Runtime startup fails when database connectivity is unavailable.

## Validation

- Session cookie is `HttpOnly`, `Path=/`, and cleared with `Max-Age=0` on logout.
- Missing or invalid session returns `401` JSON error on record write endpoints.
- Expired session returns `401` JSON error.
- Note viewing respects `is_private` flag: private notes return `404` for unauthenticated users.

## Security Boundaries

- Session ID is random UUID and never emitted in JSON responses.
- Password hash never exposed in responses.
- Session ID is compared exactly as UTF-8 string.
