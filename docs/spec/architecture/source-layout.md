# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for derived runtime topology.

## Goals

- keep directories near 12 direct children
- split files before they exceed 200 lines
- keep transport, domain, and persistence concerns separated
- enforce typed boundaries end-to-end

## Canonical Derived Tree

| Path | Purpose |
|---|---|
| `src/backend/crates/app/` | application entrypoint and service wiring |
| `src/backend/crates/http/` | HTTP handlers and DTO mapping |
| `src/backend/crates/ws/` | WS protocol and stream handling |
| `src/backend/crates/domain/` | domain services and rules |
| `src/backend/crates/db/` | Postgres repositories and migrations |
| `src/backend/crates/security/` | auth/session/csrf/rbac logic |
| `src/backend/crates/automation/` | rule/run/librarian orchestration |
| `src/frontend/` | TypeScript app (`.ts`/`.tsx`) |
| `src/frontend/app/dist/` | generated web-delivery bundle artifacts |

## Layout Constraints

| Trigger | Required Action |
|---|---|
| directory has >12 direct children | split by focused subdomain |
| file >200 lines | extract cohesive module |
| mixed IO + domain logic | separate adapter and service layers |
| handwritten `.js` runtime source appears | replace with typed TypeScript module |

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Full structure contract: [final-file-structure.md](final-file-structure.md)
- Type safety: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
