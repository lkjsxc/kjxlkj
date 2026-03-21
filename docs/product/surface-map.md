# Product Surface Map

## Public Surface

- `GET /` lists visible articles after setup completion; before any admin exists, it redirects to `/setup`.
- `GET /article/{slug}` renders a single visible article.

## Authentication Surface

- `GET /setup` renders the complete first-admin setup page when none exists.
- `GET /login` authenticates existing admin after setup completion.
- `POST /logout` destroys the admin session.

## Admin Surface

- `GET /admin` renders editor shell for authenticated admin.
- `POST /admin/*` mutates content and visibility according to policy.

## Contract Priority

- Setup-first rules override normal auth entry rules until setup is complete.
