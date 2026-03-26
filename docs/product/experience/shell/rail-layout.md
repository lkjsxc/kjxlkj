# Rail Layout Contract

## Global Rail

- The rail is present on every HTML page.
- Desktop keeps the rail as a fixed left column.
- Rail width stays compact and must not be used as the primary full result list.
- Scrollable rails must keep metadata readable without line wrapping collisions.

## Rail Sections

- Brand and session mode.
- Scope search.
- Recent note shortcuts.
- Current note context.
- Created and updated metadata.
- `Prev` and `Next` navigation cards with explicit relationship labels.
- Revision history links.
- Mode-specific text actions.

## Metadata Rules

- Created and updated values render as browser-local 24-hour time.
- Timestamp layout must stack or grid safely; it may not wrap awkward fragments.
- Raw note IDs are not shown in normal rail metadata.

## Action Treatment

- Dashboard, Home, New note, Logout, and Delete use text-style controls.
- No menu trigger, close button, or drawer backdrop appears in this phase.
