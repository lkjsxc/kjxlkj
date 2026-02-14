# Evidence Index

Back: [/docs/reference/README.md](/docs/reference/README.md)

Deterministic mapping from reconstruction stages to required proof artifacts.

## Purpose

This index defines what evidence must exist before each stage can be marked
complete in TODO ledgers and promoted in conformance ledgers.

## Evidence Rules

- Evidence MUST be deterministic and reproducible.
- Evidence MUST reference acceptance IDs from
  [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).
- Evidence MUST be reflected in
  [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md),
  [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
  [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md).
- Wave completion in [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
  MUST not outpace referenced evidence.

## Stage Evidence Map

| Stage | Scope | Primary TODO | Required Proof Contract | Status |
|---|---|---|---|---|
| Stage 00 | governance baseline and canonical docs integrity | [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md) | docs reachability, structure, and final-file-structure checks | complete |
| Stage 01 | runtime skeleton, auth/session, RBAC foundations | [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md) | `API-AUTH-*`, role/membership boundary checks, type gates | complete |
| Stage 02 | notes/event streams/search core | [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md) | `API-NOTE-*`, `API-SEARCH-*`, `WS-01..05` | complete |
| Stage 03 | web shell/editor/responsive interaction baseline | [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md) | `E2E-06`, `E2E-07`, `E2E-08`, `E2E-12`, `E2E-13`, `E2E-14` | complete |
| Stage 04 | automation, jobs, and projection consistency | [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md) | `API-AUTO-01`, `API-AUTO-02`, `OPS-01` | complete |
| Stage 05 | security, reliability, and recovery hardening | [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md) | `REG-IMP-*`, `REG-USR-*`, `PERF-*`, `OPS-02` | complete |
| Stage 06 | REST contract completion and librarian provider/prompt path | [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md) | `API-AUTO-03`, `API-AUTO-04`, OpenAPI parity checks | complete |
| Stage 07 | workspace realtime sync/replay closure | [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md) | `WS-06`, replay/idempotency/stale-cursor boundaries | complete |
| Stage 08 | frontend librarian UX and static delivery | [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md) | `E2E-15`, responsive + accessibility checks | complete |
| Stage 09 | CI/perf/release closure | [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md) | full profile pass + release gate closure | in progress |

## Verification Profiles

- Profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
- Acceptance IDs: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release closure: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Related

- Runtime trust state: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Open gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
