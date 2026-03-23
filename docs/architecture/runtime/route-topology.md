# Route Topology

## Public Routes

- `/`
- `/healthz`
- `/article/{slug}`
- `/article/{slug}/history`
- `/search`

## Auth Routes

- `/setup`
- `/login`
- `/logout`

## Admin Routes

- `/admin`
- `/admin/create`
- `/admin/rename`
- `/admin/delete/{slug}`
- `/admin/settings`
- `/admin/settings/save`
- `/admin/settings/reindex`
- `/admin/trash`
- `/admin/trash/restore/{slug}`
- `/admin/trash/delete-permanent/{slug}`

## Inline Edit Routes

- `/article/{slug}/edit`
- `/article/{slug}/history/restore`

## Routing Constraints

- Setup-first gate runs before normal auth/admin flow.
- Inline editing is admin-only and mounted on article routes.
