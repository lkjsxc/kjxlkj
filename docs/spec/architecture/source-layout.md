# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for implementation topology and sizing constraints.

## Goals

- keep directories near 12 direct children
- split files before they exceed 200 lines
- keep API, domain, and persistence concerns separated

## Canonical Tree

| Path | Purpose |
|---|---|
| `src/crates/app/kjxlkj-server/` | application entrypoint and route wiring |
| `src/crates/http/kjxlkj-http/` | HTTP handlers, DTO mapping, middleware |
| `src/crates/ws/kjxlkj-ws/` | WebSocket protocol and session routing |
| `src/crates/domain/kjxlkj-domain/` | note/event/record business rules |
| `src/crates/db/kjxlkj-db/` | SQLx repositories and migrations integration |
| `src/crates/auth/kjxlkj-auth/` | setup/login/session/CSRF logic |
| `src/crates/search/kjxlkj-search/` | FTS and backlink query services |

## Layout Constraints

| Trigger | Required Action |
|---|---|
| directory has >12 direct children | split by focused subdomain |
| source file >200 lines | extract cohesive module |
| mixed IO + domain logic | separate repository and service layers |

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Workspace policy: [workspace-manifest.md](workspace-manifest.md)
