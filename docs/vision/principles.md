# Core Principles

## Design Principles

1. Direct access: every note has one canonical root-path URL at `/{ref}`.
2. Homepage first: `/` feels like a home surface, not a bare browse dump.
3. Hybrid admin: `/admin` combines dashboard insight, settings, and a scalable library.
4. Live editing: heading-, alias-, favorite-, and visibility-derived chrome updates without reload.
5. Privacy by default: notes remain private until `Public` is checked.

## Build Principles

- Rust remains the only implementation language.
- PostgreSQL remains the only datastore.
- Docker Compose remains the verification transport.
- Browser-local rendering must stay deterministic under automated verification.
