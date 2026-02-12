# API Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Canonical Entry Points

- HTTP: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- WebSocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
- Errors: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- Librarian protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Integration Notes

- API base path is `/api`.
- Authenticated browser flows use cookie session + CSRF.
- Real-time updates use `GET /ws`.
- Librarian automation rules are created via `/api/automation/rules` with
  `action_json.kind = \"librarian_structure\"`.

## Related

- Security specs: [/docs/spec/security/README.md](/docs/spec/security/README.md)
- Domain specs: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
