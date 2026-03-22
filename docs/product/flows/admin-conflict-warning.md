# Inline Edit Conflict Warning Contract

This document is the canonical contract for conflicts in inline article editing on
`/article/{slug}`.

## Concurrency Model

- Saves are optimistic and carry `last_known_revision` from the inline editor form.
- A conflict exists when submitted `last_known_revision` differs from the persisted revision at save time.
- Conflict detection happens server-side for `POST /article/{slug}/edit` writes, including
  autosave and non-JS form submission fallback.

## Resolution Policy

- Resolution strategy is **last-write-wins**.
- The incoming save is persisted even when stale.
- The response must explicitly disclose that another revision was overwritten.

## Warning Banner Contract

- Conflict response returns the inline editor fragment and surfaces conflict details in
  `#article-edit-status`.
- Status messaging requirements:
  - clear warning that a stale editor snapshot was saved
  - previously persisted revision identifier or time (if available)
  - newly persisted revision identifier or time
  - preserves visible save-success semantics for last-write-wins behavior

## Interaction Rules

- Conflict warning appears for autosaves and explicit form submissions.
- Warning remains visible until:
  - user dismisses it, or
  - next non-conflicting save clears it.

## Operational Visibility

- Conflict responses SHOULD emit deterministic telemetry markers to support test assertions.
- Operations verification uses this contract as the source of truth.
