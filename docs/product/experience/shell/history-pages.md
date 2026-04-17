# History Pages Contract

## Layout

- History index and saved-snapshot pages reuse the note shell.
- The history index main pane keeps the live resource row above the saved-snapshot list.
- The live resource page itself owns the compact top-row navigation; the history index remains the dedicated saved-snapshot browsing surface.
- The history body stays in the main pane.
- The history index is the only full saved-snapshot browsing surface.
- Visible rail section headings remain absent here too.
- History index paging uses the shared pager contract from [../../navigation/paging/README.md](../../navigation/paging/README.md).

## Access Rules

- Guests may read only public saved snapshots.
- Admins may read all saved snapshots.
- Guests cannot open the history index list.
- Guests do not see a `History` card on live resource pages.
- Saved-snapshot pages never expose raw live-note IDs as normal page chrome.
- Saved-snapshot pages use their own opaque root-path URL at `/{id}`.
- Guest saved-snapshot pages do not show a `Back to history` link.

## Index Content

- The first card links to the live note and is labeled `Live note`.
- The `Live note` card shows `Created` and `Updated` inside the shared card metadata.
- The remaining cards list one paginated page of visible saved snapshots in `snapshot_number DESC` order.
- The first saved-snapshot card on the first page is labeled `Latest saved snapshot`.
- Remaining saved-snapshot cards use `Saved snapshot N`.
- Each saved-snapshot card shows saved-at time, visibility state, and summary preview.
- Saved-snapshot cards prefer a changed excerpt that highlights text added or removed from the previous snapshot.
- Each saved-snapshot card links directly to its root-path saved-snapshot page.
- The live-note card remains visible on every page instead of becoming part of the paginated slice.
- History paging is URL-shareable through `cursor`, `direction`, and `limit`.
- History paging works with normal browser back/forward without forcing a full reload.

## Rail Limits

- History pages may keep one history affordance, but only admin live resource pages expose the compact top-row history control.
- The rail never lists individual saved snapshots.
