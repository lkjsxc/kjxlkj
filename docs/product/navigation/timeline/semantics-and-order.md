# Timeline Semantics and Order

## Meaning

- `Prev` means the nearest older accessible live resource.
- `Next` means the nearest newer accessible live resource.
- Timeline relationships are computed from live-resource `created_at`.
- Guest timeline navigation skips private resources.
- Admin timeline navigation includes private resources.

## Shared Surface Rules

- Note and history rails expose the timeline.
- The rail always renders exactly two timeline slots.
- Missing targets use muted non-interactive cards with explanatory copy.
- Timeline labels stay outside the preview cards.

## Slot Order

- `Prev` is always the first slot.
- `Next` is always the second slot.
- Fixed slot positions do not change when one side is unavailable.
- Timeline rules are separate from search/history paging rules.
