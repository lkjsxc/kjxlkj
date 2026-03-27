# Rail Layout Contract

## Global Rail

- The rail is present on every HTML page.
- Desktop keeps the rail as a fixed left column.
- Rail width stays compact and must not be used as the primary full result list.
- Scrollable rails must keep metadata readable without line wrapping collisions.

## Rail Sections

- Brand and session mode.
- Admin-only `New note` action near the top of the rail.
- Primary navigation.
- Scope summary.
- Current note context.
- Created and updated metadata.
- `Prev` and `Next` navigation cards with explicit relationship labels.
- Revision history links.
- Mode-specific actions.

## Metadata Rules

- Created and updated values render as browser-local 24-hour time.
- Timestamp layout must stack or grid safely.
- Raw note IDs are not shown in normal rail metadata.

## Action Treatment

- Search, Dashboard, Home, New note, Logout, and Delete use restrained controls.
- `New note` is the first admin action in the rail.
- Narrow screens add one quiet top-right menu toggle plus a backdrop.
