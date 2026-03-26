# Controls and States Contract

## Action Style

- Standard actions render as text-only controls.
- Filled buttons are not the default action language.
- Delete may use color emphasis, but still reads as text-first.

## Compact Navigation Controls

- Narrow-screen menu and close controls stay icon-only.
- Their background and border remain quiet relative to the page.
- ARIA labels remain required.

## State Treatments

- Hover, focus, and active states are visible on every control.
- Disabled navigation stays readable and muted.
- Save status messaging is explicit for saving, saved, and failed states.

## Form Rules

- `Public` remains the only visibility control.
- No helper sentence appears next to the checkbox.
- The editor surface must not expose a Markdown formatting toolbar in this pass.
