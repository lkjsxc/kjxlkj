# Architecture

Back: [/docs/spec/README.md](/docs/spec/README.md)

## Documents

| Document | Purpose |
|---|---|
| [runtime.md](runtime.md) | runtime topology and supervision model |
| [deployment.md](deployment.md) | baseline docs launch and runtime target deployment |
| [source-layout.md](source-layout.md) | derived runtime topology constraints |
| [workspace-manifest.md](workspace-manifest.md) | manifest policy for Rust + TypeScript stack |
| [crates.md](crates.md) | backend Rust crate decomposition |

## System Shape (Runtime Target)

```mermaid
graph TD
 HTTP[Typed HTTP API]
 WS[Typed WebSocket]
 APP[Rust Services]
 DB[(PostgreSQL)]
 UI[TypeScript Web App]

 UI --> HTTP
 UI --> WS
 HTTP --> APP
 WS --> APP
 APP --> DB
```

## Invariants

- Request handling MUST be async and non-blocking.
- Mutation ordering MUST be deterministic per stream identity.
- App and PostgreSQL runtime target remains one compose service.
- Frontend/backend boundaries MUST be typed and versioned.

## Related

- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Domain model: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
