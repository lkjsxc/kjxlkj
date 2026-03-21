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
- See [Setup-First Contract](../../vision/setup-first.md) for the invariant.
- See [Product Surface Map](../../product/surface-map.md) for route semantics.
