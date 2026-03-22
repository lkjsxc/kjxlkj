# Admin Editor Flow

See [Product Surface Map](../surface-map.md) for endpoint scope.
See [Access Control Contract](../policies/access-control.md) for route/session requirements.

## Access Rules

- `/admin` requires a valid authenticated admin session after setup completion.
- Unauthorized access is redirected to `/login` after setup completion.
- Before setup completion, `/admin` redirects to `/setup`.

## Contract Decomposition

- Full server-rendered page contracts: [page-contracts.md](page-contracts.md).
- HTMX admin request/fragment contracts: [admin-htmx-contracts.md](admin-htmx-contracts.md).
- JavaScript UX contracts (autosave, guards, shortcuts): [admin-js-ux-contract.md](admin-js-ux-contract.md).
- Conflict warning behavior: [admin-conflict-warning.md](admin-conflict-warning.md).
- Split-view direct editing behavior: [direct-edit-mode.md](direct-edit-mode.md).

## Core Capabilities

- List Markdown articles.
- Open and edit Markdown content.
- Render server-side preview pane updates through HTMX.
- Save content atomically with last-write-wins conflict handling.
- Create, rename, and delete Markdown files.
- Toggle frontmatter `private` visibility.
- Use deterministic split-view direct-edit mode.
- Move deleted items to recoverable trash.

## Endpoint Surface

- `GET /admin`
- `GET /admin/open/{slug}`
- `POST /admin/preview`
- `POST /admin/create`
- `POST /admin/save`
- `POST /admin/rename`
- `POST /admin/delete/{slug}`
- `POST /admin/toggle-private/{slug}`

## Role Constraints

- Non-admin users are read-only and do not have editing surfaces.
- Private articles are hidden for non-admin users across navigation and search surfaces.
