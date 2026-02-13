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
- Finding-mapped reliability regression guards are in place for active `IMP-*`/`USR-*`
    runtime surfaces; attachment continuity remains deferred with attachment APIs.
- Historical implementation and user findings remain mandatory regression targets.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup exists, but full single-service supervision topology and background workers are incomplete | `M2 missing feature` | high | `E2E-01` | continue runtime reconstruction for Stage 02+ workflows |
| `LIM-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API baseline now includes notes/history/rollback/metadata/tags/search/backlinks/views plus automation rules/runs and admin export/backup job routes, but attachments/media and full acceptance breadth remain incomplete | `M2 missing feature` | high | `API-*` | extend REST coverage to remaining canonical routes |
| `LIM-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | websocket note/workspace patch+replay baseline plus automation-run workspace replay events exist, but full canonical protocol breadth and downstream consumers remain incomplete | `M2 missing feature` | high | `WS-01..05` | extend WS coverage to remaining canonical message families |
| `LIM-UI-02` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | focused notes UI shell baseline exists (setup/login split handling, compact-screen toggle, title propagation path, minimal chrome), but full production-depth E2E breadth/accessibility hardening remains incomplete | `M2 missing feature` | high | `E2E-03`, `E2E-06..08`, `E2E-11..14` | harden frontend shell and expand E2E breadth for full UX contract closure |
| `LIM-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | route/domain authorization exists for Stage 01+02 notes and Stage 03 Wave 030 view-mutation surfaces, but full API-surface authorization coverage is incomplete | `M2 missing feature` | high | `API-USER-01`, `API-WSPACE-02` | extend authorization guards to all mutation routes as Stage 03+ endpoints land |
| `LIM-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rule CRUD and run lifecycle/idempotency baseline exist, but full trigger breadth, robust side-effect coverage, and librarian automation execution remain incomplete | `M2 missing feature` | high | `API-AUTO-01`, `API-AUTO-02`, `E2E-09` | expand deterministic rule-run coverage and complete librarian/event integration |
| `LIM-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | provider + action-schema + xml parser/retry + scope/safety report baseline exists, but operation-apply runtime and downstream stream/E2E closure are incomplete | `M2 missing feature` | high | `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` | complete operation apply engine and end-to-end librarian execution surfaces |
| `LIM-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search baseline exists, but ranking depth/perf hardening and acceptance breadth are incomplete | `M2 missing feature` | high | `API-SEARCH-01`, `API-SEARCH-02` | harden search ranking/indexing and broaden acceptance coverage |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-LIB-GUARD-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | provider/scope/safety plus XML parser/retry malformed-boundary guards exist, but operation-apply and WS/E2E regression guard coverage remains incomplete | `M4 verification gap` | high | extend deterministic regression suite to operation-apply and WS/E2E librarian flows |
| `LIM-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | PERF-01/PERF-02 smoke evidence exists, but target-scale envelope and sustained throughput evidence remain incomplete | `M4 verification gap` | medium | execute full target-scale `PERF-01`/`PERF-02` and archive resource telemetry |
| `LIM-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | export/backup lifecycle and restart-recovery baseline evidence exists, but full restore-drill parity evidence remains incomplete | `M4 verification gap` | medium | execute full `OPS-01`/`OPS-02` release-gate restore drills with parity checks |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
