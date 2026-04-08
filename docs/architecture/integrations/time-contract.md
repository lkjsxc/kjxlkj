# Time Contract

## Storage

- Record timestamps remain UTC RFC3339 with trailing `Z`.
- Revision timestamps remain UTC RFC3339 with trailing `Z`.

## Presentation

- HTML must expose machine-readable UTC timestamps.
- Browser JS formats visible timestamps into browser-local `YYYY-MM-DD HH:mm` strings.
- Client-visible timestamps must not leave a trailing `UTC` marker in normal rendered text.
- Client-driven HTML fragment swaps must re-run the same local-time formatting before replacement becomes visible.
- Automated browser verification fixes a deterministic timezone for screenshots.
