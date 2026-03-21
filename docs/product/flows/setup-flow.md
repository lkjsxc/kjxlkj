# Setup Flow

## Preconditions

- No admin user exists.

## Required Flow

1. If user visits `/` before setup completion, system redirects to `/setup`.
2. User visits `/setup` and receives the full setup page (not placeholder-only content).
3. User submits initial admin credentials.
4. System creates admin record and enables normal auth flow.

## Guardrails

- Setup endpoint must reject creation when an admin already exists.
- Setup endpoint must provide deterministic failure signals for automation.

## Completion Signal

- Subsequent requests follow login/admin route rules instead of setup-first routing.

## Cross-References

- [Setup-First Contract](../../vision/setup-first.md)
- [Access Control Contract](../policies/access-control.md)
- [Route Topology](../../architecture/runtime/route-topology.md)
