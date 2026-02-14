# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

## Relevant Documents

- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)
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
- [/docs/todo/doc-map/README.md](/docs/todo/doc-map/README.md)

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

## Documentation Link Coverage

- [ ] open [/docs/todo/doc-map/README.md](/docs/todo/doc-map/README.md)
- [ ] complete [/docs/todo/doc-map/core-and-guides.md](/docs/todo/doc-map/core-and-guides.md)
- [ ] complete [/docs/todo/doc-map/policy-and-reference.md](/docs/todo/doc-map/policy-and-reference.md)
- [ ] complete [/docs/todo/doc-map/spec-api-architecture.md](/docs/todo/doc-map/spec-api-architecture.md)
- [ ] complete [/docs/todo/doc-map/spec-domain-security-technical-ui.md](/docs/todo/doc-map/spec-domain-security-technical-ui.md)
- [ ] complete [/docs/todo/doc-map/log-and-overview.md](/docs/todo/doc-map/log-and-overview.md)
- [ ] complete [/docs/todo/doc-map/todo-and-waves.md](/docs/todo/doc-map/todo-and-waves.md)
- [ ] verify full documentation-file linkage via [/docs/todo/doc-map/README.md](/docs/todo/doc-map/README.md)

## Reconstruction Pack

- [ ] [Restore runtime bootstrap and supervision path](/docs/spec/architecture/runtime.md)
- [ ] [Restore reachable HTTP service contract](/docs/spec/api/http.md)
- [ ] [Restore reachable WebSocket service contract](/docs/spec/api/websocket.md)
- [ ] [Restore typed frontend shell and editor flow in TypeScript](/docs/spec/ui/web-app.md)
- [ ] [Restore editor interaction and autosave behavior](/docs/spec/ui/editor-flow.md)
- [ ] [Restore responsive split-pane and top-left mobile menu navigation behavior](/docs/spec/ui/layout-and-interaction.md)
- [ ] [Restore RBAC/auth/session/CSRF enforcement](/docs/spec/security/README.md)
- [ ] [Restore automation and librarian review/apply behavior](/docs/spec/technical/librarian-agent.md)
- [ ] [Load every librarian-cycle prompt from JSON prompt pack files](/docs/spec/technical/librarian-prompts/README.md)
- [ ] [Restore deterministic regression pack (`REG-IMP-*`, `REG-USR-*`, `REG-UX-*`)](/docs/spec/technical/testing.md)
- [ ] [Satisfy type verification gates (`TYPE-01..03`)](/docs/spec/technical/type-safety.md)
- [ ] [Match runtime layout to final completion structure](/docs/spec/architecture/final-file-structure.md)

## Recursive Wave Program

- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] [Execute stages and waves in strict order](/docs/policy/WORKFLOW.md)
- [ ] [Attach deterministic evidence per completed wave](/docs/spec/technical/testing.md)
- [ ] [Synchronize reference ledgers with every status change](/docs/reference/CONFORMANCE.md)

## Docker Artifact Gate

- [ ] [Regenerate root `Dockerfile`, `docker-compose.yml`, and `.dockerignore` when reconstruction enters deployment stage](/docs/spec/architecture/deployment.md)
- [ ] [Ensure root Docker artifacts match deployment and Docker guide contracts](/docs/guides/DOCKER.md)
- [ ] [`docker compose config` passes from repository root](/docs/guides/DOCKER.md)
- [ ] [App runtime smoke passes (`/`, `healthz`, `readyz`)](/docs/spec/architecture/deployment.md)
- [ ] [Record Docker gate proof in audit log](/docs/log/audits/README.md)

## Completion Gate

- [ ] [All stage/wave checklists complete in order](/docs/todo/waves/README.md)
- [ ] [Docker Artifact Gate is complete](/docs/spec/architecture/deployment.md)
- [ ] [No high-severity open limitations remain](/docs/reference/LIMITATIONS.md)
- [ ] [Acceptance and typed gates pass](/docs/spec/technical/testing.md)
- [ ] [Release gate is satisfied](/docs/reference/RELEASE.md)
