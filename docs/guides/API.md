# API Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Canonical Entry Points

- HTTP: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- WebSocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
- Errors: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- Librarian protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Integration Notes

- API base path is `/api`
- browser flows use cookie session + CSRF
- real-time updates use `GET /ws`
- clients SHOULD consume API using typed contracts (Rust structs / TypeScript interfaces)

## Related

- Security specs: [/docs/spec/security/README.md](/docs/spec/security/README.md)
- Type safety contract: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Domain specs: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
