# Workspaces Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Workspace Lifecycle

| State | Meaning |
|---|---|
| Active | workspace is visible and writable based on role |
| Archived | read-only except owner/admin maintenance operations |
| Deleted | hidden from normal listing and detached from active UI |

## Ownership Rules

- Exactly one owner MUST exist for each workspace.
- Owner transfer MUST be explicit and auditable.
- Workspace slug MUST be unique in tenant scope.

## Membership Rules

- Membership is role-based (`owner`, `admin`, `editor`, `viewer`).
- Membership upsert MUST be idempotent by `(workspace_id, user_id)`.
- Removing membership MUST revoke access immediately for new requests.

## Related

- Permissions: [permissions.md](permissions.md)
- Projects: [projects.md](projects.md)
- HTTP contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
