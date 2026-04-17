# Timeline Semantics and Order

## Meaning

- `Prev` means the nearest older accessible live resource.
- `Next` means the nearest newer accessible live resource.
- Timeline relationships are computed from live-resource `created_at`.
- Guest timeline navigation skips private resources.
- Admin timeline navigation includes private resources.

## Shared Surface Rules

- Live resource pages expose the timeline in the main-pane `Prev` / `History` / `Next` row.
- The row always renders exactly two timeline target slots plus one history slot.
- Missing targets use muted non-interactive cards with explanatory copy.
- Timeline labels stay inside the shared cards.

## Slot Order

- `Prev` is always the first slot.
- `History` is always the middle slot.
- `Next` is always the third slot.
- Fixed slot positions do not change when one side is unavailable.
- Timeline rules are separate from search/history paging rules.
