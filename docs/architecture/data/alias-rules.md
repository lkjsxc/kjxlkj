# Alias Rules

## Canonical Term

- The canonical human-managed route field is `alias`.

## Format

- `alias` is lowercase ASCII.
- Allowed characters are `a-z`, `0-9`, `.`, `_`, and `-`.
- Aliases may not begin or end with `.`, `_`, or `-`.
- Repeated separator runs are rejected.

## Routing

- The canonical note page route is `/{ref}`.
- When `alias` is present, the canonical `ref` is the alias.
- When `alias` is absent, the canonical `ref` is the `id`.
- History routes append `/history` after the same canonical `ref`.

## Reservation Rules

- Aliases must be unique among live notes.
- Aliases may not match reserved application paths such as `admin`, `search`, `settings`, `preview`, `login`, `logout`, `setup`, `records`, `assets`, and `healthz`.
- Aliases may not equal the ID format.
