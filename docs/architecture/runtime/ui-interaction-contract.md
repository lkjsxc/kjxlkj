# UI Interaction Runtime Contract

This runtime contract binds page rendering, HTMX fragments, and JavaScript orchestration.

## Interaction Tiers

1. Server-rendered page tier for `/`, `/setup`, `/login`, `/search`, `/admin`,
   `/admin/settings`, `/admin/trash`, `/article/{slug}`, and `/article/{slug}/history`.
2. HTMX tier for admin create/rename/delete/toggle/settings/trash interactions plus inline
   article edit and history restore.
3. JavaScript tier for autosave-first inline editing, unsaved-change guards, keyboard
   shortcuts, and responsive menu toggle behavior.

## Runtime Invariants

- Server-side HTML is canonical for all page output and inline fragment output.
- HTMX responses target stable DOM IDs defined in product contracts.
- JavaScript enhancements are additive and must degrade to server-rendered behavior.
- Auth and setup guards execute before any tier-specific handler logic.
- Shared shell IDs remain stable across page surfaces.
- `/admin` remains a dashboard and never hosts a dedicated editor page.
- Editing occurs inline on `/article/{slug}` only.
- Inline editor field order is `title`, `private`, `body`, `last_known_revision`.
- The `private` toggle is rendered above the `body` field.
- Article pages always expose last-updated metadata and previous/next navigation links.
- Article history routes (`GET /article/{slug}/history`,
  `POST /article/{slug}/history/restore`) are admin-only.

## Conflict Handling Invariant

- `POST /article/{slug}/edit` accepts `last_known_revision`.
- Saves are autosave-first (idle + blur), with plain form submission as non-JS fallback.
- On stale revision mismatch:
  - server persists incoming write (last-write-wins),
  - response includes a visible conflict warning fragment.
- Conflict signaling semantics and banner requirements are canonical in product docs:
  - [../../product/flows/admin-conflict-warning.md](../../product/flows/admin-conflict-warning.md)

## Cross-References

- Page contracts: [../../product/flows/page-contracts.md](../../product/flows/page-contracts.md)
- HTMX fragment contracts: [../../product/flows/admin-htmx-contracts.md](../../product/flows/admin-htmx-contracts.md)
- JS behavior contracts: [../../product/flows/admin-js-ux-contract.md](../../product/flows/admin-js-ux-contract.md)
- Navigation shell contract: [../../product/flows/navigation-shell.md](../../product/flows/navigation-shell.md)
