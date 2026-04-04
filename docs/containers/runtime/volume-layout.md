# Runtime Volume Layout

## Runtime Persistence

- Runtime note, revision, and session state is persisted in PostgreSQL.
- Compose maps PostgreSQL state to the named volume `kjxlkj-postgres-data`.
- The app container does not require a writable note-storage filesystem path.

## Verification Output

- Browser verification writes screenshots to `tmp/visual-artifacts/`.
- Rust verification caches use `kjxlkj-verify-cargo` and `kjxlkj-verify-target`.
- Those artifacts are disposable verification output rather than runtime state.

## Git Rule

- Runtime persistence is external to git.
- Verification artifacts may be regenerated at any time.
