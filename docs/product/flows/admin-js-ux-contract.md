# Admin JavaScript UX Contract

This document defines richer `/admin` JavaScript behavior layered on server-rendered and HTMX contracts.

## Progressive Enhancement Boundary

- `/admin` remains usable without JavaScript through normal form submissions.
- JavaScript enhancements MUST not change server validation or authorization rules.
- HTMX remains the transport for fragment swaps; JavaScript orchestrates when requests are sent.

## Editor State Contract

- Client state tracks:
  - active `slug`
  - `last_known_revision`
  - dirty flag (`has_unsaved_changes`)
  - autosave timer handle
- Dirty state becomes `true` on user edits to `title`, `body`, or `private`.
- Dirty state resets only after successful save/open/create/rename that returns a fresh revision.

## Autosave Contract

- Trigger autosave after `2s` debounce from the latest edit.
- Trigger autosave immediately on blur for title/body/private fields when dirty.
- Trigger autosave attempt during `beforeunload` when dirty.
- Autosave uses `POST /admin/save` and follows the same conflict contract as manual saves.
- Autosave requests are skipped when no unsaved changes exist.

## Unsaved-Changes Guard Contract

- Guard navigation-triggering actions (`open`, `create`, `rename`, `delete`, leaving `/admin`) when dirty.
- Show confirmation before discarding unsaved edits.
- If a save is in flight during navigation, guard waits for completion or explicit discard.

## Keyboard Shortcut Contract

Primary modifier:
- `Ctrl` on Windows/Linux
- `Cmd` on macOS

| Shortcut | Required behavior |
| --- | --- |
| `Primary+S` | Save current document immediately and prevent browser default save dialog |
| `Primary+N` | Open new-article flow and focus first create input |
| `Primary+Shift+P` | Request fresh server-side preview (`POST /admin/preview`) |
| `Primary+K` | Focus quick-open/filter input for article navigation |

## Banner and Accessibility Contract

- `#admin-status-banner` uses `aria-live="polite"` for normal save and autosave outcomes.
- `#admin-conflict-banner` uses `aria-live="assertive"` and `role="alert"` for conflicts.
- Keyboard shortcuts and autosave updates MUST preserve editor focus/caret where possible.

## Cross-References

- HTMX transport and fragment shapes: [admin-htmx-contracts.md](admin-htmx-contracts.md)
- Conflict semantics: [admin-conflict-warning.md](admin-conflict-warning.md)
- Base page shell IDs: [page-contracts.md](page-contracts.md)
