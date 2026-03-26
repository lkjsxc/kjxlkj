# Time Contract

## Storage

- Record timestamps remain UTC RFC3339 with trailing `Z`.
- Revision timestamps remain UTC RFC3339 with trailing `Z`.

## Presentation

- HTML must expose machine-readable UTC timestamps.
- Browser JS formats visible timestamps into browser-local 24-hour strings.
- Automated browser verification fixes a deterministic timezone for screenshots.
