# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

## Relevant Documents

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

`/docs/todo/` is the execution contract for rebuilding derived artifacts from All in Docs canon.

## Start Gate

- [x] read [/README.md](/README.md)
- [x] read [/docs/README.md](/docs/README.md)
- [x] read [/docs/policy/README.md](/docs/policy/README.md)
- [x] read [/docs/spec/README.md](/docs/spec/README.md)
- [x] read [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [x] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Immediate Reconstruction Pack

- [x] restore runtime bootstrap and supervision path
- [x] restore reachable HTTP and WebSocket services
- [x] restore typed frontend shell and editor flow in TypeScript
- [x] restore RBAC/auth/session/CSRF enforcement
- [x] restore automation and librarian flow with deterministic review/apply behavior
- [x] restore deterministic regression pack (`REG-IMP-*`, `REG-USR-*`, `REG-UX-*`)
- [x] satisfy type verification gates (`TYPE-01..03`)

## Recursive Wave Program

- [x] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [x] execute stages and waves in order only
- [x] attach deterministic evidence per completed wave
- [x] synchronize reference ledgers with every status change

## Docker Artifact Gate

- [x] root `Dockerfile`, `docker-compose.yml`, and `.dockerignore` exist
- [x] root Docker artifacts match [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) and [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- [x] `docker compose config` passes from repository root
- [x] app runtime smoke passes (`docker compose up -d --build`, `curl -fsS http://127.0.0.1:8080/api/healthz`, `curl -fsS http://127.0.0.1:8080/api/readyz`, `docker compose down`)
- [x] proof recorded in [/docs/log/audits/2026-02-13-docker-artifact-contract-sync.md](/docs/log/audits/2026-02-13-docker-artifact-contract-sync.md)

## Completion Gate

- [x] all stage/wave checklists complete in order
- [x] Docker Artifact Gate is complete
- [x] no high-severity open limitations
- [x] acceptance and typed gates pass
- [x] release gate is satisfied
