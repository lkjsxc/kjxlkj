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
- `/admin/*` mutation and content operations

## Routing Order Constraint

- Setup-first gating must run before normal login/admin routing when no admin user exists.
- Before setup completion, root-route handling for `GET /` must redirect to `/setup`.
- Before setup completion, `GET /setup` must render the complete setup page contract.
- See [Setup-First Contract](../../vision/setup-first.md) for the invariant.
- See [Product Surface Map](../../product/surface-map.md) for route semantics.
