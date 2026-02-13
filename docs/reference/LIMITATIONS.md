# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-13)

- Runtime baseline is reconstructed through Stage 02 collaborative notes core.
- Auth/session, user role update, and workspace membership APIs are runtime-reachable.
- Notes CRUD/history/rollback/versioning, metadata/tags/backlinks/search baseline APIs,
  and websocket subscribe/patch/replay/idempotency/conflict flows are runtime-reachable.
- Saved views persistence APIs are runtime-reachable with role-denial checks for viewer mutations.
- Librarian provider adapter baseline is runtime-reachable for `openrouter` and
    `lmstudio` with deterministic timeout/retry/failure classification and run
    metadata persistence.
- Librarian action-schema contract validation and operation-report payload
    persistence are runtime-reachable, with scope/safety rejection guards applied
    before operation-apply stage.
- Librarian `xml_attrless` parser + repair-retry failure semantics are
    runtime-reachable, including malformed-nesting and missing-tag deterministic
    failure codes with parse diagnostics retention.
- Librarian websocket lifecycle/replay baseline is runtime-reachable, including
    typed `automation_event` delivery, stale-cursor deterministic `ack` rejection,
    mixed note+librarian commit-order replay checks, and unknown-event compatibility.
- Librarian control/review UX baseline is runtime-reachable, including
    rule provider/model/strict-mode authoring, manual launch/list/review APIs,
    deterministic review decision validation, and review-event audit linkage.
- Finding-mapped reliability regression guards are in place for active `IMP-*`/`USR-*`
    runtime surfaces; attachment continuity remains deferred with attachment APIs.
- Historical implementation and user findings remain mandatory regression targets.

## Open Follow-on Gaps

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup exists, but full single-service supervision topology and background workers remain follow-on hardening scope | `M2 missing feature` | medium | `E2E-01` | continue runtime reconstruction for extended topology workflows |
| `LIM-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API baseline includes notes/history/rollback/metadata/tags/search/backlinks/views plus automation rules/runs and admin export/backup job routes, while attachments/media and optional breadth remain follow-on scope | `M2 missing feature` | medium | `API-*` | extend REST coverage to remaining optional/extended routes |
| `LIM-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | websocket note/workspace replay baseline plus typed automation-run replay and stale-cursor rejection exist; remaining canonical breadth is follow-on scope | `M2 missing feature` | medium | `WS-01..06` | extend WS coverage to remaining optional message families |
| `LIM-UI-02` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | focused notes and librarian shell baseline exists, but full production-depth accessibility and optional E2E breadth remain follow-on scope | `M2 missing feature` | medium | `E2E-03`, `E2E-06..08`, `E2E-11..14` | harden frontend shell and expand E2E breadth for optional UX depth |
| `LIM-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | route/domain authorization exists across active baseline routes; remaining surface-hardening is follow-on scope | `M2 missing feature` | medium | `API-USER-01`, `API-WSPACE-02` | extend authorization guards as additional mutation routes land |
| `LIM-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rule/run lifecycle and review baseline exist, while broader trigger/action families remain follow-on scope | `M2 missing feature` | medium | `API-AUTO-01`, `API-AUTO-02`, `E2E-09` | expand deterministic rule-run coverage for additional trigger/action breadth |
| `LIM-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | provider + action-schema + xml parser/retry + websocket + review/apply UX baseline exists; full operation-kind apply breadth remains follow-on scope | `M2 missing feature` | medium | `API-AUTO-03`, `API-AUTO-04`, `PERF-03` | extend operation-kind apply engine to remaining action families |
| `LIM-MEDIA-02` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone media-note upload/download APIs remain deferred follow-on scope | `M2 missing feature` | medium | `API-ATT-01`, `API-ATT-02` | implement attachment/media API surface with deterministic size-guard behavior |
| `LIM-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text/backlink search baseline exists, while deeper ranking/perf tuning remains follow-on scope | `M2 missing feature` | medium | `API-SEARCH-01`, `API-SEARCH-02` | harden search ranking/indexing and broaden acceptance coverage |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-LIB-GUARD-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | provider/scope/safety plus XML parser/retry, websocket replay, review/apply, and PERF-03 baseline guards exist; remaining operation-kind breadth is follow-on | `M4 verification gap` | medium | extend deterministic regression suite to full operation-kind apply breadth |
| `LIM-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | PERF-01/PERF-02/PERF-03 baseline evidence exists, but target-scale envelope and sustained throughput telemetry remain incomplete | `M4 verification gap` | medium | execute full target-scale perf suite and archive resource telemetry |
| `LIM-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | export/backup lifecycle and restart-recovery baseline evidence exists, but full restore-drill parity evidence remains incomplete | `M4 verification gap` | medium | execute full `OPS-01`/`OPS-02` release-gate restore drills with parity checks |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
