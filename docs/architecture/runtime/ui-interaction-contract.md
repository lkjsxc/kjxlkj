# UI Interaction Runtime Contract

This runtime contract binds page rendering, HTMX fragments, and JavaScript orchestration.

## Interaction Tiers

1. Server-rendered page tier for `/`, `/setup`, `/login`, `/search`, `/admin`, `/admin/settings`, and `/admin/trash`.
2. HTMX tier for admin open/preview/save/create/rename/delete/toggle/settings/trash interactions.
3. JavaScript tier for autosave, unsaved-change guards, keyboard shortcuts, and responsive menu toggle behavior.

## Runtime Invariants

- Server-side HTML is canonical for all page and preview output.
- HTMX responses target stable DOM IDs defined in product contracts.
- JavaScript enhancements are additive and must degrade to server-rendered behavior.
- Auth and setup guards execute before any tier-specific handler logic.
- Shared shell IDs remain stable across page surfaces.

## Preview Rendering Invariant

- Markdown preview is rendered on the server and delivered as HTMX fragment swaps.
- Browser-side markdown rendering is non-authoritative and must not diverge from server output.
- Sanitization rules for article rendering also apply to preview rendering.

## Conflict Handling Invariant

- `POST /admin/save` accepts `last_known_revision`.
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
