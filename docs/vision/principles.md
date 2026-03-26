# Core Principles

## Design Principles

1. Direct access: every note has an opaque stable URL at `/{id}`.
2. Scalable browsing: public and admin indexes are searchable, paginated, and dense.
3. Live editing: heading- and visibility-derived chrome updates without reload.
4. Privacy by default: notes remain private until `Public` is checked.

## Build Principles

- Rust remains the only implementation language.
- PostgreSQL remains the only datastore.
- Docker Compose remains the verification transport.
- Browser-local rendering must stay deterministic under automated verification.
