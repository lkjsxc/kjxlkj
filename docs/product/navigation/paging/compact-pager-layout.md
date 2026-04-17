# Compact Pager Layout

## Narrow Layout Rule

- Search and history main-pane pagers stay in one left-right row on narrow screens.
- `Prev` stays on the left.
- `Next` stays on the right.
- Compact pagers do not stack into a top-bottom column.

## Stability Rules

- Compact pagers remain readable without page-level horizontal overflow.
- Disabled paging controls keep the same footprint as enabled controls.
- Compact pager layout does not own the live-resource `Prev` / `History` / `Next` strip.
