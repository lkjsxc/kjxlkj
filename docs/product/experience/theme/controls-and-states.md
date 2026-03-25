# Controls and States Contract

## Button Variants

- Primary buttons use a filled treatment with guaranteed readable text.
- Secondary buttons use a dark raised surface with explicit text color.
- Ghost buttons are reserved for low-emphasis controls such as compact navigation.
- Destructive buttons remain legible without overpowering the rest of the rail.

## Compact Navigation Controls

- The narrow-screen menu trigger is a ghost icon button in the top-right corner.
- The drawer close control is also a ghost icon button.
- Visible text labels are not shown on those controls.
- Accessible names are still required through ARIA or screen-reader-only text.

## State Treatments

- Hover, focus, and active states are visible on every control.
- Disabled or unavailable navigation states use muted text, not hidden layout shifts.
- Save status messaging is explicit for idle, saving, saved, and failed states.

## Checkbox and Form Rules

- The `Public` checkbox remains the canonical visibility control.
- Inputs, editors, and toolbar controls must inherit dark-surface styling.
- No form control may rely on user-agent default colors in dark mode.
