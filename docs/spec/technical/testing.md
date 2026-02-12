# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction.

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | local invariants | deterministic unit/property tests |
| `T1` | cross-module behavior | integration tests (HTTP/WS/DB/service) |
| `T2` | user-like runtime proof | E2E browser + API/WS assertions |

## Mandatory Acceptance Pack

| ID | Scenario |
|---|---|
| `API-AUTH-01` | first-run owner registration lockout behavior |
| `API-AUTH-02` | session cookie issuance and revocation |
| `API-USER-01` | user create/list/role update constraints |
| `API-WSPACE-01` | workspace create/update/delete and ownership invariants |
| `API-WSPACE-02` | workspace membership upsert and revoke semantics |
| `API-PROJ-01` | project create/update/delete with workspace scoping |
| `API-NOTE-01` | create + fetch note projection with workspace context |
| `API-NOTE-02` | stale version conflict (`409`) and deterministic payload |
| `API-NOTE-03` | title-only update with optimistic versioning |
| `API-NOTE-04` | note soft-delete excluded from default list/search |
| `API-REC-01` | typed metadata upsert/delete with `204` delete response |
| `API-SEARCH-01` | wiki link and backlink search correctness |
| `API-SEARCH-02` | full-text search over title/body/settings/media metadata |
| `API-VIEW-01` | saved view create/update/delete lifecycle |
| `API-DASH-01` | dashboard widget upsert/list behavior |
| `API-AUTO-01` | automation rule CRUD and deterministic validation |
| `API-AUTO-02` | automation run status retrieval and audit linkage |
| `API-ATT-01` | 500 MB attachment upload path |
| `API-ATT-02` | >500 MB deterministic reject |
| `WS-01` | subscribe and ordered replay for note stream |
| `WS-02` | subscribe and ordered replay for workspace stream |
| `WS-03` | patch conflict (`patch_rejected`) behavior |
| `WS-04` | idempotent retransmit returns same commit identity |
| `WS-05` | reconnect + ack cursor replay without full reload |
| `E2E-01` | owner setup + invite admin/editor/viewer + login |
| `E2E-02` | concurrent multi-user editing conflict resolution |
| `E2E-03` | command palette create/open/move/tag/run-rule workflow |
| `E2E-04` | graph explorer traversal and return-context behavior |
| `E2E-05` | dashboard widget configuration persistence |
| `E2E-06` | autosave and title edit without manual-save dependency |
| `E2E-07` | independent pane scroll in responsive layout |
| `E2E-08` | 320px width adaptive layout without mobile/desktop fork |
| `E2E-09` | automation trigger, run status, and audit visibility |
| `E2E-10` | session expiry and re-auth without data loss |
| `PERF-01` | CRUD/search latency under target scale |
| `PERF-02` | sustained WS stream soak with ordering integrity |
| `OPS-01` | backup/export job lifecycle and artifact retrieval |
| `OPS-02` | restart recovery with no lost committed events |

## Determinism Rules

- use bounded timeouts and explicit diagnostics
- avoid unbounded sleeps
- capture request IDs and WS sequence evidence on failures

## Related

- Performance targets: [performance.md](performance.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
