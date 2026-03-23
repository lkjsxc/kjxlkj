# Runtime Volume Layout

## Host Mount Root

All host-mounted runtime state is rooted under `./data`.

## Record Storage Path

- Host: `./data/records`
- Container: `/app/data/records`

## Git Rule

`data/` is ignored by git.
