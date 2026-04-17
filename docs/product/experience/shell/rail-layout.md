# Rail Layout Contract

## Global Rail

- The rail is present on every HTML page.
- Desktop keeps the rail as a fixed left column.
- Rail width stays compact and must not be used as the primary full result list.
- Rail scroll position is UI state and survives same-origin partial shell transitions.

## Rail Sections

- Brand and session mode.
- Admin-only create actions near the top of the rail.
- Primary navigation.
- One GitHub section linking to `https://github.com/lkjsxc/kjxlkj`.
- Mode-specific actions.

## Live-Resource Exception

- Live note pages and live media pages move live-resource context, alias, timeline navigation, and the history affordance into the main pane.
- History pages may still keep resource-specific rail context when it improves scannability.

## Create Actions

- Admin rails place `New note` first.
- The create section appears above `Open GitHub`.

## Metadata Rules

- Created and updated values render as browser-local 24-hour time.
- Live-resource cards keep Created and Updated inside shared card metadata.
- Raw IDs are not shown in normal rail metadata.
- Titles clamp to one line and summaries clamp to two lines.

## Timeline Rules

- The timeline always renders exactly two cards.
- Timeline cards may point to notes or media.
- Missing targets render as muted non-interactive cards with explanatory copy.
