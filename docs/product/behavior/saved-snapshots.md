# Saved Snapshot Behavior

## Canonical Terms

- `Live note` means the mutable current note state at `/{ref}`.
- `Saved snapshot` means one immutable saved note state with its own opaque root-path URL.
- Public copy and labels should prefer `saved snapshot` over `revision`.

## Snapshot Lifecycle

- Creating a note writes the live note and saved snapshot `1`.
- Every successful note update writes one new saved snapshot after the live note is updated.
- Saved snapshots preserve the post-save state, not the pre-save state.
- Saved snapshots are immutable once written.
- Saved snapshots are never soft-deleted when the live note is deleted.

## Snapshot Fields

- Each saved snapshot stores `id`, `record_id`, `snapshot_number`, `alias`, `title`, `summary`, `body`, `is_private`, and `created_at`.
- `snapshot_number` increments from `1` per note.
- `alias`, `title`, and `summary` are copied from the saved note state so history cards never need to derive them from later live-note changes.

## Access Model

- `/{ref}` always resolves to the live note.
- `/{snapshot_id}` resolves to one immutable saved snapshot.
- `/{ref}/history` shows the live note plus paginated saved snapshots.
- History JSON is admin-only and returns saved snapshots, not the mutable live note.

## Visibility

- Guests may open only public live notes and public saved snapshots.
- Admins may open all live notes and all saved snapshots.
- Snapshot visibility follows the stored `is_private` value captured at save time.
