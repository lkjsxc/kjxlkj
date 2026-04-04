# Geometry and Radius Contract

## Rectangular Radius Scale

- Rectangular controls and panels use a tight radius family rather than soft rounded corners.
- Canonical rectangular radii are `2px`, `3px`, and `4px`.
- `2px` is for the smallest compact controls and inline chrome.
- `3px` is for standard buttons, inputs, rows, and rail cards.
- `4px` is the upper bound for larger rectangular shells such as note cards, auth cards, and preview panels.
- Rectangular UI may not return to the older `12px` through `24px` radius range.

## Shape Rules

- Buttons, text inputs, textareas, selects, search cards, note cards, settings rows, rail cards, and preview shells all use the same tight family.
- Mobile and compact layouts keep the same radius family rather than inflating corners.
- Tight corners should read precise, dense, and utilitarian.
- Rounded pills remain allowed only for semantic pills such as visibility state.
- Circular icon marks remain circular.

## Verification

- Browser verification should fail if major rectangular surfaces drift back to soft rounded-card styling.
- Dark flat surfaces plus tight corners are canonical together; changing only one of those traits is incomplete.
