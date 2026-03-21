# Setup Flow

## Preconditions

- No admin user exists.

## Required Flow

1. User visits `/setup`.
2. User submits initial admin credentials.
3. System creates admin record and enables normal auth flow.

## Guardrails

- Setup endpoint must reject creation when an admin already exists.
- Setup endpoint must provide deterministic failure signals for automation.

## Completion Signal

- Subsequent requests follow login/admin route rules instead of setup-first routing.

## Cross-References

- [Setup-First Contract](../../vision/setup-first.md)
- [Access Control Contract](../policies/access-control.md)
- [Route Topology](../../architecture/runtime/route-topology.md)
