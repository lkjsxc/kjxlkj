# Audio Configuration

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Audio feedback settings.

## Overview

The editor can emit audio feedback (terminal bell) for errors and alerts.

## Terminal Bell

When an error occurs (e.g., motion fails, search not found), the terminal bell character (BEL, 0x07) is sent to the terminal.

| Setting | Default | Description |
|---|---|---|
| `bell` | `true` | Enable terminal bell |

## Visual Bell

As an alternative to the terminal bell, a visual flash can be shown:

| Setting | Default | Description |
|---|---|---|
| `visualbell` | `false` | Flash screen instead of bell |

## Screen Reader Integration

When `accessibility.screen_reader` is enabled, the editor sends appropriate ARIA-like cues through terminal escape sequences for screen reader software.

## Related

- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- UX: [/docs/spec/ux/README.md](/docs/spec/ux/README.md)
