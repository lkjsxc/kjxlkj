# Public Site Flow

## Logged-Out Behavior

- Before first admin exists, `GET /` redirects to `/setup`.
- After setup completion, `GET /` shows non-private articles only.
- `GET /article/{slug}` returns 404 for private articles.

## Logged-In Admin Behavior

- Admin can view both public and private articles.
- Public URLs remain unchanged regardless of session state.

## Rendering Rules

- Markdown is rendered server-side.
- Rendered HTML is sanitized before response.
- Article pages should expose stable canonical links.
