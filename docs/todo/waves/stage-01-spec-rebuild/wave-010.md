# Wave 010: Runtime and Workspace Bootstrap

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] scaffold workspace crates (`workspace`, `rbac`) and wire into app runtime
- [x] add base DB migrations for users, workspaces, membership, projects
- [x] expose readiness/liveness for new topology

## Verification Tasks

- [x] run migration and startup smoke checks
- [x] run T1 ownership invariants checks

## Evidence Placeholder

- [x] `Check: cargo workspace compile + migration smoke + readiness startup smoke + ownership invariants`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)`
