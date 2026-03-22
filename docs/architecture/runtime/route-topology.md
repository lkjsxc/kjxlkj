# Route Topology

## Public Routes

- `/`
- `/article/{slug}`

## Auth Routes

- `/setup`
- `/login`
- `/logout`

## Admin Routes

- `/admin`
- `/admin/open/{slug}`
- `/admin/preview`
- `/admin/create`
- `/admin/save`
- `/admin/rename`
- `/admin/delete/{slug}`
- `/admin/toggle-private/{slug}`

## Route Mode Constraint

- `GET /admin` returns a complete page for non-HTMX navigation.
- Admin interaction routes return HTMX fragments when `HX-Request: true` is present.

## Routing Order Constraint

- Setup-first gating must run before normal login/admin routing when no admin user exists.
- Before setup completion, root-route handling for `GET /` must redirect to `/setup`.
- Before setup completion, `GET /setup` must render the complete setup page contract.
- See [Setup-First Contract](../../vision/setup-first.md) for the invariant.
- See [Product Surface Map](../../product/surface-map.md) for route semantics.
