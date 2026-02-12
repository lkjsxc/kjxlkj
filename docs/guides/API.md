# API Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Canonical Entry Points

- HTTP: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- WebSocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
- Errors: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Integration Notes

- API version is `/api/v1`.
- Authenticated browser flows use cookie session + CSRF.
- Real-time note updates use `GET /ws/v1/notes`.

## Related

- Security specs: [/docs/spec/security/README.md](/docs/spec/security/README.md)
- Domain specs: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
