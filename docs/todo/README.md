# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

## Relevant Documents

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
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

- [x] [Restore runtime bootstrap and supervision path](/docs/spec/architecture/runtime.md)
- [x] [Restore reachable HTTP service contract](/docs/spec/api/http.md)
- [x] [Restore reachable WebSocket service contract](/docs/spec/api/websocket.md)
- [x] [Restore typed frontend shell and editor flow in TypeScript](/docs/spec/ui/web-app.md)
- [x] [Restore editor interaction and autosave behavior](/docs/spec/ui/editor-flow.md)
- [x] [Restore RBAC/auth/session/CSRF enforcement](/docs/spec/security/README.md)
- [x] [Restore automation and librarian review/apply behavior](/docs/spec/technical/librarian-agent.md)
- [ ] [Restore deterministic regression pack (`REG-IMP-*`, `REG-USR-*`, `REG-UX-*`)](/docs/spec/technical/testing.md)
- [x] [Satisfy type verification gates (`TYPE-01..03`)](/docs/spec/technical/type-safety.md)
- [x] [Match runtime layout to final completion structure](/docs/spec/architecture/final-file-structure.md)

## Recursive Wave Program

- [x] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [x] [Execute stages and waves in strict order](/docs/policy/WORKFLOW.md)
- [x] [Attach deterministic evidence per completed wave](/docs/spec/technical/testing.md)
- [x] [Synchronize reference ledgers with every status change](/docs/reference/CONFORMANCE.md)

## Docker Artifact Gate

- [x] [Keep root `Dockerfile`, `docker-compose.yml`, and `.dockerignore` present](/docs/spec/architecture/deployment.md)
- [x] root Docker artifacts match [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) and [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- [x] [`docker compose config` passes from repository root](/docs/guides/DOCKER.md)
- [x] [App runtime smoke passes (`healthz`/`readyz`)](/docs/spec/architecture/deployment.md)
- [x] proof recorded in [/docs/log/audits/2026-02-14-runtime-bootstrap-and-docker-gate.md](/docs/log/audits/2026-02-14-runtime-bootstrap-and-docker-gate.md)

## Completion Gate

- [x] [All stage/wave checklists complete in order](/docs/todo/waves/README.md)
- [x] [Docker Artifact Gate is complete](/docs/spec/architecture/deployment.md)
- [ ] [No high-severity open limitations remain](/docs/reference/LIMITATIONS.md)
- [ ] [Acceptance and typed gates pass](/docs/spec/technical/testing.md)
- [ ] [Release gate is satisfied](/docs/reference/RELEASE.md)
