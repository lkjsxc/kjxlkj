# Canonical Specifications

Back: [/docs/README.md](/docs/README.md)

`/docs/spec/` defines target behavior for the web-notes platform.

## Authority

- `spec` defines what the implementation MUST do.
- `reference` defines what is currently verified.
- `todo` defines staged execution toward conformance.

## Intra-Spec Conflict Rule

If two spec files conflict:

1. more specific leaf spec overrides parent index pages
2. safety and determinism constraints in `technical/` override convenience behavior
3. architecture invariants override local feature mechanics
4. unresolved conflicts MUST be logged in `reference/LIMITATIONS.md`

## Spec Domains

| Domain | Scope |
|---|---|
| [architecture/](architecture/README.md) | Runtime, workspace, deployment shape |
| [api/](api/README.md) | HTTP and WebSocket contracts |
| [domain/](domain/README.md) | Notes, events, metadata, attachments, search |
| [security/](security/README.md) | Auth, sessions, CSRF, transport policy |
| [technical/](technical/README.md) | Testing, performance, migrations, operations |
| [ui/](ui/README.md) | Web app interaction and static hosting boundary |

## Normative Language

| Term | Meaning |
|---|---|
| `MUST` | Required |
| `MUST NOT` | Forbidden |
| `SHOULD` | Preferred unless justified |
| `MAY` | Optional |

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Current-state evidence: [/docs/reference/README.md](/docs/reference/README.md)
- Execution controls: [/docs/todo/README.md](/docs/todo/README.md)
