# API Specifications

Back: [/docs/spec/README.md](/docs/spec/README.md)

Normative external interfaces for HTTP and WebSocket clients.

## Documents

| Document | Purpose |
|---|---|
| [http.md](http.md) | REST endpoints and semantics |
| [websocket.md](websocket.md) | WebSocket protocol and message flow |
| [types.md](types.md) | External payload schemas |
| [errors.md](errors.md) | Error model and status code contract |
| [openapi.md](openapi.md) | OpenAPI source-of-truth contract |
| [librarian-xml.md](librarian-xml.md) | attribute-less XML-like LLM protocol contract |

## Interface Principles

- API base path MUST be `/api`.
- WebSocket endpoint MUST be `GET /ws`.
- JSON payloads MUST be UTF-8 and schema-valid.
- Write endpoints MUST enforce optimistic version checks where applicable.
- WebSocket ordering MUST reflect committed event sequence.
- Librarian LLM envelopes MUST follow the attribute-less XML-like protocol in
  [librarian-xml.md](librarian-xml.md).

## Related

- Domain behavior: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
- Security requirements: [/docs/spec/security/README.md](/docs/spec/security/README.md)
