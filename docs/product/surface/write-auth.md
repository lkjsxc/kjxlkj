# Write Authentication Contract

## Session Auth Rule

All write operations require one valid admin session:

- `POST /login` sets `session_id` after valid credentials.
- `POST /logout` clears `session_id`.
- `POST`, `PUT`, and `DELETE` on `/resources/*` require a valid non-expired session.
- `POST /admin/markdown-preview` requires a valid non-expired session.
- Admin credentials and sessions are persisted in PostgreSQL.

## Validation

- Missing, invalid, or expired session returns `401` on JSON and multipart write endpoints.
- HTML admin pages still redirect to `/login` when no valid session exists.
- Private resource pages and private file routes return `404` for unauthenticated readers.

## Security Boundaries

- Session IDs are random UUIDs and never emitted in JSON.
- Password hashes are never exposed in responses.
- Media file routes must respect the same visibility and snapshot-visibility rules as HTML pages.
