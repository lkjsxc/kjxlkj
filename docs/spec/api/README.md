# API Specifications

Back: [/docs/spec/README.md](/docs/spec/README.md)

Normative external interfaces for HTTP and WebSocket clients.

## Documents

| Document | Purpose |
|---|---|
| [http.md](http.md) | REST endpoints and behavior |
| [websocket.md](websocket.md) | realtime protocol and replay semantics |
| [types.md](types.md) | payload schemas and enums |
| [errors.md](errors.md) | machine error model |
| [openapi.md](openapi.md) | OpenAPI governance |
| [openapi.yaml](openapi.yaml) | canonical OpenAPI document |
| [librarian-xml.md](librarian-xml.md) | attribute-less XML protocol for agent operations |

## Interface Principles

- API base path MUST be `/api`.
- WS endpoint MUST be `GET /ws`.
- Note IDs MUST remain immutable and independent from note titles.
- Search MUST support hybrid lexical+semantic retrieval modes.
- Agent automation MUST support JSON-defined prompt and KV memory policies.

## Related

- Domain behavior: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
- Security requirements: [/docs/spec/security/README.md](/docs/spec/security/README.md)
