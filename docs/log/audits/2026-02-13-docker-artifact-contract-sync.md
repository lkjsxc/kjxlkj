# Docker Artifact Contract Sync Audit (2026-02-13)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

Deterministic sync pass to align Docker documentation with the current app
runtime container and verified startup flow.

## Canonical Sources Used

- [/README.md](/README.md)
- [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/README.md](/docs/reference/README.md)

## Consistency Matrix

| Requirement ID | Canonical Document | Requirement Statement | Code Path(s) | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|---|
| `AUD-DOCKER-01` | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | Docker guide must describe app runtime container (not docs static server) | `/docs/guides/DOCKER.md` | n/a | aligned | `M5 stale docs` (closed) | `spec-update` | guide now documents service `kjxlkj`, `/api/healthz`, `/api/readyz`, and SQLite volume |
| `AUD-DOCKER-02` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | deployment spec baseline must match compose service and healthcheck behavior | `/docs/spec/architecture/deployment.md` | `docker compose config` | aligned | `M5 stale docs` (closed) | `spec-update` | compose template updated to `kjxlkj` service with `/api/readyz` healthcheck |
| `AUD-DOCKER-03` | [/docs/todo/README.md](/docs/todo/README.md) | TODO Docker gate must verify app runtime endpoints, not docs endpoint | `/docs/todo/README.md` | smoke command set | aligned | `M5 stale docs` (closed) | `spec-update` | Docker gate now requires `/api/healthz` and `/api/readyz` checks |
| `AUD-DOCKER-04` | `/Dockerfile`, `/docker-compose.yml`, `/.dockerignore` | root Docker artifacts must build and run `kjxlkj-server` | `/Dockerfile`, `/docker-compose.yml`, `/.dockerignore` | compose config/build/run smoke | aligned | `M2 missing feature` (closed) | `implement` | app container builds, starts healthy, and serves runtime endpoints |
| `AUD-DOCKER-05` | [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | conformance snapshot must reflect verified app-runtime startup checks | `/docs/reference/CONFORMANCE.md`, `/docs/reference/README.md` | n/a | aligned | `M5 stale docs` (closed) | `spec-update` | snapshot updated to app-runtime artifacts + partial runtime conformance |

## Closed Mismatches With Proof

1. `AUD-DOCKER-01`
   - proof: Docker guide is updated to runtime service contract and API checks.
2. `AUD-DOCKER-02`
   - proof: deployment spec baseline compose template now matches runtime service.
3. `AUD-DOCKER-03`
   - proof: top-level TODO Docker gate now verifies runtime APIs.
4. `AUD-DOCKER-04`
   - proof command: `docker compose config` (pass).
   - proof command: `docker compose up -d --build` (pass; service healthy).
   - proof command: `curl -fsS http://127.0.0.1:8080/api/healthz` (pass).
   - proof command: `curl -fsS http://127.0.0.1:8080/api/readyz` (pass).
   - proof command: `docker compose down` (pass).
5. `AUD-DOCKER-05`
   - proof: reference snapshot and conformance rows now describe app-runtime
     startup verification rather than docs-launch behavior.

## Deferred Mismatches

No Docker-scope mismatches deferred in this pass.
