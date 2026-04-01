# History and Favorites Behavior

## Favorite Ordering

- Favorite order is persistent global note state.
- Favorite lists on Home and Dashboard use `favorite_position ASC`.
- Newly favorited notes append to the end of the favorite list.
- Unfavoriting clears saved favorite position.
- Admins reorder favorites from `/admin`.
- Reordering operates on the full current favorite set, not only the homepage-sized subset.
- The homepage favorites section ends with a `View more notes` card that links to `/search?scope=favorites`.

## Favorite Search Scope

- `scope=favorites` filters `/search` to favorite notes only.
- Empty-query favorite scope defaults to `favorite_position_asc`.
- Non-empty-query favorite scope still defaults to `relevance`.
- Favorite scope keeps pagination and sorting server-side.

## Favorite Reorder API

- `PUT /records/favorites/order` is admin-only.
- Request body contains one `ids` array listing the full favorite set in final order.
- Duplicate IDs, missing favorite IDs, unknown IDs, and non-favorite IDs are invalid.
- Successful reorder normalizes positions into a contiguous ascending sequence.

## History Pagination

- HTML history pages and `GET /records/{id}/history` share the same pagination model.
- Pagination uses `cursor`, `direction`, and `limit`.
- Empty cursor returns the first page.
- Visible revisions remain ordered `revision_number DESC`.
- `direction=prev` still renders revisions in normal on-screen newest-to-oldest order.
- The current note card remains outside the paginated revision slice on HTML history pages.
