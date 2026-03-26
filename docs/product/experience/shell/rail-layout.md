# Rail Layout Contract

## Desktop Rail

- Note and history pages keep a persistent left rail on desktop.
- Rail width stays compact and must not be used for large note indexes.
- Scrollable rails must keep metadata readable without line wrapping collisions.

## Rail Sections

- Brand and session mode.
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
- Compact navigation trigger and close controls remain icon-only ghost controls.
