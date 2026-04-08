# Timeline Semantics and Order

## Meaning

- `Prev` means the nearest older accessible live note.
- `Next` means the nearest newer accessible live note.
- Timeline relationships are computed from live-note `created_at`.
- Guest timeline navigation skips private notes.
- Admin timeline navigation includes private notes.

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
