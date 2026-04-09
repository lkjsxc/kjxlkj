# Alias Rules

## Canonical Term

- The canonical human-managed route field is `alias`.

## Format

- Raw alias input is trimmed and normalized before validation.
- `alias` is stored as lowercase ASCII.
- Allowed characters are `a-z`, `0-9`, `-`, `_`, and `.`.
- Direct typing or paste must preserve internal `-`, `_`, and `.` separators.
- Aliases may not begin or end with a separator.
- Consecutive separators are rejected even when they differ, such as `-.`, `._`, or `__`.
- Whitespace normalizes to `-` before validation.

## Routing

- The canonical resource page route is `/{ref}`.
- When `alias` is present, the canonical `ref` is the alias.
- When `alias` is absent, the canonical `ref` is the `id`.
- History routes append `/history` after the same canonical `ref`.

## Reservation Rules

- Aliases must be unique among live resources.
- Aliases may not match reserved application paths such as `admin`, `search`, `login`, `logout`, `setup`, `resources`, `assets`, `robots.txt`, `sitemap.xml`, and `healthz`.
- Aliases may not equal the ID format.
