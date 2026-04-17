# View Counting Behavior

## Counted Opens

- Only successful guest opens of live resource root pages count as views.
- Counted routes are `GET /{ref}` when `ref` resolves to one live note or one live media resource.
- Counted opens increment both `view_count_total` and the current UTC bucket in `resource_daily_views`.

## Excluded Opens

- Authenticated admin opens of live note pages do not count.
- Authenticated admin opens of live media pages do not count.
- Saved-snapshot root URLs do not count, whether they are opened by guests or admins.
- File delivery routes such as `/{ref}/file` and `/{snapshot_id}/file` do not count.
- `404`, unauthorized, and redirected requests do not count.

## Consumers

- Dashboard totals for `Views total`, `Views 1d`, `Views 7d`, `Views 30d`, and `Views 90d` use counted opens only.
- Popularity windows use counted opens only.
- Counted opens apply equally to notes and media once a resource is public and guest-readable.
