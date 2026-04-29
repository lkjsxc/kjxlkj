# Browser Failure Triage Contract

## Investigate First

- Button text contrast regressions.
- Drawer trigger drawing too much attention.
- Light surfaces reappearing in the dark shell.
- Compact layout clipping or overlap.
- Old UI artifacts such as `RECENT`, rail search, or mode-switch text returning.
- Live viewer tests must prove media playback, not only WebRTC `ontrack`.
- Live viewer video must have `readyState`, `videoWidth`, advancing `currentTime`, and inbound RTP stats.

## Recovery Steps

1. Reproduce with the same compose command and viewport.
2. Inspect computed colors for the failed control or surface.
3. Confirm the docs canon still points to the intended dark tokens and compact-control rules.
4. Fix the implementation before accepting screenshot drift.
