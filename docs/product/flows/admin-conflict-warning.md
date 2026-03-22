# Admin Conflict Warning Contract

This document is the canonical contract for save conflicts in `/admin`.

## Concurrency Model

- Saves are optimistic and carry `last_known_revision` from the editor.
- A conflict exists when submitted `last_known_revision` differs from the persisted revision at save time.
- Conflict detection happens server-side for all save requests, including autosave.

## Resolution Policy

- Resolution strategy is **last-write-wins**.
- The incoming save is persisted even when stale.
- The response must explicitly disclose that another revision was overwritten.

## Warning Banner Contract

- Conflict response makes `#admin-conflict-banner` visible.
- Banner attributes:
  - `id="admin-conflict-banner"`
  - `role="alert"`
  - `aria-live="assertive"`
  - `data-conflict="true"`
- Banner content requirements:
  - clear warning that a stale editor snapshot was saved
  - previously persisted revision identifier or time (if available)
  - newly persisted revision identifier or time
  - action links or buttons: reload latest, continue editing

## Interaction Rules

- Conflict banner appears for manual saves and autosaves.
- `#admin-status-banner` still reports save success so last-write-wins behavior is visible.
- Banner remains visible until:
  - user dismisses it, or
  - next non-conflicting save clears it.

## Operational Visibility

- Conflict responses SHOULD emit deterministic telemetry markers to support test assertions.
- Operations verification uses this contract as the source of truth.
