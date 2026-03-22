# UI Contract Verification Checklist

## Scope

Use this checklist to verify UI contracts defined in product and architecture docs before and after implementation changes.

## Server-Rendered Page Checks

1. `GET /` follows setup-first redirect rules and renders full home page after setup.
2. `GET /setup` renders full setup form only while setup is pending.
3. `GET /login` renders full login page only after setup completion.
4. `GET /admin` renders full admin shell only for authenticated admins.
5. Each page exposes stable root IDs from [../../product/flows/page-contracts.md](../../product/flows/page-contracts.md).

## HTMX Admin Checks

1. `POST /admin/preview` updates `#admin-preview-pane` with server-rendered sanitized HTML.
2. `GET /admin/open/{slug}` returns editor and preview fragments with stable IDs.
3. `POST /admin/save` returns status fragment and updated revision token.
4. Create/rename/delete/toggle flows update list/editor fragments deterministically.
5. Auth failures on HTMX requests return deterministic redirect signaling.

## JavaScript UX Checks

1. Autosave fires after 2 seconds of idle editing.
2. Autosave fires immediately on blur when dirty.
3. Before unload triggers save attempt or unload warning when dirty.
4. Unsaved-change guards protect open/create/rename/delete and route-exit actions.
5. Shortcut bindings are enforced:
   - `Ctrl/Cmd+S`
   - `Ctrl/Cmd+N`
   - `Ctrl/Cmd+Shift+P`
   - `Ctrl/Cmd+K`

## Conflict Warning Checks

1. Stale save is persisted (last-write-wins).
2. Conflict response makes `#admin-conflict-banner` visible.
3. Conflict banner includes overwritten and current revision context.
4. `#admin-status-banner` still reports save completion.

## Required Validation Commands

```bash
cargo run --bin kjxlkj -- docs validate-topology
cargo run --bin kjxlkj -- quality check-lines
```
