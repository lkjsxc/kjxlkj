# Backend Crate Decomposition

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Backend runtime is a Rust workspace under `src/backend/crates/` when derived runtime exists.

## Required Crate Groups

| Group | Purpose |
|---|---|
| app | process startup and dependency wiring |
| http | REST transport layer |
| ws | realtime transport layer |
| domain | business logic and invariants |
| db | SQLite repositories and migrations |
| security | auth/session/csrf/rbac services |
| automation | rule and librarian orchestration |

## Decomposition Rules

- runtime wiring MUST stay in `app`
- HTTP/WS transport code MUST stay outside domain core logic
- DB repositories MUST stay isolated from transport DTOs
- authorization decisions MUST flow through typed security services
- librarian/provider adapters MUST stay in automation services

## Related

- Source layout: [source-layout.md](source-layout.md)
- Runtime model: [runtime.md](runtime.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
