# Public Site Flow

## Logged-Out Behavior

- Before first admin exists, `GET /` redirects to `/setup`.
- After setup completion, `GET /` shows only non-private articles (articles are private by default).
- `GET /article/{slug}` returns 404 for private articles.
- `GET /search` is available and only returns public matches.

## Logged-In Admin Behavior

- Admin can view both public and private articles.
- Public URLs remain unchanged regardless of session state.
- `GET /search` may include public and private matches.
- Article pages include inline editor controls for admin only.
- Article history pages are admin-only.

## Rendering Rules

- Markdown is rendered server-side.
- Rendered HTML is sanitized before response.
- Article pages should expose stable canonical links.
- Shared navigation shell is rendered on page surfaces after setup completion.
- Article page shows last updated date and previous/next links.
- Article pages must not display author attribution/byline metadata.
