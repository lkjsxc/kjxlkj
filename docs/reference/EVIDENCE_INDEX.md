# Evidence Index

Back: [/docs/reference/README.md](/docs/reference/README.md)

Deterministic mapping from TODO stages to required proof artifacts.

## Evidence Rules

- Evidence MUST be reproducible.
- Evidence MUST reference acceptance IDs in
  [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).
- TODO completion MUST not outpace evidence capture.

## Stage Evidence Map

| Stage | Scope | Primary TODO | Required Proof | Status |
|---|---|---|---|---|
| `S00` | governance baseline | [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md) | docs integrity checks | `complete` |
| `S01` | runtime scaffold | [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md) | build and type gates | `complete` |
| `S02` | notes + search | [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md) | `API-NOTE-*`, `API-SEARCH-*` | `complete` |
| `S03` | runtime integration | [/docs/todo/waves/stage-03-runtime-integration/README.md](/docs/todo/waves/stage-03-runtime-integration/README.md) | DB and service integration tests | `complete` |
| `S04` | automation + agent | [/docs/todo/waves/stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md) | `API-AUTO-*`, `AGENT-*` | `complete` |
| `S05` | security closure | [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md) | auth/session/csrf tests | `partial` |
| `S06` | REST contract closure | [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md) | API acceptance set | `complete` |
| `S07` | websocket sync | [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md) | WS replay/idempotency set | `complete` |
| `S08` | frontend and hosting | [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md) | E2E + build checks | `partial` |
| `S09` | CI and release | [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md) | full profile pass | `complete` |
| `S10` | hardening backlog | [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md) | targeted hardening proofs | `partial` |

## Related

- CI profiles: [CI.md](CI.md)
- Release gate: [RELEASE.md](RELEASE.md)
