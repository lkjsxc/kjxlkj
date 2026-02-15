# Wave Program

Back: [/docs/todo/README.md](/docs/todo/README.md)

Ordered implementation workflow for reconstructing runtime artifacts from
canonical documentation.

## Relevant Documents

- [/docs/policy/INSTRUCT.md](/docs/policy/INSTRUCT.md)
- [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
- [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)

## Orientation References

- [/docs/guides/README.md](/docs/guides/README.md)
- [/docs/guides/QUICKSTART.md](/docs/guides/QUICKSTART.md)
- [/docs/guides/API.md](/docs/guides/API.md)
- [/docs/guides/LIBRARIAN.md](/docs/guides/LIBRARIAN.md)
- [/docs/guides/RECONSTRUCTION_BOOTSTRAP.md](/docs/guides/RECONSTRUCTION_BOOTSTRAP.md)
- [/docs/overview/README.md](/docs/overview/README.md)
- [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
- [/docs/overview/glossary.md](/docs/overview/glossary.md)
- [/docs/overview/principles.md](/docs/overview/principles.md)

## Execution Rules

- [x] restructure-step RULE-01: execute stages only in listed order per [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md) [doc-link](/docs/policy/WORKFLOW.md)
- [x] restructure-step RULE-02: execute waves only in listed order within each stage per [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md) [doc-link](/docs/policy/WORKFLOW.md)
- [x] restructure-step RULE-03: require deterministic evidence per wave from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step RULE-04: synchronize ledgers on each wave transition in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Ordered Stages

- [x] restructure-step S00: Governance and Canonical Baseline in [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md) [doc-link](/docs/todo/waves/stage-00-pivot-governance/README.md)
- [x] restructure-step S01: Workspace and Auth Foundation in [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md) [doc-link](/docs/todo/waves/stage-01-spec-rebuild/README.md)
- [x] restructure-step S02: Notes and Realtime Core in [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md) [doc-link](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)
- [x] restructure-step S03: Web App Shell and Editor UX in [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md) [doc-link](/docs/todo/waves/stage-03-single-container-runtime/README.md)
- [x] restructure-step S04: Schema, Automation, and Jobs in [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md) [doc-link](/docs/todo/waves/stage-04-schema-and-projections/README.md)
- [x] restructure-step S05: Security, Reliability, and Recovery in [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md) [doc-link](/docs/todo/waves/stage-05-auth-and-security/README.md)
- [x] restructure-step S06: REST and Librarian Provider Completion in [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md) [doc-link](/docs/todo/waves/stage-06-rest-api/README.md)
- [x] restructure-step S07: WebSocket Replay and Automation Events in [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md) [doc-link](/docs/todo/waves/stage-07-websocket-sync/README.md)
- [x] restructure-step S08: Frontend Delivery and Responsive Closure in [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md) [doc-link](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)
- [x] restructure-step S09: CI, Drift Closure, and Release in [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md) [doc-link](/docs/todo/waves/stage-09-ci-performance-release/README.md)
- [x] restructure-step S10: Hardening and Improvement Backlog in [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md) [doc-link](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)

## Program Exit

- [x] restructure-step EXIT-01: confirm no unresolved high-severity blockers in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/LIMITATIONS.md)
- [x] restructure-step EXIT-02: confirm no unresolved high-severity `M1`/`M2` rows in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)
- [x] restructure-step EXIT-03: confirm release closure in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) [doc-link](/docs/reference/RELEASE.md)
