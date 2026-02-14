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
| `API-AUTH-03` | setup availability status endpoint reflects owner lock state |
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
| `API-DASH-01` | dashboard widget upsert/list behavior (optional extension) |
| `API-AUTO-01` | automation rule CRUD and deterministic validation |
| `API-AUTO-02` | automation run status retrieval and audit linkage |
| `API-AUTO-03` | librarian rule validation for provider mode (`openrouter`, `lmstudio`) |
| `API-AUTO-04` | librarian XML protocol parse/retry/fail semantics (`xml_attrless`) |
| `API-AUTO-05` | JSON prompt-pack manifest/stage loading and prompt-pack hash reporting |
| `API-ATT-01` | 500 MB attachment upload path |
| `API-ATT-02` | >500 MB deterministic reject |
| `API-OPS-01` | admin export/backup launch and job-status retrieval |
| `WS-01` | subscribe and ordered replay for note stream |
| `WS-02` | subscribe and ordered replay for workspace stream |
| `WS-03` | patch conflict (`patch_rejected`) behavior |
| `WS-04` | idempotent retransmit returns same commit identity |
| `WS-05` | reconnect + ack cursor replay without full reload |
| `WS-06` | librarian automation events stream in commit order with replay cursor support |
| `E2E-01` | owner setup + invite admin/editor/viewer + login |
| `E2E-ROOT-01` | site root serves usable setup/login/workspace shell |
| `E2E-02` | concurrent multi-user editing conflict resolution |
| `E2E-03` | command palette create/open/move/tag/run-rule workflow |
| `E2E-04` | graph explorer traversal and return-context behavior |
| `E2E-05` | dashboard widget configuration persistence (optional extension) |
| `E2E-06` | autosave and title edit without manual-save dependency |
| `E2E-07` | independent pane scroll in responsive layout |
| `E2E-08` | 320px width adaptive layout without mobile/desktop fork |
| `E2E-09` | automation trigger, run status, and audit visibility |
| `E2E-10` | session expiry and re-auth without data loss |
| `E2E-11` | setup-locked state renders login-only UI with no setup-like appearance |
| `E2E-12` | small-screen menu toggle collapses/restores navigation to enlarge editor |
| `E2E-13` | note title rename propagates immediately to notes list/related surfaces |
| `E2E-14` | default editor chrome omits inline version/save/delete controls |
| `E2E-15` | librarian run restructures documentation notes with deterministic audit trail |
| `E2E-16` | pre-auth session probe `401` path is non-fatal and action-guided |
| `E2E-17` | editor status rail (`saving/saved/conflict/offline`) and draft integrity remain consistent |
| `E2E-18` | conflict recovery actions (`reload/reapply/copy`) work without draft loss |
| `E2E-19` | compact layout at `320px` preserves collapse/restore and no horizontal overflow |
| `E2E-20` | baseline note-first mode stays usable with optional modules disabled |
| `E2E-21` | keyboard-first and focus-order flows remain deterministic across panel toggles/review flows |
| `E2E-22` | accessible names/roles/status announcements exist for async-critical controls |
| `E2E-23` | desktop layout (`>=1024px`) keeps note list left and editor right |
| `E2E-24` | compact layout (`<1024px`) keeps editor primary and top-left menu reveals note list |
| `PERF-01` | CRUD/search latency under target scale |
| `PERF-02` | sustained WS stream soak with ordering integrity |
| `PERF-03` | librarian batch structuring throughput under bounded token/time budgets |
| `OPS-01` | backup/export job lifecycle and artifact retrieval |
| `OPS-02` | restart recovery with no lost committed events |

## Finding Regression Pack

| ID | Finding | Required Scenario |
|---|---|---|
| `REG-IMP-001` | `IMP-001` | synced/draft split prevents incorrect patch base |
| `REG-IMP-002` | `IMP-002` | duplicate idempotency key replay returns same commit identity |
| `REG-IMP-003` | `IMP-003` | metadata delete returns strict `204` contract |
| `REG-IMP-004` | `IMP-004` | reconnect ack-cursor replay is deterministic |
| `REG-IMP-005` | `IMP-005` | attachment stream continuity enforcement rejects partial/corrupt output |
| `REG-USR-001` | `USR-001` | session `401` is non-fatal pre-auth path |
| `REG-USR-002` | `USR-002` | idempotency key fallback works without `crypto.randomUUID` |
| `REG-USR-003` | `USR-003` | autosave-first markdown editing confidence path |
| `REG-USR-004` | `USR-004` | setup-lock shows login-only view with no setup-like UI |
| `REG-USR-005` | `USR-005` | compact layout collapse/restore expands editor area |
| `REG-USR-006` | `USR-006` | baseline UX remains note-first without mandatory dashboard clutter |
| `REG-USR-007` | `USR-007` | title rename propagates same-cycle across list/navigation |
| `REG-USR-008` | `USR-008` | default editor chrome remains minimal |

## UX Reconstruction Regression Pack

| ID | Requirement Link | Required Scenario |
|---|---|---|
| `REG-UX-001` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-AUTH-01..03` | auth transitions and pre-auth `401` are deterministic and non-fatal |
| `REG-UX-002` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-EDIT-01..07` | editor state model, autosave, idempotency replay, and conflict recovery are deterministic |
| `REG-UX-003` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-LAYOUT-01..04` | responsive behavior at desktop and `320px` preserves editing flow |
| `REG-UX-004` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-NAV-01..02` | note-first baseline with optional modules and keyboard-first command flows |
| `REG-UX-005` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-FEEDBACK-01..02` | save/conflict/offline feedback and actionable error states are present |
| `REG-UX-006` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-A11Y-01..02` | focus order and accessibility semantics are validated |
| `REG-UX-007` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) `UX-LIB-01..02` | librarian review/apply flow preserves user control and draft safety |

## Typed Contract Verification

| ID | Scenario |
|---|---|
| `TYPE-01` | backend compile/type gate passes (`cargo check --workspace`) |
| `TYPE-02` | frontend TypeScript strict type-check passes (`tsc --noEmit`) |
| `TYPE-03` | repository contains no handwritten JavaScript runtime source (generated `dist/*.js` allowed) |

## Determinism Rules

- use bounded timeouts and explicit diagnostics
- avoid unbounded sleeps
- capture request IDs and WS sequence evidence on failures
- capture provider kind/model/prompt-hash evidence for librarian runs

## Related

- UX requirements: [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- Findings map: [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md)
- Performance targets: [performance.md](performance.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
