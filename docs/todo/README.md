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

- [ ] read [/README.md](/README.md)
- [ ] read [/docs/README.md](/docs/README.md)
- [ ] read [/docs/policy/README.md](/docs/policy/README.md)
- [ ] read [/docs/spec/README.md](/docs/spec/README.md)
- [ ] read [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [ ] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Immediate Reconstruction Pack

- [ ] [Restore runtime bootstrap and supervision path](/docs/spec/architecture/runtime.md)
- [ ] [Restore reachable HTTP service contract](/docs/spec/api/http.md)
- [ ] [Restore reachable WebSocket service contract](/docs/spec/api/websocket.md)
- [ ] [Restore typed frontend shell and editor flow in TypeScript](/docs/spec/ui/web-app.md)
- [ ] [Restore editor interaction and autosave behavior](/docs/spec/ui/editor-flow.md)
- [ ] [Restore RBAC/auth/session/CSRF enforcement](/docs/spec/security/README.md)
- [ ] [Restore automation and librarian review/apply behavior](/docs/spec/technical/librarian-agent.md)
- [ ] [Restore deterministic regression pack (`REG-IMP-*`, `REG-USR-*`, `REG-UX-*`)](/docs/spec/technical/testing.md)
- [ ] [Satisfy type verification gates (`TYPE-01..03`)](/docs/spec/technical/type-safety.md)
- [ ] [Match runtime layout to final completion structure](/docs/spec/architecture/final-file-structure.md)

## Recursive Wave Program

- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] [Execute stages and waves in strict order](/docs/policy/WORKFLOW.md)
- [ ] [Attach deterministic evidence per completed wave](/docs/spec/technical/testing.md)
- [ ] [Synchronize reference ledgers with every status change](/docs/reference/CONFORMANCE.md)

## Docker Artifact Gate

- [ ] [Keep root `Dockerfile`, `docker-compose.yml`, and `.dockerignore` present](/docs/spec/architecture/deployment.md)
- [ ] root Docker artifacts match [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) and [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
- [ ] [`docker compose config` passes from repository root](/docs/guides/DOCKER.md)
- [ ] [App runtime smoke passes (`healthz`/`readyz`)](/docs/spec/architecture/deployment.md)
- [ ] proof recorded in [/docs/log/audits/2026-02-13-docker-artifact-contract-sync.md](/docs/log/audits/2026-02-13-docker-artifact-contract-sync.md)

## Completion Gate

- [ ] [All stage/wave checklists complete in order](/docs/todo/waves/README.md)
- [ ] [Docker Artifact Gate is complete](/docs/spec/architecture/deployment.md)
- [ ] [No high-severity open limitations remain](/docs/reference/LIMITATIONS.md)
- [ ] [Acceptance and typed gates pass](/docs/spec/technical/testing.md)
- [ ] [Release gate is satisfied](/docs/reference/RELEASE.md)
