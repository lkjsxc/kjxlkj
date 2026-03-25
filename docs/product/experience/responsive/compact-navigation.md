# Compact Navigation Contract

## Trigger Behavior

- At `900px` width and below, the rail is hidden by default.
- A top-right ghost icon button opens the drawer.
- The visible control is icon-only.
- The accessible name remains explicit.

## Close Behavior

- The drawer includes a subtle icon-only close button.
- Backdrop click and `Esc` still close the drawer.
- The close control must feel quieter than the drawer content.

## Anti-Regression Rules

- No bright white button boxes on dark compact chrome.
- No visible `Menu` or `Close` text labels in the narrow-screen shell.
- Compact controls may not overlap or clip page content.
