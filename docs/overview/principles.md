# Principles

Back: [/docs/overview/README.md](/docs/overview/README.md)

Core principles for the workspace-suite platform.

## Principles

| Principle | Meaning | Consequence |
|---|---|---|
| All in Docs governance | Docs are the canonical product. | Derived runtime can be rebuilt or discarded safely. |
| Deterministic writes | Mutations serialize by target stream identity. | Stable ordering and replay semantics. |
| Async-first runtime | IO and long-running work are non-blocking. | HTTP/WS responsiveness under load. |
| Explicit conflict semantics | Version mismatch is deterministic (`409` or WS reject). | No hidden last-write-wins corruption. |
| Typed boundaries only | Runtime source is statically typed end-to-end. | Lower ambiguity and safer refactors. |
| Security baseline | Auth, sessions, CSRF, and RBAC are mandatory. | Safe multi-user operation. |
| Evidence-backed closure | Completion claims require deterministic proof. | No status drift between docs and reality. |

## Related

- Architecture: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- API: [/docs/spec/api/README.md](/docs/spec/api/README.md)
- Security: [/docs/spec/security/README.md](/docs/spec/security/README.md)
