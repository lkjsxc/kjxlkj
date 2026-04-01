# Core Principles

## Design Principles

1. Direct access: every note has one canonical root-path URL at `/{ref}`.
2. Homepage first: `/` feels like a home surface, not a bare browse dump.
3. Split admin work cleanly: `/admin` is overview and action entry, `/admin/settings` is canonical configuration.
4. Live editing: heading-, alias-, favorite-, visibility-, and preview-derived chrome updates happen without reload.
5. Privacy defaults stay explicit: initial installs default to private notes, but new-note visibility is configurable.

## Build Principles

- Rust remains the only implementation language.
- PostgreSQL remains the only datastore.
- Docker Compose remains the verification transport.
- Browser-local rendering must stay deterministic under automated verification.
