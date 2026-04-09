# History and Favorites Behavior

## Favorites

- Favorite state is shared resource state across notes and media.
- `PUT /resources/favorites/order` is admin-only.
- Favorite ordering uses one persistent sequence across mixed resources.
- Home and dashboard favorites may therefore contain both notes and media.

## History Pagination

- HTML history pages and `GET /resources/{id}/history` share the same pager contract from [../navigation/paging/README.md](../navigation/paging/README.md).
- Page one keeps the live resource visible above the paginated saved snapshots.
- Snapshot cards use `Latest saved snapshot` for the newest visible snapshot on the first page.

## History Separation

- History UI must clearly separate the mutable live resource from immutable saved snapshots.
- Live resource metadata should reflect current state.
- Saved snapshot metadata must reflect the stored historical state, including file metadata for media.
