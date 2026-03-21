# Product Surface Map

## Public Surface

- `GET /` lists visible articles.
- `GET /article/{slug}` renders a single visible article.

## Authentication Surface

- `GET /setup` initializes first admin when none exists.
- `GET /login` authenticates existing admin after setup completion.
- `POST /logout` destroys the admin session.

## Admin Surface

- `GET /admin` renders editor shell for authenticated admin.
- `POST /admin/*` mutates content and visibility according to policy.

## Contract Priority

- Setup-first rules override normal auth entry rules until setup is complete.
