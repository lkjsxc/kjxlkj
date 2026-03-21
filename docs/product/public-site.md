# Public Site Behavior

## Logged-Out Experience

- `GET /` shows a list of non-private articles.
- `GET /article/{slug}` renders one article if it is public.
- Requesting a private article while logged out returns 404.

## Logged-In Experience

- Logged-in admin sees both public and private content.
- Admin access does not change public URLs.

## Rendering

- Markdown is converted to HTML on the server.
- Rendered HTML is sanitized before response.
- The page includes canonical links and readable typography.
