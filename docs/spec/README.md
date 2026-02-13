# Canonical Specifications

Back: [/docs/README.md](/docs/README.md)

`/docs/spec/` defines target behavior for runtime reconstructions.

## Authority

- `spec` defines what implementations MUST do.
- `reference` defines what is currently verified.
- `todo` defines staged execution toward conformance.

## Intra-Spec Conflict Rule

If spec files conflict:

1. more specific leaf spec overrides index pages
2. safety and determinism constraints in `technical/` override convenience behavior
3. architecture invariants override local feature mechanics
4. unresolved conflicts MUST be logged in `reference/LIMITATIONS.md`

## Spec Domains

| Domain | Scope |
|---|---|
| [architecture/](architecture/README.md) | runtime shape, deployment, layout |
| [api/](api/README.md) | HTTP and WebSocket contracts |
| [domain/](domain/README.md) | notes, events, metadata, attachments, search |
| [security/](security/README.md) | auth, sessions, CSRF, transport policy |
| [technical/](technical/README.md) | testing, typing, performance, migrations, operations |
| [ui/](ui/README.md) | web app behavior and interaction contracts |

## Normative Language

| Term | Meaning |
|---|---|
| `MUST` | required |
| `MUST NOT` | forbidden |
| `SHOULD` | preferred unless justified |
| `MAY` | optional |

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Current state: [/docs/reference/README.md](/docs/reference/README.md)
- Execution controls: [/docs/todo/README.md](/docs/todo/README.md)
