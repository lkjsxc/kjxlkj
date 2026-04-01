# History Pages Contract

## Layout

- History index and revision pages reuse the note shell.
- The rail keeps current-note context, timeline cards, one history card, and actions.
- The history body stays in the main pane.
- The history index is the only full revision-browsing surface.
- Visible rail section headings remain absent here too.
- History index paging uses the same `Previous` and `Next` language as Search.

## Access Rules

- Guests may read only public revisions.
- Admins may read all revisions.
- Revision pages never expose raw note IDs as normal page chrome.

## Index Content

- The first card links to the current note.
- The remaining cards list one paginated page of visible revisions in `revision_number DESC` order.
- Each revision card shows revision label, saved-at time, and visibility state.
- The current-note card remains visible on every page instead of becoming part of the paginated slice.
- History paging is URL-shareable through `cursor`, `direction`, and `limit`.

## Rail Limits

- The rail history section contains exactly one card labeled `All history`.
- The rail never lists individual revisions.
- Revision pages keep the same single-card history affordance.
