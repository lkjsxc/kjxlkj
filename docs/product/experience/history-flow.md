# Revision History Experience Contract

## History Entry Points

- Note pages expose history links in the navigation rail.
- `GET /{slug}/history` renders the history index for a note.
- `GET /{slug}/history/{revision_number}` renders a historical snapshot.

## Guest Rules

- Guests can reach history only for currently public notes.
- Guests can see revision metadata only for snapshots whose stored state is public.
- Guests can open only public snapshots.
- Private snapshots return `404`.

## Admin Rules

- Admins can see all revisions for accessible notes.
- Admins can open private or public snapshots.
- Revision pages are read-only historical views.

## History Index Content

- Current version link.
- Revision list ordered by `revision_number` descending.
- Each revision row shows revision number, created timestamp, and visibility state.

## Revision Snapshot Content

- Snapshot title extracted from the historical body.
- Historical visibility state.
- Read-only rendered Markdown body.
- Links back to current note and history index.
