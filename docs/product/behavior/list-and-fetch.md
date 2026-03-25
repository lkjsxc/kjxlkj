# List and Fetch Behavior

## List (Admin Dashboard)

- Returns all notes for authenticated admin.
- Returns only public notes for unauthenticated users.
- Sort order is by `updated_at` descending (most recent first).
- Response includes: slug, body (first 200 chars for preview), is_private, updated_at.
- Cursor-based pagination using `updated_at` + `slug` as cursor.

## Fetch (`GET /{slug}`)

- Returns full note content if accessible.
- Returns `404` if note does not exist.
- Returns `404` if note is private and user is not authenticated.
- Response must include `body`, `is_private`, `created_at`, `updated_at`.
