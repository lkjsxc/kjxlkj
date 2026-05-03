# Saved Snapshot Behavior

## Canonical Terms

- `Live resource` means the mutable current note or media state at `/{ref}`.
- `Saved snapshot` means one immutable saved state with its own opaque root-path URL.

## Snapshot Lifecycle

- Creating a resource writes the live resource and saved snapshot `1`.
- Every successful metadata update writes one new saved snapshot after the live resource updates.
- Saved snapshots preserve post-save state, not pre-save state.
- Saved snapshots are immutable once written.
- Media uploads create new live resources instead of replacing binaries on older media resources.

## Snapshot Fields

- Each saved snapshot stores `id`, `resource_id`, `kind`, `snapshot_number`, `alias`, `title`, `summary`, `body`, `visibility`, and `created_at`.
- Media snapshots also store immutable file-object metadata, derivative metadata, and the object references used by `/{snapshot_id}/file`.
- `snapshot_number` increments from `1` per live resource.

## Access Model

- `/{ref}` resolves to the live resource.
- `/{snapshot_id}` resolves to one immutable saved snapshot.
- `/{user}/{ref}/history` requires resource write permission and shows the live resource plus paginated saved snapshots.
- History JSON requires resource write permission and returns saved snapshots, not the mutable live resource.
- Public saved snapshots remain directly readable by guests who know the opaque snapshot URL.
- Guest saved-snapshot pages do not expose a link back to the history list.
