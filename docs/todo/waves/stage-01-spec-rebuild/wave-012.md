# Wave 012: RBAC and Membership Controls

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] implement global role update and workspace membership APIs
- [x] enforce route-level and domain-level authorization guards
- [x] emit auditable security events for role/membership mutations

## Verification Tasks

- [x] run permission matrix tests across owner/admin/editor/viewer
- [x] run forbidden-path integration checks

## Evidence Placeholder

- [x] `Check: role/membership mutation matrix with allowed and forbidden paths plus audit event assertions`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)`
