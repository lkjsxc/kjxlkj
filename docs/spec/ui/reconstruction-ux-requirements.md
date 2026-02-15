# Reconstruction UX Requirements

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

Canonical UX requirements for rebuilding `kjxlkj` from an All in Docs baseline.

## Scope

- These requirements are normative and apply to all reconstructed runtime/UI work.
- `IMP-*` and `USR-*` findings are first-class regression drivers, not optional notes.
- A UX requirement is not considered complete until mapped acceptance IDs pass.

## UX Principles

1. Keep authoring confidence higher than feature density.
2. Keep state transitions deterministic and observable.
3. Keep mobile and desktop in one responsive tree.
4. Keep advanced modules optional in the baseline flow.
5. Keep recovery paths explicit for every failure-prone interaction.

## Requirement Matrix

| UX ID | Requirement | Why this exists | Source Findings | Acceptance IDs |
|---|---|---|---|---|
| `UX-AUTH-01` | Pre-auth `GET /api/auth/session` `401` MUST be treated as expected unauthenticated state, not fatal UI error. | prevent false-failure first impression | `USR-001` | `REG-USR-001`, `E2E-10` |
| `UX-AUTH-02` | Setup-locked state (`409`) MUST render login-only UI and MUST hide setup-like visuals. | prevent onboarding confusion | `USR-004` | `REG-USR-004`, `E2E-11` |
| `UX-AUTH-03` | Session expiry MUST redirect to login while preserving unsaved draft recovery path. | avoid silent data loss during auth churn | `USR-001` | `E2E-10`, `E2E-18` |
| `UX-EDIT-01` | Editor MUST maintain separate synced snapshot and local draft state. | prevent incorrect patch base and conflict loops | `IMP-001` | `REG-IMP-001`, `WS-05`, `E2E-17` |
| `UX-EDIT-02` | Autosave MUST be default with bounded debounce and visible status transitions (`saving`, `saved`, `conflict`, `offline`). | maintain editing confidence | `USR-003` | `REG-USR-003`, `E2E-06`, `E2E-17` |
| `UX-EDIT-03` | Idempotency key generation MUST work without `crypto.randomUUID`. | avoid environment-specific save failures | `USR-002` | `REG-USR-002`, `E2E-06` |
| `UX-EDIT-04` | Title rename MUST propagate to lists/navigation in the same interaction cycle. | prevent stale context and misnavigation | `USR-007` | `REG-USR-007`, `E2E-13` |
| `UX-EDIT-05` | Default editor chrome MUST omit inline save/version/delete controls; advanced controls MAY be discoverable via secondary actions. | reduce editing clutter | `USR-008` | `REG-USR-008`, `E2E-14` |
| `UX-EDIT-06` | Conflict states MUST provide explicit user action paths (`reload latest`, `reapply draft`, `copy draft`). | make recovery deterministic under contention | `IMP-004` | `REG-IMP-004`, `WS-03`, `E2E-18` |
| `UX-EDIT-07` | Duplicate idempotency retransmit MUST surface same commit identity instead of duplicate-success ambiguity. | guarantee replay-safe mental model | `IMP-002` | `REG-IMP-002`, `WS-04` |
| `UX-LAYOUT-01` | One responsive component tree MUST support desktop and mobile without mode forks. | avoid divergence and inconsistent behavior | `USR-005` | `E2E-07`, `E2E-08` |
| `UX-LAYOUT-02` | On constrained widths, menu toggle MUST collapse/restore navigation and preserve editor focus continuity. | maximize editing area on small screens | `USR-005` | `REG-USR-005`, `E2E-12`, `E2E-19` |
| `UX-LAYOUT-03` | Navigation and editor panes MUST retain independent vertical scrolling where dual-pane layout is active. | preserve context while browsing and editing | `USR-003`, `USR-005` | `E2E-07` |
| `UX-LAYOUT-04` | At `320px` width, UI MUST avoid horizontal scrolling for core flows and keep actionable controls reachable. | enforce real compact-device usability | `USR-005` | `E2E-08`, `E2E-19` |
| `UX-LAYOUT-05` | At `>= 1024px`, note list MUST stay on the left and editor MUST stay on the right in one split layout. | preserve predictable desktop authoring orientation | `USR-005` | `E2E-07`, `REG-UX-003` |
| `UX-LAYOUT-06` | At `< 1024px`, editor MUST be primary by default and a top-right menu button MUST reveal note list. | preserve compact-screen editing priority with discoverable navigation | `USR-005` | `E2E-12`, `E2E-19`, `REG-UX-003` |
| `UX-LAYOUT-07` | On constrained widths, selecting a note from the menu MUST close the menu and return focus to editing context. | avoid manual close friction in compact navigation flow | `USR-005` | `E2E-23`, `REG-USR-005` |
| `UX-NAV-01` | Note-first workflow is baseline; dashboard/workspace switcher/project rail remain optional and off by default. | reduce cognitive overhead | `USR-006` | `REG-USR-006`, `E2E-03`, `E2E-20` |
| `UX-NAV-02` | Command palette MUST provide keyboard-first access to core create/open/move/tag/run actions with deterministic feedback. | speed and accessibility for power users | `USR-003` | `E2E-03`, `E2E-21` |
| `UX-FEEDBACK-01` | Save/sync/conflict/offline states MUST be visible but low-noise in primary editing view. | keep trust without panel clutter | `USR-003`, `USR-008` | `E2E-17`, `E2E-20` |
| `UX-FEEDBACK-02` | Each failure path MUST include one explicit next action and one machine-readable error code. | improve recoverability and supportability | `IMP-003`, `IMP-004` | `API-REC-01`, `WS-03`, `E2E-18` |
| `UX-A11Y-01` | Keyboard navigation and focus order MUST remain deterministic when panels open/close. | protect non-pointer workflows | `USR-005` | `E2E-21` |
| `UX-A11Y-02` | Interactive controls MUST expose accessible names/roles and announce async state transitions. | support assistive technologies | `USR-003` | `E2E-22` |
| `UX-LIB-01` | Librarian operation previews MUST support per-operation accept/reject and deterministic audit trace. | prevent opaque automation side effects | `USR-003` | `E2E-15`, `E2E-21` |
| `UX-LIB-02` | Applying librarian changes MUST preserve unresolved local drafts and never silently overwrite user edits. | guard author intent during automation | `IMP-001`, `USR-003` | `E2E-15`, `E2E-17` |

## Required Non-Functional UX Budgets

| Budget ID | Requirement | Validation |
|---|---|---|
| `UX-BUDGET-01` | `95%` of local keystroke-to-paint updates complete within `100ms` in baseline note editor flows. | `PERF-01`, `E2E-17` |
| `UX-BUDGET-02` | Mobile (`320px`) core flows complete without layout break for create, edit, save, rename, and conflict recovery. | `E2E-08`, `E2E-12`, `E2E-19` |
| `UX-BUDGET-03` | No critical path requires optional modules (dashboard/workspace switcher/project rail). | `E2E-03`, `E2E-20` |

## Closure Rule

A UX requirement is considered closed only when all are true:

1. the requirement is mapped to at least one deterministic acceptance ID
2. acceptance evidence is linked in wave checklists and reference ledgers
3. limitation and drift ledgers are synchronized with status

## Related

- Findings map: [findings-traceability.md](findings-traceability.md)
- Testing: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
