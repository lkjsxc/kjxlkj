# Permissions Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Role Set

| Role | Required capability envelope |
|---|---|
| `owner` | full control including membership and ownership transfer |
| `admin` | manage workspace settings, members, projects, automation |
| `editor` | create/edit/delete notes, views, and dashboard widgets |
| `viewer` | read-only access to allowed workspace content |

## Authorization Rules

- Route-level authorization MUST validate authenticated identity and role.
- Domain-level authorization MUST re-validate permission before mutation commit.
- Denied operations MUST return deterministic `403` with stable error code.
- Authorization checks MUST include workspace scope, and project scope when present.

## Audit Rules

- Mutations affecting membership, roles, note deletion, and automation MUST emit
 auditable events with `request_id`, `actor_id`, and `workspace_id`.

## Related

- Workspaces: [workspaces.md](workspaces.md)
- Automation: [automation.md](automation.md)
- Error contract: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
