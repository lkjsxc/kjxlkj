# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- The canonical product pivot is complete at docs level only.
- Runtime artifacts are not yet available.
- All runtime requirement rows are open until reconstruction evidence exists.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-BOOTSTRAP-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | Actix/Tokio runtime not implemented | `M2 missing feature` | high | `E2E-01` | scaffold workspace and server bootstrap |
| `LIM-API-CORE-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | REST endpoints not implemented | `M2 missing feature` | high | `API-SETUP-01`, `API-NOTE-03` | implement API v1 contracts |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS patch stream not implemented | `M2 missing feature` | high | `WS-01..05` | implement authenticated WS note protocol |
| `LIM-DB-01` | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | SQLx schema/migrations absent | `M2 missing feature` | high | `API-NOTE-04`, `E2E-04` | add migration set and repositories |
| `LIM-AUTH-01` | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | setup/login/session/CSRF flows absent | `M2 missing feature` | high | `API-AUTH-02`, `E2E-01` | implement auth/session middleware |
| `LIM-DEPLOY-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | single-container compose path absent | `M2 missing feature` | high | `OPS-01` | add Dockerfile + compose + supervisor flow |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-OPENAPI-01` | [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) | canonical OpenAPI file exists but CI schema validation is not yet proven | `M4 verification gap` | medium | run OpenAPI validation in CI profile |
| `LIM-SPA-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | SPA assets not yet reconstructed | `M2 missing feature` | medium | build React/Vite shell and static hosting |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
