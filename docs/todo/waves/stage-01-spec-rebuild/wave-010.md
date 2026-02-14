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

- [ ] scaffold workspace crates (`workspace`, `rbac`) and wire into app runtime -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] add base DB migrations for users, workspaces, membership, projects -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] expose readiness/liveness for new topology -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Verification Tasks

- [ ] run migration and startup smoke checks -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] run T1 ownership invariants checks -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Evidence Placeholder

- [ ] `Check: cargo workspace compile + migration smoke + readiness startup smoke + ownership invariants` -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] `Result: pass` -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)`
