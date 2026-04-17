# Compact Timeline Layout

## Narrow Layout Rule

- Narrow live resource pages keep `Prev`, `History`, and `Next` in one compact row when space allows.
- If the viewport cannot safely fit three equal cards, the row may wrap to multiple lines while preserving card widths within each visible line.
- Compact drawers do not move the live-resource timeline back into the rail.

## Stability Rules

- Timeline cards keep stable footprints even when disabled.
- `Prev`, `History`, and `Next` cards keep matched widths and matched minimum heights.
- Compact timeline navigation must stay readable without page-level horizontal overflow.
- Main-pane pager layout is controlled separately by [../paging/compact-pager-layout.md](../paging/compact-pager-layout.md).
