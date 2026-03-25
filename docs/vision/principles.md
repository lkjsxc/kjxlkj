# Core Principles

## Design Principles

1. Direct access: Notes served at clean URLs (`/{slug}`).
2. Live editing: Changes saved automatically on blur.
3. Revision history: Every change creates a new revision.
4. Privacy by default: Notes are private unless explicitly made public.

## Build Principles

- Rust is the only implementation language.
- PostgreSQL is the single data store.
- Docker Compose is the verification transport.
- Tests and quality gates are mandatory before acceptance.
