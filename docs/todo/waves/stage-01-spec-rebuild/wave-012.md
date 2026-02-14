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

- [ ] implement global role update and workspace membership APIs -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] enforce route-level and domain-level authorization guards -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] emit auditable security events for role/membership mutations -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Verification Tasks

- [ ] run permission matrix tests across owner/admin/editor/viewer -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] run forbidden-path integration checks -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Evidence Placeholder

- [ ] `Check: role/membership mutation matrix with allowed and forbidden paths plus audit event assertions` -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] `Result: pass` -> [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)`
