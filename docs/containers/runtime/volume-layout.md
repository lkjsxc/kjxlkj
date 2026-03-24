# Runtime Volume Layout

## Host Mount Root

All host-mounted runtime state is rooted under `./data`.

## Record Storage Path

- Host: `./data/records`
- Container: `/app/data/records`

## Auth/Session Storage Path

- Auth/session state is persisted in PostgreSQL referenced by `DATABASE_URL`.
- No filesystem fallback for auth/session persistence.

## Git Rule

`data/` is ignored by git.
