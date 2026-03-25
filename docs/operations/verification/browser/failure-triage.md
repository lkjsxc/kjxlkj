# Browser Failure Triage Contract

## Investigate First

- Button text contrast regressions.
- Drawer trigger or close control drawing too much attention.
- Light surfaces reappearing in the dark shell.
- Compact layout clipping or overlap.

## Recovery Steps

1. Reproduce with the same compose command and viewport.
2. Inspect computed colors for the failed control or surface.
3. Confirm the docs canon still points to the intended dark tokens and compact-control rules.
4. Fix the implementation before accepting screenshot drift.
