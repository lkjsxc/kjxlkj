# Controls and States Contract

## Action Style

- Standard actions render as text-first controls.
- Filled buttons are not the default action language.
- Delete may use color emphasis, but still reads as text-first.

## Compact Navigation Controls

- Narrow layouts expose one quiet top-right menu toggle.
- The drawer may use a backdrop but no loud close chrome.
- Rail interactions must remain keyboard accessible.
- Search paging controls should make backward and forward movement obvious.

## State Treatments

- Hover, focus, and active states are visible on every control.
- Disabled navigation stays readable and muted.
- Disabled timeline cards keep the same size as active cards.
- Save failure is explicit.
- Save success is silent.

## Form Rules

- `Public` remains the only visibility control.
- No helper sentence appears next to the checkbox.
- The rail must not expose a search box.
- The editor must not expose a visible mode switch.
- Toolbar overflow resolves by wrapping, not by a separate scrollbar strip.
- Login page shows the brand only above the sign-in form and button.
- Editor metadata controls should share one card language rather than mixing unrelated field styles.
- Search query state may be displayed in a read-only card beside sort rather than only inside the input.
