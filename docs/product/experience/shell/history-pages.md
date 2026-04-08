# History Pages Contract

## Layout

- History index and saved-snapshot pages reuse the note shell.
- The rail keeps live-note context, timeline cards, one history card, the GitHub control, and trailing actions.
- The history body stays in the main pane.
- The history index is the only full saved-snapshot browsing surface.
- Visible rail section headings remain absent here too.
- History index paging uses the shared pager contract from [../../navigation/paging/README.md](../../navigation/paging/README.md).

## Access Rules

- Guests may read only public saved snapshots.
- Admins may read all saved snapshots.
- Saved-snapshot pages never expose raw live-note IDs as normal page chrome.
- Saved-snapshot pages use their own opaque root-path URL at `/{id}`.

## Index Content

- The first card links to the live note and is labeled `Live note`.
- The `Live note` card shows `Created` and `Updated` inside the shared card metadata.
- The remaining cards list one paginated page of visible saved snapshots in `snapshot_number DESC` order.
- The first saved-snapshot card on the first page is labeled `Latest saved snapshot`.
- Remaining saved-snapshot cards use `Saved snapshot N`.
- Each saved-snapshot card shows saved-at time, visibility state, and summary preview.
- Each saved-snapshot card links directly to its root-path saved-snapshot page.
- The live-note card remains visible on every page instead of becoming part of the paginated slice.
- History paging is URL-shareable through `cursor`, `direction`, and `limit`.

## Rail Limits

- The rail history section contains exactly one card labeled `History`.
- The rail places `History` above `Open GitHub` on both guest and admin note/history pages.
- Note/history action ordering follows [actions/section-order.md](actions/section-order.md).
- The rail never lists individual saved snapshots.
- Saved-snapshot pages keep the same single-card history affordance.
- Rail timeline semantics and compact ordering follow [../../navigation/timeline/README.md](../../navigation/timeline/README.md).
