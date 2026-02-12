# Principles

Back: [/docs/overview/README.md](/docs/overview/README.md)

Core principles for the workspace-suite platform.

## Principles (normative)

| Principle | Meaning | Consequence |
|---|---|---|
| Deterministic writes | Note and automation mutations are serialized by target stream. | Stable ordering and replay semantics. |
| Async-first runtime | IO and long-running work are non-blocking Tokio tasks. | HTTP/WS responsiveness under load. |
| Event-sourced history | Write path appends immutable events. | Auditable history and rollback support. |
| Projection reads | Read APIs use projection tables. | Fast query, dashboard, and search behavior. |
| Explicit conflicts | Version mismatch returns deterministic `409` or WS reject. | No hidden last-write-wins corruption. |
| Docs-first governance | Docs are authoritative over implementation claims. | Traceable, reconstructable system behavior. |
| Small-model protocol compatibility | LLM interaction uses attribute-less XML-like envelopes. | Stable behavior across local and remote low-parameter models. |
| Security baseline | Auth, sessions, CSRF, and role checks are mandatory in browser flows. | Safe single-tenant multi-user operation. |

## Related

- Architecture: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- API: [/docs/spec/api/README.md](/docs/spec/api/README.md)
- Security: [/docs/spec/security/README.md](/docs/spec/security/README.md)
