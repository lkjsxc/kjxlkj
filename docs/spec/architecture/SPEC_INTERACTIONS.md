# Specification Interactions

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Cross-spec dependency graph and single-source ownership rules.

## Ownership Rule

Each normative rule MUST have one primary source. Other files MAY reference it but MUST NOT redefine it.

## High-Risk Interaction Clusters

### Cluster C1: Communication Core

- Primary: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Primary: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Primary: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- Depends on: [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- Depends on: [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- Depends on: [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md)
- Depends on: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Validation: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

### Cluster C2: Search + Agent

- Primary: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
- Primary: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- Depends on: [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md)
- Depends on: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Validation: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

### Cluster C3: Notes + Realtime UI

- Primary: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Primary: [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- Depends on: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Depends on: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Validation: [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)

## Dependency Matrix

| Target Contract | Must Be Implemented After | Why |
|---|---|---|
| HTTP mutating routes | auth + sessions + csrf | enforce identity and anti-forgery |
| WebSocket replay | events + idempotency | deterministic stream resume |
| Agent YOLO actions | permissions + workspace boundaries | prevent cross-scope writes |
| Hybrid search API | lexical + semantic + degradation semantics | deterministic query behavior |
| Frontend autosave/conflict UX | PATCH conflict + WS replay | consistent merge path |

## Drift Prevention

- Any spec change in cluster `C1` MUST update the communication test matrix.
- Any spec change in cluster `C2` MUST update prompt and degradation assertions.
- Any spec change in cluster `C3` MUST update E2E acceptance mapping.

## Related

- Build ordering: [BUILD_SEQUENCE.md](BUILD_SEQUENCE.md)
- Completion map: [completion-file-map.md](completion-file-map.md)
- TODO trace matrix: [/docs/reference/TODO_TRACE_MATRIX.md](/docs/reference/TODO_TRACE_MATRIX.md)
